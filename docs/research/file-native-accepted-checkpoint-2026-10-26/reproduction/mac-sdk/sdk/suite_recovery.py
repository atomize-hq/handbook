"""R24 native constructors, preserving the protected capsule's exact obligations."""
from pathlib import Path
import hashlib
from suite_bootstrap import need
from native_resources_v3 import strict

ANCESTOR='docs/item-attachments-d000009/evidence'
DEEP=ANCESTOR+'/deep/source.txt'

def reference(c):return c.call('dump')['result']['state']
def navigation_case(c,mode,n):
    c.init(mode,n);initial=reference(c);c.verify('initial-navigation')
    c.publish('semantic-change',[dict(kind='delete',id='d000000'),dict(kind='rename',id='d000001',text='Renamed title'),
        dict(kind='edge',id='d000001',text='cites',target='d000002'),dict(kind='edge',id='d000001',text='cites',target='d000002'),
        dict(kind='path',id='d000009',text=ANCESTOR)])
    state=reference(c);need(set(state['docs'])==set(initial['docs'])-{'d000000'},'exact-document-deletion')
    need(state['docs']['d000001']['title']=='Renamed title' and state['docs']['d000009']['path']==ANCESTOR,'exact-title-path')
    expected=[dict(kind='cites',target='d000002',external=False)]*2
    need(state['docs']['d000001']['edges'][-2:]==expected,'ordered-duplicate-relationships')
    c.verify('ordered-edges')
    c.publish('inverse',[dict(kind='path',id='d000009',text=DEEP)]);c.verify('inverse-namespace')
    target='d000002';ops=[]
    for key,doc in reference(c)['docs'].items():
        for ordinal,edge in enumerate(doc['edges']):
            if not edge['external'] and edge['target']==target:ops.append(dict(kind='drop-edge',id=key,value=ordinal))
    c.publish('remove-target',ops+[dict(kind='delete',id=target)]);c.verify('removed-target')
    c.call('cancel',request_id='terminal-fence');pin=c.selected()
    c.call('lifecycle',operation='pin',name='durable-pin',selected=pin)
    packet=c.call('packet')['result'];packet_prefix=packet['path']
    packet_bytes={p.relative_to(c.root).as_posix():hashlib.sha256(p.read_bytes()).hexdigest()
                  for p in (c.root/packet_prefix).rglob('*') if p.is_file()}
    if mode=='A':c.publish('pending-backup-transition',[dict(kind='path',id='d000009',text=ANCESTOR)],env={'EXP_DEFER_REFRESH':'1'})
    request=dict(request='eligible',base=c.selected(),ops=[dict(kind='label',id='d000003',text='eligible')],packet=False)
    candidate=c.call('prepare',request=request)['result']['candidate']
    if mode=='A':
        c.call('refresh',code=86,env={'EXP_EXIT':'refresh-file'})
        need(not (c.root/'view'/ANCESTOR).exists(),'genuine-pending-refresh')
    restored=c.a.backup_restore(c,'operational-restored')
    need(restored.selected()==c.selected() and not (restored.root/'DISPOSABLE.json').is_symlink(),'selected-restored-exact')
    if mode=='A':
        need(restored.read('PENDING.json')==c.read('PENDING.json') and restored.read('VIEW.json')==c.read('VIEW.json'),'pending-view-restored')
    restored.verify('full-operational-restoration')
    need(restored.call('resolve',request_id='terminal-fence')['result']['outcome']=='cancelled-fenced','fence-survived')
    need(restored.document('PINS.json')['durable-pin']==pin,'persistent-pin-restored')
    for name,h in packet_bytes.items():need(hashlib.sha256(restored.read(name,8*1024**2)).hexdigest()==h,'packet-bytes-restored')
    restored.call('select',candidate=candidate)
    need(restored.call('resolve',request_id='eligible')['result']['outcome']=='accepted','eligible-candidate-restored')
    if mode=='A':restored.call('resume')
    restored.publish('valid-successor',[]);restored.verify('accepted-successor')
    restored.a.preserve_move(restored.root,'SELECTED.json')
    restored.call('inspect',code=2)
    saved=restored.read('SELECTED.json.withheld');restored.write('SELECTED.json',saved)
    restored.call('lifecycle',operation='retire',selected=pin,code=2)
    return restored.verify('required-evidence-refusal-and-restoration')

