"""Bind exact primary index symbols before attribution-only isolated edits."""
import json, pathlib, subprocess
HERE=pathlib.Path(__file__).resolve().parent
REPO=HERE.parents[2]
CLI=pathlib.Path('C:/Users/spmcc/AppData/Local/pnpm-cache/dlx/0d139a0cd124739f2ff8499929822f327233958cc0ac609c1c658e58fe68e1fc/1a1280908f9-61a0/node_modules/.pnpm/gitnexus@1.6.12_graphology-types@0.24.8_zod@4.6.5/node_modules/gitnexus/dist/cli/index.js')
SELECTED={
 'work_state/transaction.rs':['scan','parsed','output','decision','terminal','publication','noncommit','execute','validate_authority_history','current_grant','preflight'],
 'work_state/store.rs':['inventory','manifest','required','open'],
 'work_state/mod.rs':['encode'],
 'work_state/scope.rs':['Function:crates/engine/src/work_state/scope.rs:WorkCompositionClauseV1.permits#1','Function:crates/engine/src/work_state/scope.rs:WorkRegistrationScopeV1.permits#1'],
 'local_authority/work_registration.rs':['with_work_registration_current','with_work_registration_establishment','validate_current','capture_proof','validate_historical'],
 'local_authority/local_check.rs':['work_registration_current'],
 'local_authority.rs':['acquire'],
 'repository_durability.rs':['read_regular_bounded','list_directory_bounded','pin_windows_parent','windows_handle_info','open_windows_directory_guard','acquire_existing_exclusive_lock_v1','lock_existing','drop','release'],
 'definition_identity.rs':['canonical_json_bytes'],
}
def call(args):
    p=subprocess.run(['node',str(CLI),*args,'--repo',str(REPO)],capture_output=True,text=True,encoding='utf-8')
    pos=p.stdout.find('{')
    return json.loads(p.stdout[pos:]) if pos>=0 else {'error':p.stderr,'exit':p.returncode}
def main():
    output=json.loads((HERE/'instrumentation-gitnexus-impact.json').read_text()) if (HERE/'instrumentation-gitnexus-impact.json').exists() else []
    for file,names in SELECTED.items():
        path='crates/engine/src/'+file
        for name in names:
            if any(x['file']==path and x.get('requested',x['name'])==name for x in output):continue
            context=call(['context',name,'--file',path]) if not name.startswith('Function:') else call(['context',name.rsplit(':',1)[-1].split('.')[1].split('#')[0],'--file',path])
            uid=context.get('symbol',{}).get('uid')
            if not uid and context.get('status')=='ambiguous':
                candidates=[x for x in context['candidates'] if x['kind']=='Function' and x['filePath']==path]
                if name.startswith('Function:'):candidates=[x for x in candidates if x['uid']==name]
                if len(candidates)==1:
                    uid=candidates[0]['uid']
            if not uid:raise RuntimeError(json.dumps({'file':path,'name':name,'context':context}))
            impact=call(['impact',uid,'--direction','upstream'])
            if impact.get('risk')=='UNKNOWN':
                search_name=name.rsplit('.',1)[-1].split('#')[0] if name.startswith('Function:') else name
                direct=subprocess.run(['rg','-n',search_name+r'\(','crates/engine/src'],cwd=REPO,capture_output=True,text=True,encoding='utf-8').stdout
                if not direct:raise RuntimeError(json.dumps({'file':path,'name':name,'impact':impact}))
                impact['source_crosscheck']=direct.splitlines()
                impact['attribution_only_gate']='Resolved source references reviewed; graph UNKNOWN is retained as a limitation, not low risk.'
            output.append({'file':path,'name':name.rsplit('.',1)[-1].split('#')[0] if name.startswith('Function:') else name,'requested':name,'uid':uid,'context':context,'impact':impact})
            (HERE/'instrumentation-gitnexus-impact.json').write_text(json.dumps(output,indent=2)+'\n')
            print(json.dumps({'file':path,'name':name,'risk':impact.get('risk'),'count':impact.get('impactedCount')}),flush=True)
if __name__=='__main__':main()
