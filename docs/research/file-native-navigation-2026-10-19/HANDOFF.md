# Navigation experiment: focused native handoff

Standalone research only. The coordinator owns integration and platform lanes.
Historical projection profiles 1/2 retain their original rendering/replay rules.
Profile 3 measures fact dirtiness with complete YAML output; profile 4 adds
binary SHA-256 stable-ID routing and grouped YAML. `EXP_NAVIGATION=2/3/4` selects
new experimental targets only. Replay uses the prepared immutable profile.

From `docs/research/file-native-incremental-engine-experiment-2026-10-18`:

```text
cargo test --locked --offline --workspace -j2 --target-dir <owned-build>
cargo build --locked --offline --release -j2 --target-dir <owned-build>
python ../file-native-navigation-2026-10-19/focused.py --exe <owned-build>/release/file-native-incremental --base <fresh-owned-case-root> --capsule
```

Use `.exe` on Windows. Python 3.11.9 or newer is sufficient; no Python package
installation is needed. Include the engine `src/`, Cargo files, consumers,
`verify_store.py`, this whole handoff directory and the recovery directory's
Python files in a compact source transfer. The included legacy fixture archive
contains unmodified ordinary/packet A/B profile-1 candidates copied from the
existing recovery experiment's protected fixtures; its manifest binding is
reported separately. It is test input, never a new accepted history.

`focused.py` checks A/B payload and batched payload dirtiness, byte-based local
split/merge, typed ordered duplicate adjacency, adjacency collapse, target path
changes, deletion, external edits/interruption, profile-1/2 frozen candidates,
legacy packet candidates, genuinely pending A work, and profile-4 upgrades.
It also runs the actual recovery `capsule.py` with the independent profile-4
oracle and bounded external process sampling. Ordinary children have 120-second
limits; the entire focused capsule has an original 1,800-second budget. Point
reads use 256 MiB RSS limits, other children 2 GiB. Mac sampling uses `ps`, Linux
uses `/proc`, Windows uses `GetProcessMemoryInfo`. Missing samples are null.

Profile identity is a mandatory marker on the existing provenance FileSpec in
the immutable projection map. It is preserved by both packet-writing paths and
checkpoint carry-forward; it does not add an ordinary file. Optional fields are
omitted from all legacy FileSpec serialization.

Supplementation is omitted only for the bounded `self-describing-json` dialect:
strict duplicate-free closed JSON containing the exact ID, title, kind and
ordered typed/local/external relationships. Its ordinary tree path supplies
location. The absence test converts a small real fixture, prepares/replays a
packet and checkpoints it with zero navigation files. Loss/restoration of source
completeness changes supplementation even on payload-only edits. Other current
synthetic fixture kinds still require supplementation. This is no general source
parser, Markdown transformation, or new Handbook artifact family.

Groups target 512 records / 256 KiB. Local updates inspect the affected group
and potential leaf siblings; they do not scan the complete header range. The
source graph's reverse-reference validation still runs where required. Large
adjacency uses ordinal-preserving parts of at most 256 relationships / 256 KiB;
an existing valid single relationship larger than that byte geometry is an
explicit counted exception up to the existing object bound plus framing. The
document uses a directory/count reference, so large adjacency does not require
an unbounded document record. Whole adjacency inspection can remain expensive;
the counters and high-degree measurements must remain attached to this result.

`oracle.py` decodes complete checked C1 indexes independently, reconstructs every
graph index, parses all emitted YAML, compares exact ordered graph facts and
authored bytes, and checks A-current/B-selected-once materializations. It imports
no engine renderer or resolver. B construction and verification have separate
timers. Range counters distinguish returned rows, examined leaf pairs and page
visits, including namespace/reverse/journal queries. Generated parts/bytes are
not the same quantity as changed output files or physical writes.

The preexisting GitNexus graph cannot resolve this worktree; retained impact and
detect-changes failures remain UNKNOWN, with manual call tracing substituted.
No power-loss qualification, N100k run, production adoption, or packing is implied.

## Closed Windows measurement handoff

The final engine is frozen at `4bde156323812ef18c0dea6798cfc2108aaad933`.
Its Windows binary SHA-256 is
`d4b7c495abfc029e8fabd6fcfc75501c5de04ce72dd38847a69bc8824022303f`.
The ordinary Windows matrix and full focused suite used core
`6e66a7523612b80386cf9cc2b96d5341579b54fd`, executable SHA-256
`6336944d13967dfce04e4cc40b3459cdac4718b223dda147dcd202598ad50558`.
The final core only omits inherited Markdown guidance for new profile4 genesis;
eight checks on the final binary verify this change. Do not relabel the matrix.

Read `report.md`, `results/qualification-summary.json`, and the indexed raw
archive for coverage and censors. The 195 primary operation rows check complete
views; 24 extra deferred-view rows are separate controls. N10k A4 initialization
was censored then resumed from its authentic selected prefix, retaining the
original admission epoch. Full A10k profiles3/2 are authenticated restore-backed
comparators with an untimed normalization publication. N10k B initialization
was censored before selection and was never restarted. B10k profiles2/3 and all
B10k ordinary operations are unperformed.

Late harness options support authenticated operational restore, original-epoch
continuation, payload-only seeding at the existing creation barrier and owned
byte/file guards. Native focused defaults do not use these matrix-only options.
Do not reuse an existing successful base or fabricate a selector/control file.
The amended full-control launcher retains successful fixtures; its completed
original runs used the stricter 1GiB/100000-file guard. Profile3's fixture had
already been removed by the old launcher before the retention amendment; its
raw results and original input capture survive. Profile2's fixture is retained.
The old suspended launcher was stopped after child completion to preserve it.

The coordinator owns Linux/Mac/integration and P. Linux final4bde full qualification
was reported passed; Mac was incomplete at this worker's last update. This
handoff does not authorize or declare a P-gate pass. All source/tool bindings,
current census, residue and rejected-cleanup details are explicit in the report.