def namespace_case(c,direction,phase):
    c.init('A',16)
    if direction=='down':c.publish('up-first',[dict(kind='path',id='d000009',text=ANCESTOR)])
    else:
        c.write('view/'+DEEP,b'external divergent source')
        c.write('view/'+ANCESTOR+'/external.txt',b'external subtree')
    target=ANCESTOR if direction=='up' else DEEP
    if direction=='down' and phase=='refresh-preserved':c.write('view/'+ANCESTOR,b'changed ancestor')
    c.publish('interrupted',[dict(kind='path',id='d000009',text=target)],env={'EXP_DEFER_REFRESH':'1'})
    c.call('refresh',code=86,env={'EXP_EXIT':phase})
    proof=c.verify('critical-namespace-crash')
    need(proof['underlying']['navigation']['readable_declared_current'] is False,'incomplete-view-must-be-declared')
    if phase=='refresh-namespace-preserved':
        c.write('view/'+ANCESTOR+'/second-external.txt',b'external second incarnation')
    c.publish('correcting',[dict(kind='path',id='d000009',text='docs/item-corrected-d000009.txt')],env={'EXP_DEFER_REFRESH':'1'})
    c.call('refresh');c.call('resume')
    if direction=='up':
        preserved=[p.read_bytes() for p in (c.root/'divergence').rglob('*') if p.is_file()]
        need(b'external divergent source' in preserved and b'external subtree' in preserved,'external-namespace-divergence-lost')
        if phase=='refresh-namespace-preserved':need(b'external second incarnation' in preserved,'second-incarnation-lost')
    return c.verify('correcting-successor')

def accounting(c,mode):
    c.init(mode,16);accepted=c.lost('transfer','accounting-transfer',[dict(kind='label',id='d000000',text='transfer')])
    interrupted=c.selected();c.call('checkpoint',request_id='checkpoint')
    c.call('lifecycle',operation='retire',selected=interrupted)
    c.call('lifecycle',operation='collect');c.call('lifecycle',operation='collect')
    need(c.call('resolve',request_id='transfer')['result']==accepted,'permanent-outcome-after-repeat-collection')
    return c.verify('accounting-transfer')

def half_preparation(c,mode):
    c.init(mode,16);c.publish('half',[],code=86,env={'EXP_EXIT':'allocation'})
    restored=c.a.backup_restore(c,'half-restored')
    restored.call('lifecycle',operation='collect',code=2)
    registry=restored.document('RESERVATIONS.json')
    need('version' not in registry and len(registry)==1,'legacy-capsule-must-not-be-auto-converted-to-v2')
    restored.call('lifecycle',operation='abandon',attempt=next(iter(registry)))
    restored.call('lifecycle',operation='collect');restored.publish('after-half',[])
    return restored.verify('half-preparation-restored')

def selected_once(c,mode):
    from suite_ordinary import ordinary
    c.init(mode,16);saved=c.selected();old=ordinary(c,'selected-once-before',selected=saved) if mode=='B' else None
    c.publish('skip-1',[dict(kind='delete',id='d000000')],env={'EXP_DEFER_REFRESH':'1'})
    c.publish('skip-2',[dict(kind='path',id='d000009',text=ANCESTOR)],env={'EXP_DEFER_REFRESH':'1'})
    if mode=='A':
        need(c.verify('declared-mixed-view')['underlying']['navigation']['readable_declared_current'] is False,'A-skipped-view-declaration')
        c.call('refresh')
    else:need(ordinary(c,'selected-once-after',selected=saved)['files']==old['files'],'immutable-B-selected-once')
    return c.verify('skipped-or-selected-once-final')

def dispatch(c,name):
    if name.startswith('navigation-and-operational-backup/'):
        _,mode,n=name.split('/');return navigation_case(c,mode,int(n[1:]))
    if name.startswith('namespace-interruption-correcting-successor/'):
        _,direction,phase=name.split('/');return namespace_case(c,direction,phase)
    if name.startswith('checkpoint-transfer-retire-repeat-gc/'):return accounting(c,name[-1])
    if name.startswith('half-preparation-backup-recovery/'):return half_preparation(c,name[-1])
    if name=='impossible-final-namespace':
        c.init('A',16);before=c.selected();c.publish('impossible',[dict(kind='path',id='d000008',text=ANCESTOR)],code=2)
        need(c.selected()==before,'impossible-operation-changed-selected');return c.verify('impossible-refusal')
    if name.startswith('skipped-or-selected-once/'):return selected_once(c,name[-1])
    raise RuntimeError('closed-R24-ID-unimplemented:'+name)

