"""Prepare only an isolated, baseline managed worktree. Research, not a migration."""
import json, pathlib, re, shutil, subprocess, sys
here = pathlib.Path(__file__).resolve().parent
primary = here.parents[2]
target = pathlib.Path(sys.argv[1]).resolve()
if target == primary or primary in target.parents:
    raise ValueError('experiment requires isolated checkout')
if subprocess.check_output(['git','rev-parse','HEAD'],cwd=target,text=True).strip() != '8013948ebd3fbef6725f5d72f3d29120f7617009':
    raise ValueError('wrong experimental baseline')
store = target/'crates/engine/src/work_state/store.rs'
s = store.read_text(encoding='utf-8')
assert '_research_owner_bytes' not in s
s = s.replace('    pub owner: Owner,\n}', '    pub owner: Owner,\n    _research_owner_bytes: Option<(Vec<u8>, Vec<u8>)>,\n}',1)
s = s.replace('required(self.root, OWNER)? != encode(&self.owner)?','required(self.root, OWNER)? != self._research_owner_bytes.as_ref().map(|v| v.0.clone()).map_or_else(|| encode(&self.owner), Ok)?',1)
s = s.replace('required(self.root, ANCHOR)? != encode(&self.owner)?','required(self.root, ANCHOR)? != self._research_owner_bytes.as_ref().map(|v| v.0.clone()).map_or_else(|| encode(&self.owner), Ok)?',1)
s = s.replace('required(self.root, BOOT)? != encode(&self.owner.bootstrap)?','required(self.root, BOOT)? != self._research_owner_bytes.as_ref().map(|v| v.1.clone()).map_or_else(|| encode(&self.owner.bootstrap), Ok)?',1)
s = s.replace('    let result = WorkLock { root, lock, owner };', '''    let mode = std::env::var("HCM_RESEARCH_VARIANT").unwrap_or_default();
    let _research_owner_bytes = if mode == "owner" || mode.starts_with("combined") {
        Some((encode(&owner)?, encode(&owner.bootstrap)?))
    } else { None };
    let result = WorkLock { root, lock, owner, _research_owner_bytes };''',1)
# This deliberately removes ancestor inventory observations. Keep separate from
# the semantics-preserving combined control and probe its changed refusal behavior.
s = s.replace('''    optional(root, p)?.ok_or_else(|| {''', '''    let mode = std::env::var("HCM_RESEARCH_VARIANT").unwrap_or_default();
    let observed = if mode.contains("direct") {
        match root.read_regular_bounded(p, MAX).map_err(fs_error)? {
            RepositoryReadV1::Absent => None,
            RepositoryReadV1::Found(v) => Some(v),
        }
    } else { optional(root, p)? };
    observed.ok_or_else(|| {''',1)
store.write_text(s,encoding='utf-8')
dur = target/'crates/engine/src/repository_durability.rs'
s = dur.read_text(encoding='utf-8')
assert '_research_parents' not in s
s = s.replace('    _ancestor_guards: Vec<File>,','    _ancestor_guards: Vec<File>,\n    #[cfg(windows)]\n    _research_parents: std::sync::Mutex<std::collections::BTreeMap<PathBuf, File>>,',1)
s = s.replace('                _ancestor_guards: guards,','                _ancestor_guards: guards,\n                _research_parents: std::sync::Mutex::new(std::collections::BTreeMap::new()),',1)
old = '            guards.push(open_windows_directory(&current)?);'
start = s.index('    fn pin_windows_parent(')
pos = s.index(old,start)
s = s[:pos] + '''            let mode = std::env::var("HCM_RESEARCH_VARIANT").unwrap_or_default();
            let relative_parent = current.strip_prefix(&self.root_path).map_err(|_| unsafe_fs("research parent outside root"))?;
            let eligible = matches!(relative_parent.to_str(), Some(".handbook") | Some(".handbook\\\\work-state") | Some(".handbook\\\\work-state\\\\v1"));
            if eligible && (mode == "binding" || mode.starts_with("combined")) {
                let mut parents = self._research_parents.lock().map_err(|_| unsafe_fs("research parent cache poisoned"))?;
                if !parents.contains_key(&current) {
                    parents.insert(current.clone(), open_windows_directory(&current)?);
                }
                let retained = parents.get(&current).unwrap();
                directory_identity(retained)?; // fresh directory/reparse/delete-pending check
                guards.push(retained.try_clone().map_err(|_| unsafe_fs("research held-parent duplicate failed"))?);
            } else {
                guards.push(open_windows_directory(&current)?);
            }''' + s[pos+len(old):]
