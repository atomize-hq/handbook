"""SOURCE-ONLY native adapter. No legacy decoder, corpus import, or launch CLI.
Per-case growth/response limits come from a parent-bound forecast manifest;
final shared protocol methods and complete case coverage await its frozen input.
"""
import contextlib,hashlib,json,math,os,threading,time
from pathlib import Path
import admission_v20 as admission
from guard_v20 import Events
import role_driver_v33 as roles

ROOT=admission.ROOT
READERS=frozenset(('resolve','point','inspect','lookup-bench'))
_LOCK=threading.RLock()
_PENDING={k:{'bytes':0,'files':0} for k in admission.LIMITS}


@contextlib.contextmanager
def reserve(category,size,files):
    if category not in admission.LIMITS or any(type(x) is not int or x<0 for x in (size,files)):raise ValueError('forecast-category-amount')
    with _LOCK:
        request={k:dict(v) for k,v in _PENDING.items()};request[category]['bytes']+=size;request[category]['files']+=files
        admission.admit(admission.snapshot(shared=False),request,admission.limits())
        _PENDING[category]['bytes']+=size;_PENDING[category]['files']+=files
    try:yield
    finally:
        with _LOCK:_PENDING[category]['bytes']-=size;_PENDING[category]['files']-=files


def owned_fixture(path):
    p=Path(path)
    if not p.is_absolute() or '..' in p.parts or not p.resolve().is_relative_to(ROOT/'fixtures'):raise ValueError('native-owned-fixture')
    for q in [p,*p.parents]:
        if q.is_relative_to(ROOT) and q.is_symlink():raise ValueError('native-fixture-symlink')
    return p


def write_input(path,data,maximum):
    if len(data)>maximum:raise RuntimeError('native-request-prewrite-cap')
    with reserve('F12',len(data),1):
        with Path(path).open('xb') as out:out.write(data)


class Call:
    """One immutable request/producer/forecast owns its exact result and row."""
    def __init__(self,adapter,owner,request,expected,env,producer,forecast,reservations,input_row):
        self.call_id=owner.call_id;self._adapter=adapter;self._owner=owner
        self._request=request;self._expected=expected;self._env=None if env is None else dict(env);self._producer=producer
        self._forecast=forecast;self._reservations=reservations;self._input_row=input_row
        self._row=None;self._value=None;self._error=None;self._done=False;self._lock=threading.Lock()
    def wait_started(self,timeout=None):return self._owner.wait_started(timeout)
    @property
    def started_proof_ref(self):return self._owner.started_proof_ref
    @property
    def input_released(self):return self._owner.input_released
    def interrupt(self,operation):return self._owner.interrupt(operation)
    def wait(self,timeout=None):
        with self._lock:
            if not self._done:
                try:
                    result=self._owner.wait(timeout)
                    stdout=Path(result['outputs']['stdout']['path']).read_bytes();stderr=Path(result['outputs']['stderr']['path']).read_bytes()
                    if not result['output_complete']:raise RuntimeError('native-output-incomplete')
                    value=admission.strict_json(stdout) if stdout else None
                    binding=result['producer_binding']
                    row={'call_id':self.call_id,'input':self._request,'env':self._env,'exit':result['exit'],'stdout':stdout.decode('utf-8'),'stderr':stderr.decode('utf-8'),'seconds':result['seconds'],'guarded_seconds':result['guarded_seconds'],'violations':[],'rss_limit':result['limit_bytes'],'sampled_peak_rss':result['peak_RSS_bytes'],'producer_kind':binding['producer_kind'],'producer_source_pin':binding['producer_source_pin'],'active_candidate_source_pin':result['active_candidate_source_pin'],'exe_sha256':result['binary_sha256'],'executable_sha256':result['binary_sha256'],'source_binding':binding,'forecast_id':self._forecast['id'],'native_role_result':str(Path(result['outputs']['stdout']['path']).parent/'result.json')}
                    self._row=row;self._value=value
                    with self._adapter._lock:
                        self._adapter.events.append(row)
                        self._adapter._log.emit({'kind':'native-call','call_id':self.call_id,'input':self._input_row,'env':self._env,'exit':row['exit'],'seconds':row['seconds'],'producer_kind':row['producer_kind'],'producer_source_pin':row['producer_source_pin'],'active_candidate_source_pin':row['active_candidate_source_pin'],'binary_sha256':row['exe_sha256'],'outputs':result['outputs'],'role_result':row['native_role_result'],'forecast_id':row['forecast_id'],'violations':[]})
                    if row['exit']!=self._expected:
                        # Only this real per-call semantic conflict participates in
                        # Harness' AssertionError contract. Technical failures did
                        # not return a role result and never arrive at this branch.
                        conflict=(self._request['action']=='publish' and self._expected==0 and row['exit']==2 and isinstance(value,dict) and value.get('error')=='conflict:expected-publication' and value.get('result') is None)
                        observed_exit=(row['exit']==86 and self._expected==0 and isinstance(self._env,dict) and self._env.get('EXP_EXIT') in self._forecast.get('exit_phases',[]))
                        if conflict or observed_exit:raise AssertionError(row)
                        raise RuntimeError('unexpected-native-semantic-response:'+repr(row))
                except BaseException as exc:
                    self._error=exc
                    semantic=(isinstance(exc,AssertionError) and self._row is not None and exc.args==(self._row,))
                    if not semantic:
                        try:
                            with self._adapter._lock:self._adapter._log.emit({'kind':'native-call-censored','call_id':self.call_id,'input':self._input_row,'env':self._env,'error':repr(exc),'owner_closed':self._owner.closed})
                        except BaseException as log_error:self._error=RuntimeError('native-call-and-censor-log-failure:'+repr((exc,log_error)))
                finally:
                    if self._owner.closed:
                        self._done=True;self._reservations.close()
                    else:
                        self._adapter._ownership_HOLD=True
            if self._error is not None:raise self._error
            return self._value
    @property
    def row(self):
        if not self._done:raise RuntimeError('native-call-result-not-complete')
        return self._row


