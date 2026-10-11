"""Build driver against one isolated source checkout, never alter product source."""
import pathlib, shutil, subprocess, sys
here=pathlib.Path(__file__).resolve().parent
source=pathlib.Path(sys.argv[1]).resolve()
driver=source/'docs/research/work-registration-phase-refactor-2026-10-11/native'
(driver/'src').mkdir(parents=True,exist_ok=True)
previous=here.parent/'work-state-capacity-and-performance-2026-10-10/harness'
manifest=(previous/'Cargo.toml').read_text().replace('[workspace]','[workspace]\n\n[features]\nprofile = []').replace('serde_json = "1"','serde_json = "1"\nserde = "1"')
(driver/'Cargo.toml').write_text(manifest)
lock=(previous/'Cargo.lock').read_text()
marker='name = "work-state-capacity-research"'
pos=lock.index(marker)
lock=lock[:pos]+lock[pos:].replace(' "serde_json",',' "serde",\n "serde_json",',1)
(driver/'Cargo.lock').write_text(lock)
shutil.copyfile(here/'native_main.rs',driver/'src/main.rs')
print(driver/'Cargo.toml')