# Attribute successful returned listings/reads by path. No result is reused.
s = s.replace('        platform::list_directory(self, &relative, entry_limit)', '''        let result = platform::list_directory(self, &relative, entry_limit);
        if let Ok(entries) = &result { crate::research_profile::count_path("list", relative.as_str(), entries.len() as u64); }
        result''',1)
s = s.replace('        Ok(RepositoryReadV1::Found(bytes))', '''        crate::research_profile::count_path("read", relative.as_str(), bytes.len() as u64);
        Ok(RepositoryReadV1::Found(bytes))''',1)
dur.write_text(s,encoding='utf-8')
tx=target/'crates/engine/src/work_state/transaction.rs'
s=tx.read_text(encoding='utf-8')
s=s.replace('    // Both an acceptance candidate and a visible request for the same key are','    let research_relations = crate::research_profile::Span::enter("pairwise_and_lineage_checks");\n    // Both an acceptance candidate and a visible request for the same key are',1)
s=s.replace('    validate_authority_history(&rows)?;\n    tree.validate()', '    validate_authority_history(&rows)?;\n    drop(research_relations);\n    tree.validate()',1)
tx.write_text(s,encoding='utf-8')
# Small set of causal spans, reused from prior evidence. Instrumented and disabled
# controls use the same binary. Raw calls retain inclusive and exclusive timing.
selected = {
 'scan','scan_reusing','parsed','parsed_reusing','execute','validate_authority_history',
 'inventory','manifest','canonical_json_bytes','read_regular_bounded','list_directory_bounded',
 'pin_windows_parent','windows_handle_info','open_windows_directory_guard',
 'with_work_registration_establishment','validate_current','required'}
files = [target/('crates/engine/src/'+p) for p in (
 'work_state/store.rs','work_state/transaction.rs','definition_identity.rs',
 'repository_durability.rs','local_authority/work_registration.rs')]
insertions=[]
for file in files:
    s=file.read_text(encoding='utf-8')
    edits=[]
    for m in re.finditer(r'\bfn\s+(\w+)\b[^;{]*\{',s):
        if m.group(1) in selected:
            label=f'{file.relative_to(target).as_posix()}:{s[:m.start()].count(chr(10))+1}:{m.group(1)}'
            edits.append((m.end(),f'\n    let _research_span = crate::research_profile::Span::enter("{label}");\n'))
            insertions.append(label)
    for pos,code in reversed(edits): s=s[:pos]+code+s[pos:]
    file.write_text(s,encoding='utf-8')
shutil.copyfile(here/'metrics.rs',target/'crates/engine/src/research_profile.rs')
lib=target/'crates/engine/src/lib.rs'
lib.write_text(lib.read_text(encoding='utf-8')+'\n#[allow(unsafe_code)]\npub mod research_profile;\n',encoding='utf-8')
native=target/'docs/research/work-state-native-optimization-2026-10-10/native'
(native/'src').mkdir(parents=True,exist_ok=True)
old=primary/'docs/research/work-state-capacity-and-performance-2026-10-10/harness'
for name in ('Cargo.toml','Cargo.lock'): shutil.copyfile(old/name,native/name)
shutil.copyfile(here/'native_main.rs',native/'src/main.rs')
(target/'research-insertions.json').write_text(json.dumps(insertions,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'target':str(target),'spans':len(insertions)}))
