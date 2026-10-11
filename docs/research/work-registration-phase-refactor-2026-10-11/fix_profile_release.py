"""Correct only the observer for existing explicit native unlock in isolated sources."""
import hashlib,json,pathlib,shutil,sys
HERE=pathlib.Path(__file__).resolve().parent;PRIMARY=HERE.parents[2]
for value in sys.argv[1:]:
    source=pathlib.Path(value).resolve()
    assert source!=PRIMARY and PRIMARY not in source.parents
    path=source/'crates/engine/src/repository_durability.rs'
    old='        file.unlock()\n            .map_err(|_| uncertain("exclusive repository lock release was uncertain"))'
    new='        let result = file.unlock()\n            .map_err(|_| uncertain("exclusive repository lock release was uncertain"));\n        crate::research_profile::lock_released(&file);\n        result'
    text=path.read_text(encoding='utf-8');assert text.count(old)==1
    before=hashlib.sha256(path.read_bytes()).hexdigest()
    path.write_text(text.replace(old,new),encoding='utf-8')
    shutil.copyfile(HERE/'metrics.rs',source/'crates/engine/src/research_profile.rs')
    print(json.dumps({'isolated_source':str(source),'before_sha256':before,'after_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'original_unlock_error_mapping_retained':True}))
