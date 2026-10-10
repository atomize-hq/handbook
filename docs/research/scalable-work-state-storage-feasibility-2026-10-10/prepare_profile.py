"""Patch only a fresh, external git-archive copy; never the product checkout."""
import argparse
import hashlib
import json
import pathlib
import re
import shutil

def prepare_profile():
    p = argparse.ArgumentParser()
    p.add_argument('copy', type=pathlib.Path)
    a = p.parse_args()
    here = pathlib.Path(__file__).resolve().parent
    repo = here.parents[2]
    target = a.copy.resolve()
    if target == repo or repo in target.parents or not (target / 'Cargo.toml').is_file():
        raise ValueError('requires an existing source archive outside checkout')
    files = list((target / 'crates/engine/src/work_state').glob('*.rs'))
    files += [target / ('crates/engine/src/' + f) for f in (
        'repository_durability.rs', 'definition_identity.rs',
        'local_authority/work_registration.rs', 'local_authority/local_check.rs',
        'local_authority.rs')]
    selected = {
        'scan','parsed','inventory','manifest','validate','open','encode','decode','fp',
        'output','decision','terminal','publication','noncommit','validate_authority_history',
        'canonical_json_bytes','parse_schema_json','from_bytes','fingerprint',
        'validate_historical','validate_shape','capture_proof','validate_current',
        'with_work_registration_current','with_work_registration_establishment',
        'work_registration_current','require_stable_path_epoch','retain_selected_tree',
        'retain_exact_files_checkpoint_v1','retain_work_history_tree_guard_v1',
        'read_regular_bounded','list_directory_bounded','regular_file_snapshot',
        'retained_file_fingerprint','open_regular_optional','pin_windows_parent',
        'open_retained_directory','list_directory','open_windows_directory_guard',
        'windows_handle_info','lock_existing','acquire_existing_exclusive_lock_v1',
        'write_new_durable','rename_noreplace_durable','replace_exact_durable',
        'reserve_intent','write_exact','optional','required'}
    for file in files:
        if not file.is_file() or 'crate::research_profile::Span::enter' in file.read_text(encoding='utf-8'):
            raise ValueError('requires a fresh uninstrumented archive: ' + str(file))
    manifest = []
    for file in files:
        source = file.read_text(encoding='utf-8')
        relative = file.relative_to(target).as_posix()
        insertions = []
        for m in re.finditer(r'\bfn\s+(\w+)\b[^;{]*\{', source):
            if m.group(1) not in selected:
                continue
            line = source[:m.start()].count('\n') + 1
            label = f'{relative}:{line}:{m.group(1)}'
            insertions.append((m.end(), f'\n    let _research_span = crate::research_profile::Span::enter("{label}");\n'))
            manifest.append({'file':relative,'line':line,'function':m.group(1)})
        for pos, code in reversed(insertions):
            source = source[:pos] + code + source[pos:]
        file.write_text(source, encoding='utf-8')
    shutil.copyfile(here / 'profile_metrics.rs', target / 'crates/engine/src/research_profile.rs')
    lib = target / 'crates/engine/src/lib.rs'
    lib.write_text(lib.read_text(encoding='utf-8') + '\n#[allow(unsafe_code)]\npub mod research_profile;\n', encoding='utf-8')
    harness = target / 'docs/research/scalable-work-state-storage-feasibility-2026-10-10/native'
    harness.mkdir(parents=True)
    old = repo / 'docs/research/work-state-capacity-and-performance-2026-10-10/harness'
    shutil.copyfile(old / 'Cargo.toml', harness / 'Cargo.toml')
    shutil.copyfile(old / 'Cargo.lock', harness / 'Cargo.lock')
    (harness / 'src').mkdir()
    shutil.copyfile(here / 'native_main.rs', harness / 'src/main.rs')
    (target / 'profile-insertions.json').write_text(json.dumps(manifest, indent=2)+'\n')
    print(json.dumps({'copy':str(target),'insertions':len(manifest),
        'recipe_sha256':hashlib.sha256(pathlib.Path(__file__).read_bytes()).hexdigest()}))

if __name__ == '__main__':
    prepare_profile()