class NativeAdapter:
    def __init__(self,config):
        fields={'run_name','control_only','engine_argv','candidate_binding','candidate_binding_sha256','forecasts','forecasts_sha256','source_pin','role_identity_budget','producers'}
        if not isinstance(config,dict) or set(config)!=fields:raise ValueError('native-config-closed-shape')
        name=config['run_name']
        if not isinstance(name,str) or not name.replace('-','').isalnum() or len(name)>64:raise ValueError('native-run-name')
        if type(config['control_only']) is not bool or type(config['role_identity_budget']) is not int or not 1<=config['role_identity_budget']<=4096:raise ValueError('native-control-identity-budget')
        self.config=config;self._lock=threading.RLock();self._serial=0;self.events=[];self._calls=[];self._ownership_HOLD=False
        self._folder=ROOT/'evidence/capsules'/name;self._folder.mkdir(parents=True,exist_ok=False)
        self._log=Events(self._folder/'calls.jsonl.gz',262144)
        self._forecasts=roles.producer.document(config['forecasts'],config['forecasts_sha256'],262144)
        if not isinstance(self._forecasts,dict) or set(self._forecasts)!={'source_pin','rows'} or self._forecasts['source_pin']!=config['source_pin'] or not isinstance(self._forecasts['rows'],list):raise ValueError('native-forecast-manifest')
        self._forecast_index={}
        for row in self._forecasts['rows']:
            fields={'id','case_id','action','F08_bytes','F08_files','stdout_cap','stderr_cap','input_cap','RSS_cap','receipt_cap','role'}
            if not isinstance(row,dict) or not fields<=set(row) or set(row)-fields-{'exit_phases'} or row['id'] in self._forecast_index:raise ValueError('native-forecast-closed-row')
            if any(type(row[k]) is not int or row[k]<0 for k in fields-{'id','case_id','action','role'}):raise ValueError('native-forecast-numeric-bound')
            if row['role'] not in ('reader','checker'):raise ValueError('native-forecast-explicit-role')
            if not isinstance(row.get('exit_phases',[]),list) or any(not isinstance(x,str) or not x for x in row.get('exit_phases',[])):raise ValueError('native-forecast-exit-phases')
            self._forecast_index[row['id']]=row
        if config['control_only']:
            if config['source_pin']!='CONTROL_ONLY' or Path(config['engine_argv'][0]).resolve()!=Path(roles.sys.executable).resolve():raise ValueError('native-control-Python-source')
            self.binary_sha256=roles.sha256(Path(config['engine_argv'][0]).resolve());self.source_binding={'source_pin':'CONTROL_ONLY','producer_kind':'control'}
        else:
            bound=roles.producer.document(config['candidate_binding'],config['candidate_binding_sha256'],65536)
            if bound['source_pin']!=config['source_pin'] or Path(config['engine_argv'][0]).resolve()!=Path(bound['binary_path']):raise ValueError('native-candidate-source-binary')
            self.binary_sha256=bound['binary_sha256'];self.source_binding=bound
    def start(self,root,action,*,case_id,forecast_id,code=0,env=None,on_started=None,timeout=115,producer='candidate',interrupt_grant=None,control_interrupts=(),**fields):
        if self._ownership_HOLD:raise RuntimeError('native-adapter-owned-closeout-HOLD')
        root=owned_fixture(root)
        if type(timeout) not in (int,float) or not math.isfinite(timeout) or not 0<timeout<=115:raise ValueError('native-call-timeout')
        reserved={'env','on_started','forecast_id','case_id','producer','code','root','action','exe_override'}
        if set(fields)&reserved:raise ValueError('native-request-options-in-payload')
        if env is not None and (not isinstance(env,dict) or any(not isinstance(k,str) or not k.startswith('EXP_') or not isinstance(v,str) for k,v in env.items())):raise ValueError('native-env-options')
        for key in ('EXP_READY','EXP_RELEASE','EXP_PACK_CANCEL'):
            if env is not None and key in env:owned_fixture(env[key])
        forecast=self._forecast_index.get(forecast_id)
        if forecast is None or (forecast['case_id'],forecast['action'])!=(case_id,action):raise RuntimeError('native-source-complete-case-forecast-required')
        if forecast['role']!=('reader' if action in READERS else 'checker'):raise ValueError('native-action-role-forecast')
        request={'root':str(root),'action':action,**fields};data=(json.dumps(request,separators=(',',':'),allow_nan=False)+'\n').encode()
        if len(data)>forecast['input_cap']:raise RuntimeError('native-request-cap')
        with self._lock:self._serial+=1;name=self.config['run_name']+'-'+str(self._serial)
        inputs=self._folder/(name+'-input.json');write_input(inputs,data,forecast['input_cap'])
        produced={'argv':list(self.config['engine_argv']),'historical_grant':None}
        if producer!='candidate':
            produced=self.config['producers'].get(producer)
            if not isinstance(produced,dict) or set(produced)!={'argv','historical_grant'} or produced['historical_grant'] is None:raise RuntimeError('native-parent-released-producer-required')
        allowed=[code]
        if action=='publish' and code==0:allowed=[0,2]
        if isinstance(env,dict) and 'EXP_EXIT' in env:
            if env['EXP_EXIT'] not in forecast.get('exit_phases',[]):raise RuntimeError('source-mapped-exit-phase-required')
            if code==0:allowed.append(86)
        stack=contextlib.ExitStack()
        try:
            # Snapshot already counts every existing root exactly once. Only
            # per-case new generation/content/metadata/temp growth is reserved.
            # Pending bounds remain conservative until actual operation closeout.
            stack.enter_context(reserve('F08',forecast['F08_bytes'],forecast['F08_files']))
            stack.enter_context(reserve('F12',forecast['stdout_cap']+forecast['stderr_cap']+forecast['RSS_cap']+forecast['receipt_cap'],4))
            options={'name':name,'timeout':timeout,'identity_budget':self.config['role_identity_budget'],'binding':self.config['candidate_binding'],'binding_sha256':self.config['candidate_binding_sha256'],'input_path':inputs,'evidence_cap':forecast['RSS_cap'],'control_only':self.config['control_only'],'expected_exit':tuple(dict.fromkeys(allowed)),'env':env,'stdout_cap':forecast['stdout_cap'],'stderr_cap':forecast['stderr_cap'],'input_cap':forecast['input_cap'],'receipt_cap':forecast['receipt_cap'],'case_context':{'parent_run_id':self.config['run_name'],'case_id':case_id},'historical_grant':produced['historical_grant'],'interrupt_grant':interrupt_grant,'control_interrupts':control_interrupts,'on_started':on_started}
            owner=roles.OwnedCall(forecast['role'],produced['argv'],**options)
            call=Call(self,owner,request,code,env,produced,forecast,stack,{'path':str(inputs),'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()})
            with self._lock:self._calls.append(call)
            return call
        except BaseException:stack.close();raise
    def call(self,root,action,**options):return self.start(root,action,**options).wait()
    def close(self):
        errors=[]
        for call in list(self._calls):
            if call._done:continue
            try:call.wait()
            except BaseException as exc:errors.append(repr(exc))
        self._log.close()
        return errors


class Driver:
    """Facade preserves per-call AssertionError rows and immutable identities.
    Binding to the final shared Driver signature awaits its exact README; callers
    currently supply explicit case/forecast IDs, never an implicit generic guess.
    """
    def __init__(self,adapter):
        self.adapter=adapter;self.events=adapter.events
        self.binary_sha256=adapter.binary_sha256;self.source_binding=adapter.source_binding
    def call(self,root,action,timeout=120,code=0,env=None,*,exe_override=None,producer='candidate',on_started=None,case_id=None,forecast_id=None,**fields):
        if exe_override is not None:
            target=Path(exe_override).resolve(strict=True);candidate=Path(self.adapter.config['engine_argv'][0]).resolve(strict=True)
            if target!=candidate:
                matches=[key for key,row in self.adapter.config['producers'].items() if isinstance(row,dict) and Path(row['argv'][0]).resolve(strict=True)==target and row.get('historical_grant') is not None]
                if len(matches)!=1:raise RuntimeError('unbound-historical-executable-override')
                if producer not in ('candidate',matches[0]):raise RuntimeError('historical-producer-override-conflict')
                producer=matches[0]
        if case_id is None or forecast_id is None:
            context=self.adapter.current_case(root) if hasattr(self.adapter,'current_case') else None
            if context is None:raise RuntimeError('shared-case-preflight-context-required')
            case_id=context['case_id'];forecast_id=context['forecast_for_call'](action,fields,env)
        return self.adapter.call(root,action,producer=producer,case_id=case_id,forecast_id=forecast_id,code=code,env=env,on_started=on_started,timeout=min(timeout,115),**fields)


def require_shared_protocol(release,expected_sha256):
    """Read-only exact manifest check; does not import a legacy decoder/core."""
    row=roles.producer.document(release,expected_sha256,262144)
    fields={'source_pin','protocol_core','shared_v5_decoder','case_families','workspace_units','ordinary_cases','focused_cases','version_map','parent_released'}
    if not isinstance(row,dict) or set(row)!=fields or row['parent_released'] is not True:raise RuntimeError('frozen-shared-protocol-release-required')
    for key in ('protocol_core','shared_v5_decoder'):
        item=row[key]
        if not isinstance(item,dict) or set(item)!={'path','bytes','sha256'}:raise ValueError('shared-protocol-member-shape')
        p=Path(item['path'])
        if p.is_symlink() or not p.is_file() or not p.resolve().is_relative_to(ROOT/'source') or p.stat().st_size!=item['bytes'] or roles.sha256(p)!=item['sha256']:raise RuntimeError('shared-protocol-member-exact-source')
    for family,count in [('packing',52),('strict',24),('recovery',24),('focus',8)]:
        cases=row['case_families'].get(family)
        if not isinstance(cases,list) or len(cases)!=count or len(set(cases))!=count:raise RuntimeError('shared-case-ID-coverage:'+family)
    if not row['workspace_units'] or not row['ordinary_cases'] or not row['focused_cases'] or not row['version_map']:raise RuntimeError('shared-full-unit-ordinary-focused-version-map-required')
    return row


if __name__=='__main__':raise SystemExit('SOURCE_ONLY_FROZEN_SHARED_PROTOCOL_AND_PARENT_CONTROL_REVIEW_REQUIRED')
