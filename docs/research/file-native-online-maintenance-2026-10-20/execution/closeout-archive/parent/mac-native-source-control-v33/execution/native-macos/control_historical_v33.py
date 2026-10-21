"""PROPOSED synthetic validator controls only; never launches an old binary.
All synthetic sources/grants are F08 fixtures. Temporary admission.ROOT is scoped
only to this serial pure-loader test after every owned role call has closed.
"""
import hashlib,json,sys,time
from pathlib import Path
import admission_v20 as admission
import producer_binding_v32 as producer


def main():
 actual=admission.ROOT;shadow=actual/'fixtures/native-controls-v33/historical-shadow';shadow.mkdir(parents=True,exist_ok=False)
 def write(relative,value):
  p=shadow/relative;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(value));return str(p)
 def ref(path):return {'path':path,'sha256':producer.digest(path)}
 guard=producer.digest(Path(__file__).with_name('guard_v20.py'));final='SYNTHETIC_FINAL_V32';old='SYNTHETIC_HISTORICAL_V32'
 current=Path(sys.executable).resolve();historical=Path('/bin/echo').resolve()
 p=shadow/'source/tiny.rs';p.parent.mkdir();p.write_bytes(b'candidate synthetic source\n')
 source=write('evidence/source-binding.json',{'source_pin':final,'engine_release':True,'guard_sha256':guard,'source_files':[{'path':'tiny.rs','bytes':p.stat().st_size,'sha256':producer.digest(p)}]})
 binding=write('evidence/candidate-binding.json',{'source_pin':final,'source_binding':source,'source_binding_sha256':producer.digest(source),'guard_sha256':guard,'binary_path':str(current),'binary_sha256':producer.digest(current),'engine_release':True})
 oldsource=shadow/'old-source/old.rs';oldsource.parent.mkdir();oldsource.write_bytes(b'historical synthetic source\n')
 manifest=write('evidence/old-source-manifest.json',{'source_pin':old,'files':[{'path':'old.rs','bytes':oldsource.stat().st_size,'sha256':producer.digest(oldsource)}]})
 receipt=write('evidence/old-build-receipt.json',{'kind':'synthetic-validator-only-no-build','source_pin':old,'binary_sha256':producer.digest(historical)})
 payload=(json.dumps({'root':str(shadow/'fixtures/historical-case/disposable/store'),'action':'init','mode':'B','count':16},separators=(',',':'))+'\n').encode();env={};context={'parent_run_id':'synthetic-v33','case_id':'historical-validator'};until=time.time()+30
 grant={'kind':'native-historical-case-grant-v1','parent_released':True,'grant_id':'synthetic-not-an-execution-grant','parent_run_id':context['parent_run_id'],'case_id':context['case_id'],'active_candidate_pin':final,'active_candidate_source_binding_sha256':producer.digest(source),'active_guard_sha256':guard,'historical_source_pin':old,'historical_source_root':str(oldsource.parent),'historical_source_manifest_path':manifest,'historical_source_manifest_sha256':producer.digest(manifest),'historical_build_receipt_path':receipt,'historical_build_receipt_sha256':producer.digest(receipt),'historical_binary_path':str(historical),'historical_binary_sha256':producer.digest(historical),'allowed_operations':['init'],'allowed_request_sha256':[hashlib.sha256(payload).hexdigest()],'allowed_environment_sha256':[hashlib.sha256(b'{"EXP_NAVIGATION":"4"}').hexdigest()],'allowed_new_fixture_prefix':'fixtures/historical-case','fixed_expiry':until}
 gp=write('evidence/old-case-grant.json',grant);reference=ref(gp);token={'source_pin':final,'fixed_cutoff':until+1};rows=[]
 def load(*,binary=historical,body=payload,ctx=context,g=reference):return producer.bind(binary,token,binding,producer.digest(binding),request_bytes=body,env=env,case_context=ctx,historical_grant=g)
 admission.ROOT=shadow
 try:
  good=load();assert good['producer_kind']=='historical' and good['producer_source_pin']==old and good['active_candidate_source_pin']==final and good['binary_sha256']==producer.digest(historical)
  tests=[('wrong-case',lambda:load(ctx={'case_id':'other','parent_run_id':context['parent_run_id']})),('wrong-request',lambda:load(body=payload+b' ')),('wrong-binary',lambda:load(binary=Path('/bin/cat').resolve()))]
  for name,changes in [('unknown-field',{'unexpected':True}),('wrong-active-pin',{'active_candidate_pin':'OTHER'}),('expired',{'fixed_expiry':time.time()-1})]:
   q=write('evidence/'+name+'.json',dict(grant,**changes));tests.append((name,lambda q=q:load(g=ref(q))))
  for name,fn in tests:
   try:fn()
   except (RuntimeError,ValueError) as e:rows.append({'case':name,'refused':str(e)})
   else:raise AssertionError('historical validator accepted '+name)
  saved=oldsource.read_bytes();oldsource.write_bytes(saved+b'changed')
  try:
   try:load()
   except RuntimeError as e:assert 'source-member-mismatch' in str(e);rows.append({'case':'historical-source-byte-drift','refused':str(e)})
   else:raise AssertionError('historical source drift accepted')
  finally:oldsource.write_bytes(saved)
 finally:admission.ROOT=actual
 admission.bounded_receipt(actual/'evidence/control/historical-validator-v33-proof.json',{'kind':'synthetic-historical-binding-validator-only','positive_actual_old_labels':True,'negative_controls':rows,'old_binary_launched':False,'candidate_or_historical_engine_qualified':False,'ROOT_restored':admission.ROOT==actual})
 return rows

if __name__=='__main__':raise SystemExit('SOURCE_ONLY_PARENT_CONTROL_REVIEW_REQUIRED')