# Native command ownership shared with workspace, oracle and ordinary routes.
import base64,copy,json,os,selectors,signal,subprocess,threading,time,uuid,sys,fcntl
READER_ACTIONS={'resolve','point','inspect','lookup-bench','ordinary'}
import native_guard as guard
from suite_bootstrap import encoded,digest,owned,retain_birth,observed_children,OwnedBirthUncertain
from native_resources_v3 import confined,regular_read
def descendants(pid):
    answer=set()
    try:
        for task in Path('/proc',str(pid),'task').iterdir():
            try: answer.update(map(int,(task/'children').read_text().split()))
            except FileNotFoundError: pass
    except FileNotFoundError: pass
    return answer
class Commands:
    def __init__(self,host):
        self.host=host;self.lock=threading.RLock();self.active={}
    def close(self):
        errors=[]
        with self.lock:
            for state in self.active.values():
                try:self._kill(state,'case-close')
                except BaseException as e:errors.append(repr(e))
        for state in list(self.active.values()):
            if not state['done'].wait(5):errors.append('command-owner-not-joined')
        need(not errors,'native-command-close:'+repr(errors))
    def _kill(self,state,reason):
        # Only our retained pidfds are authority; no reopened PID or foreign group.
        with self.lock:retained=list(state['owned'].items())
        for pid,(start,fd) in retained:
            try:signal.pidfd_send_signal(fd,signal.SIGKILL)
            except ProcessLookupError:pass
            except BaseException as error:state['errors'].append('retained-signal:'+repr(error))
        state['kill_reasons'].append(reason)
    def _retain(self,state,row,stage,parent=None):
        with self.lock:
            starts={pid:birth for pid,(birth,fd) in state['owned'].items()}
            handles={pid:fd for pid,(birth,fd) in state['owned'].items()}
            retain_birth(starts,handles,row,self.lock,stage,parent)
            state['owned'][row['pid']]=(starts[row['pid']],handles[row['pid']])

    def run(self,root,action,argv,stdin=b'',*,expected=0,seconds=120,env=None):
        need(type(stdin) is bytes,'command-bytes-input')
        self.host.command_binding(action,argv,stdin,env)
        root=owned(root,guard.ROOT/'fixtures')
        deadline=min(time.time()+seconds,self.host.driver.owner['absolute_deadline'],guard.CUTOFF)
        need(deadline>time.time(),'native-command-fixed-deadline')
        limit=256*1024**2 if action in READER_ACTIONS else 2*1024**3
        executable=Path(argv[0]);binary_sha=digest(executable)
        token=uuid.uuid4().hex;gate_r,gate_w=os.pipe()
        state=dict(owned={},done=threading.Event(),kill_reasons=[],errors=[],pid=None)
        stop=threading.Event();watcher=None;p=None;rawout=bytearray();rawerr=bytearray()
        maximum=0;observations=[];sent=0;full=False;input_closed=False;selection=selectors.DefaultSelector()
        lease=None;wire_lease=None;failure=None;record=None;initial=None;exec_proof=None;pipe_fds=[]
        try:
            # Allocate exact output geometry and reserve before creating a child.
            out_r,out_w=os.pipe();pipe_fds.extend([out_r,out_w])
            err_r,err_w=os.pipe();pipe_fds.extend([err_r,err_w])
            tail=sum(fcntl.fcntl(fd,fcntl.F_GETPIPE_SZ) for fd in (out_w,err_w))
            payload=dict(root=str(root),action=action,operation=action,argv=argv,stdin_bytes=len(stdin),
                         stdin_sha256=hashlib.sha256(stdin).hexdigest())
            lease=self.host.budget.reserve_call(token,payload,env,tail)
            need(len(stdin)<=lease['input_bytes'],'complete-command-stdin-forecast')
            wire_bound=(4*(len(stdin)+lease['stdout_bytes']+lease['stderr_bytes']+tail)+2)//3+8192
            self.host.budget.reserve_evidence('command-wire-'+token,wire_bound,0)
            wire_lease='command-wire-'+token
            p=subprocess.Popen([sys.executable,str(Path(guard.__file__).resolve()),'--child',str(gate_r),*argv],
                               stdin=subprocess.PIPE,stdout=out_w,stderr=err_w,
                               pass_fds=(gate_r,),close_fds=True,env=self.host.environment(env))
            os.close(out_w);pipe_fds.remove(out_w);os.close(err_w);pipe_fds.remove(err_w)
            p.stdout=os.fdopen(out_r,'rb',buffering=0);pipe_fds.remove(out_r)
            p.stderr=os.fdopen(err_r,'rb',buffering=0);pipe_fds.remove(err_r)
            os.close(gate_r);gate_r=None
            initial=guard.process(p.pid)
            need(initial and initial['rss'] and initial['hwm'] and initial['pgid']==os.getpgrp(),'blocked-native-command-positive-rss')
            state['pid']=p.pid;self._retain(state,initial,'command-root')
            with self.lock:self.active[token]=state
            def memory():
                nonlocal maximum
                try:
                    while not stop.is_set():
                        with self.lock:
                            pending=[];seen=set()
                            for pid,(start,fd) in list(state['owned'].items()):
                                row=guard.process(pid)
                                if row is None:continue
                                if row['start']!=start:raise OwnedBirthUncertain(pid,start,'command-monitor',row['start'],'retained-PID-reused')
                                pending.append((row,None))
                            while pending:
                                row,parent=pending.pop();pid=row['pid']
                                if pid in seen:continue
                                seen.add(pid)
                                self._retain(state,row,'command-child',parent)
                                if row['state']=='Z':continue
                                need(row['pgid']==os.getpgrp(),'command-child-left-owned-group')
                                need(state['owned'][pid][0]==row['start'] and row['rss'] is not None and row['hwm'] is not None,'command-live-rss-identity')
                                maximum=max(maximum,row['hwm'])
                                observations.append(dict(epoch=time.time(),pid=pid,start=row['start'],rss=row['rss'],hwm=row['hwm']))
                                if row['rss']>limit:
                                    state['errors'].append('command-role-rss-censor');self._kill(state,'rss')
                                pending.extend((child,row) for child in observed_children(row))
                            if time.time()>=deadline:state['errors'].append('command-time-censor');self._kill(state,'deadline')
                        stop.wait(.01)
                except BaseException as e:
                    state['errors'].append('command-monitor:'+repr(e))
                    with self.lock:self._kill(state,'monitor-error')
            watcher=threading.Thread(target=memory,daemon=False);watcher.start()
            os.write(gate_w,b'G');os.close(gate_w);gate_w=None
            # Bash/Python stdin scripts cannot begin file I/O before this proof.
            while True:
                need(time.time()<deadline,'actual-command-exec-deadline')
                row=guard.process(p.pid);need(row and row['start']==initial['start'] and row['state']!='Z','command-exited-before-native-exec-proof')
                try:actual=Path(os.readlink('/proc/'+str(p.pid)+'/exe'))
                except FileNotFoundError:actual=None
                if actual==executable.resolve():
                    live_exe=Path('/proc',str(p.pid),'exe');live_stat=live_exe.stat();named_stat=executable.stat()
                    need(row['rss'] and row['rss']<=limit and digest(live_exe)==binary_sha==digest(executable) and
                         (live_stat.st_dev,live_stat.st_ino)==(named_stat.st_dev,named_stat.st_ino),'actual-command-exec-rss-sha-inode')
                    exec_proof=dict(path=str(actual),sha256=binary_sha,device=live_stat.st_dev,inode=live_stat.st_ino,
                                    rss=row['rss'],hwm=row['hwm'],before_stdin_delivery=True)
                    break
                time.sleep(.001)
            started_ns=time.perf_counter_ns();self.host.command_started(token,p.pid,initial['start'],action,started_ns)
            for name,f in [('stdin',p.stdin),('stdout',p.stdout),('stderr',p.stderr)]:
                os.set_blocking(f.fileno(),False)
                selection.register(f,selectors.EVENT_WRITE if name=='stdin' else selectors.EVENT_READ,name)
            kill_at=None
            while selection.get_map():
                if state['errors']:
                    kill_at=kill_at or time.monotonic()
                    need(time.monotonic()-kill_at<5,'command-pipe-closure-censor')
                ready=selection.select(.01);ready.sort(key=lambda x:x[0].data=='stdin')
                for item,_ in ready:
                    f,name=item.fileobj,item.data
                    if name=='stdin':
                        try:sent+=os.write(f.fileno(),stdin[sent:sent+65536])
                        except BrokenPipeError:
                            state['errors'].append('command-incomplete-input');selection.unregister(f);f.close();input_closed=True
                            continue
                        if sent==len(stdin):
                            selection.unregister(f);f.close();input_closed=True;full=True
                    else:
                        try:block=os.read(f.fileno(),16384)
                        except BlockingIOError:continue
                        if not block:selection.unregister(f);f.close();continue
                        target=rawout if name=='stdout' else rawerr;target.extend(block)
                        if len(target)>lease[name+'_bytes']:
                            state['errors'].append('command-output-censor:'+name)
                            with self.lock:self._kill(state,'output')
                        need(len(rawout)+len(rawerr)<=lease['stdout_bytes']+lease['stderr_bytes']+tail,'command-tail-reservation-exceeded')
            p.wait(timeout=5)
            need(full or not stdin,'command-full-input-required')
        except BaseException as e:
            failure=repr(e);state['errors'].append(failure)
        finally:
            if p is not None:
                with self.lock:self._kill(state,'owner-finally')
                try:p.wait(timeout=5)
                except subprocess.TimeoutExpired:state['errors'].append('command-root-not-reaped')
            stop.set()
            if watcher is not None:
                watcher.join(5)
                if watcher.is_alive():state['errors'].append('command-monitor-not-joined')
            selection.close()
            for fd in (gate_r,gate_w):
                if fd is not None:os.close(fd)
            for fd in pipe_fds:os.close(fd)
            if p is not None:
                # Drain setup-failure tails in this owner only. A missing EOF is
                # an explicit raw-capture failure, never a truncated success.
                for f,target in [(p.stdout,rawout),(p.stderr,rawerr)]:
                    if f.closed:continue
                    os.set_blocking(f.fileno(),False)
                    while True:
                        try:b=os.read(f.fileno(),16384)
                        except BlockingIOError:state['errors'].append('command-raw-eof-not-observed');break
                        if not b:break
                        target.extend(b)
                    f.close()
                if p.stdin and not p.stdin.closed:p.stdin.close()
            with self.lock:
                if watcher is None or not watcher.is_alive():
                    for start,fd in state['owned'].values():os.close(fd)
                record=dict(kind='native-command',token=token,root=str(root),action=action,argv=argv,
                    env=env,binary_sha256=binary_sha,pid=state['pid'],start=initial['start'] if initial else None,
                    initial_blocked_rss=initial['rss'] if initial else None,actual_exec=exec_proof,
                    stdin_bytes=len(stdin),input_bytes_sent=sent,input_fully_delivered=full,stdin_closed=input_closed,
                    stdin_sha256=hashlib.sha256(stdin).hexdigest(),stdin_bytes_b64=base64.b64encode(stdin).decode(),
                    stdout=rawout.decode(errors='replace'),stderr=rawerr.decode(errors='replace'),
                    stdout_bytes_b64=base64.b64encode(rawout).decode(),stderr_bytes_b64=base64.b64encode(rawerr).decode(),
                    stdout_sha256=hashlib.sha256(rawout).hexdigest(),stderr_sha256=hashlib.sha256(rawerr).hexdigest(),
                    exit=p.returncode if p else None,errors=state['errors'],memory=observations,peak_rss=maximum,
                    rss_limit=limit,started_ns=locals().get('started_ns'),done_ns=time.perf_counter_ns(),
                    root_reaped=p is not None and p.returncode is not None,
                    watcher_joined=watcher is None or not watcher.is_alive(),outer_guard_closure_still_required=True)
            try:
                self.host.emit(record)
                if lease is not None:self.host.budget.settle(token)
                if wire_lease is not None:self.host.budget.settle(wire_lease)
            finally:state['done'].set()
        need(record['exit']==expected and not state['errors'],'native-command-failed:'+repr(record))
        return record
