"""Add attribution-only hooks to disposable sources; never change algorithms."""
import hashlib,json,pathlib,re,shutil,subprocess,sys
HERE=pathlib.Path(__file__).resolve().parent
PRIMARY=HERE.parents[2]
target=pathlib.Path(sys.argv[1]).resolve()
if target==PRIMARY or PRIMARY in target.parents:raise ValueError('isolated source required')
impacts=json.loads((HERE/'instrumentation-gitnexus-impact.json').read_text())
selected={
 'work_state/transaction.rs':{'scan','parsed','output','decision','terminal','publication','noncommit','execute','validate_authority_history','current_grant','preflight'},
 'work_state/store.rs':{'inventory','manifest','required','open'},
 'work_state/mod.rs':{'encode'},
 'work_state/scope.rs':{'permits'},
 'local_authority/work_registration.rs':{'with_work_registration_current','with_work_registration_establishment','validate_current','capture_proof','validate_historical'},
 'local_authority/local_check.rs':{'work_registration_current'},
 'local_authority.rs':{'acquire'},
 'repository_durability.rs':{'read_regular_bounded','list_directory_bounded','pin_windows_parent','windows_handle_info','open_windows_directory_guard','acquire_existing_exclusive_lock_v1','lock_existing'},
 'definition_identity.rs':{'canonical_json_bytes'},
}
def replace(s,old,new,count=1):
    if s.count(old)!=count:raise ValueError(f'Unexpected source multiplicity {s.count(old)} for {old[:80]!r}')
    return s.replace(old,new,count)
inserts=[];provenance=[]
for file,names in selected.items():
    path=target/'crates/engine/src'/file;s=path.read_text(encoding='utf-8');before=hashlib.sha256(path.read_bytes()).hexdigest()
    if '_research_span' in s:raise ValueError('already instrumented')
    is_refactor=file=='work_state/transaction.rs' and 'fn scan_with_artifacts(' in s
    # New helper owned by the previously impact-checked scan/parser; no graph
    # identity exists for it at the baseline index.
    if is_refactor:names=(names-{'scan'})|{'scan_with_artifacts','expected','item_output'}
    changes=[]
    for m in re.finditer(r'\bfn\s+(\w+)\b[^;{]*\{',s):
        name=m.group(1)
        if name not in names:continue
        if name not in {'scan_with_artifacts','expected','item_output'} and not any(x['file']=='crates/engine/src/'+file and x['name']==name for x in impacts):raise ValueError('missing impact '+name)
        label='work_scan' if name in {'scan','scan_with_artifacts'} else file+':'+name
        if file=='work_state/scope.rs':label+=':'+str(s[:m.start()].count('\n')+1)
        changes.append((m.end(),f'\n    let _research_span = crate::research_profile::Span::enter("{label}");\n'))
        inserts.append({'file':file,'name':name,'label':label})
    for pos,code in reversed(changes):s=s[:pos]+code+s[pos:]
    if file=='work_state/mod.rs':
        s=replace(s,'    Ok(bytes)\n}', '    crate::research_profile::count("work_encoded_bytes", bytes.len() as u64);\n    Ok(bytes)\n}')
    if file=='definition_identity.rs':
        s=replace(s,'    serde_json_canonicalizer::to_vec(value).map_err(|_| {','    let result = serde_json_canonicalizer::to_vec(value).map_err(|_| {')
        s=replace(s,'            "RFC 8785 canonicalization failed",\n        )\n    })','            "RFC 8785 canonicalization failed",\n        )\n    });\n    if let Ok(bytes) = &result { crate::research_profile::count("canonical_encoded_bytes", bytes.len() as u64); }\n    result')
    if file=='repository_durability.rs':
        s=replace(s,'        platform::list_directory(self, &relative, entry_limit)','        let result = platform::list_directory(self, &relative, entry_limit);\n        if let Ok(entries) = &result { crate::research_profile::count_path("list", relative.as_str(), entries.len() as u64); }\n        result')
        s=replace(s,'        Ok(RepositoryReadV1::Found(bytes))','        crate::research_profile::count_path("read", relative.as_str(), bytes.len() as u64);\n        Ok(RepositoryReadV1::Found(bytes))')
        s=replace(s,'        file.lock()\n            .map_err(|_| lock_unavailable("exclusive repository lock acquisition failed"))?;','        let _research_wait = crate::research_profile::Span::enter("lock_wait");\n        file.lock()\n            .map_err(|_| lock_unavailable("exclusive repository lock acquisition failed"))?;\n        drop(_research_wait);')
        s=replace(s,'        Ok(RepositoryExclusiveLockV1 {','        crate::research_profile::lock_acquired(&file, relative.as_str());\n        Ok(RepositoryExclusiveLockV1 {')
        s=replace(s,'            let _ = file.unlock();','            let _ = file.unlock();\n            crate::research_profile::lock_released(&file);')
        if not any(x['file']=='crates/engine/src/repository_durability.rs' and x['name']=='release' for x in impacts):raise ValueError('missing explicit release impact')
        s=replace(s,'        file.unlock()\n            .map_err(|_| uncertain("exclusive repository lock release was uncertain"))','        let result = file.unlock()\n            .map_err(|_| uncertain("exclusive repository lock release was uncertain"));\n        crate::research_profile::lock_released(&file);\n        result')
    if is_refactor:
        s=replace(s,'        if let Some(value) = self.records.get(bytes) {','        if let Some(value) = self.records.get(bytes) {\n            crate::research_profile::count("artifact_hit_intent_bytes", bytes.len() as u64);')
        s=replace(s,'        let intent: Intent = decode(bytes)?;','        crate::research_profile::count("artifact_miss_intent_bytes", bytes.len() as u64);\n        let intent: Intent = decode(bytes)?;')
        s=replace(s,'        self.records.insert(bytes.to_vec(), Rc::clone(&value));','        self.records.insert(bytes.to_vec(), Rc::clone(&value));\n        crate::research_profile::gauge("artifact_entries_peak", self.records.len() as u64);\n        crate::research_profile::gauge("artifact_buffer_bytes_peak", self.records.iter().map(|(key, v)| key.len() + v.decision.len() + v.snapshot.len() + v.terminal.len() + v.publication.len() + v.noncommit.len()).sum::<usize>() as u64);')
        s=replace(s,'        if !proofs.contains(&proof_bytes) {','        if proofs.contains(&proof_bytes) { crate::research_profile::count("pure_historical_proof_hit_bytes", proof_bytes.len() as u64); }\n        if !proofs.contains(&proof_bytes) {\n            crate::research_profile::count("pure_historical_proof_miss_bytes", proof_bytes.len() as u64);')
    path.write_text(s,encoding='utf-8');provenance.append({'file':file,'before_sha256':before,'after_sha256':hashlib.sha256(path.read_bytes()).hexdigest()})
shutil.copyfile(HERE/'metrics.rs',target/'crates/engine/src/research_profile.rs')
lib=target/'crates/engine/src/lib.rs';lib.write_text(lib.read_text(encoding='utf-8')+'\n#[allow(unsafe_code)]\npub mod research_profile;\n',encoding='utf-8')
(target/'research-instrumentation.json').write_text(json.dumps({'inserts':inserts,'sources':provenance},indent=2)+'\n')
print(json.dumps({'target':str(target),'span_functions':len(inserts),'source_files':len(provenance)}))
