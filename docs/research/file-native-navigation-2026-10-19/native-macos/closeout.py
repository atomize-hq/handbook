import os,json,hashlib,shutil,gzip,datetime,subprocess
from pathlib import Path
O=Path(__file__).resolve().parent;R=O/'runtime';V=O/'evidence';W=O.parents[3];G=Path('/Users/spensermcconnell/__Active_Code/atomize-hq/handbook/.git')
def census(root):
 n=b=0
 for d,ds,fs in os.walk(root,followlinks=False):
  for f in fs:
   s=os.lstat(os.path.join(d,f));n+=1;b+=s.st_size
 return dict(root=str(root),files=n,logical_bytes=b)
for name in ['events.json','cases.json','capsule-resources.json','recovery-capsule/events.json']:
 src=R/'focused'/name
 if src.exists():
  target=V/('raw-'+name.replace('/','-')+'.gz')
  with target.open('wb') as f:
   with gzip.GzipFile(filename='',mode='wb',fileobj=f,mtime=0) as z:z.write(src.read_bytes())
commands=json.loads((V/'commands.json').read_bytes());cases=json.loads((R/'focused/cases.json').read_bytes());events=json.loads((R/'focused/events.json').read_bytes());recovery=json.loads((R/'focused/recovery-capsule/events.json').read_bytes());resources=json.loads((R/'focused/capsule-resources.json').read_bytes())
pins=json.loads((V/'source-pins.json').read_bytes());more={}
for pattern in ['docs/research/file-native-recovery-reclamation-2026-10-19/*.py','docs/research/file-native-incremental-engine-experiment-2026-10-18/consumers/*/src/*.rs','docs/research/file-native-incremental-engine-experiment-2026-10-18/consumers/*/Cargo.toml','docs/research/file-native-incremental-engine-experiment-2026-10-18/verify_store.py']:
 for file in W.glob(pattern):more[str(file.relative_to(W))]=hashlib.sha256(file.read_bytes()).hexdigest()
(V/'additional-source-pins.json').write_text(json.dumps(more,indent=2))
# Bound exact final engine bytes independently against the pinned git object.
match={name:hashlib.sha256(subprocess.check_output(['git','show','4bde156:'+name],cwd=W)).hexdigest()==digest for name,digest in pins['files'].items()}
assert all(match.values())
(V/'source-verification.json').write_text(json.dumps(match,indent=2))
rows=[census(x) for x in [W,G,Path('/private/tmp/file-native-q-macos-20261019'),Path('/Users/spensermcconnell/.codex/worktrees/file-native-q-macos-20261019/handbook'),Path('/Users/spensermcconnell/.npm/_npx')]]
resource=dict(timestamp=datetime.datetime.now(datetime.timezone.utc).isoformat(),roots=rows,owned_plus_shared_git_growth_bytes=rows[0]['logical_bytes']+max(0,rows[1]['logical_bytes']-185803386),owned_plus_shared_git_growth_files=rows[0]['files']+max(0,rows[1]['files']-5264),allowance_bytes=805306368,allowance_files=75000,free_bytes=shutil.disk_usage(W).free,continuous_disk_peak='missing',shared_git_growth_includes_other_chat_writes_conservatively=True,prefetch_full_worktree_bytes=258508273,prefetch_full_worktree_files=4536)
assert resource['owned_plus_shared_git_growth_bytes']<805306368 and resource['owned_plus_shared_git_growth_files']<75000 and resource['free_bytes']>20*1024**3
(V/'closeout-census.json').write_text(json.dumps(resource,indent=2))
summary=dict(status='CENSORED-original-functional-cutoff',source=pins['source'],exe=json.loads((V/'exe.json').read_bytes()),focused_cases=cases,focused_events=len(events),recovery_recorded_events=len(recovery),recovery_last_action=recovery[-1]['input'],recovery_completed_case_count='not durably recorded before SIGKILL',focused_summary_exists=(R/'focused/summary.json').exists(),recovery_summary_exists=(R/'focused/recovery-capsule/summary.json').exists(),sampled_engine_rss_max_bytes=max(x['sampled_peak_rss'] or 0 for x in events+resources),engine_missing_rss_samples=sum(x['sampled_peak_rss'] is None for x in events+resources),aggregate_sampled_peak_rss_bytes=max(x['aggregate_sampled_peak_rss_bytes'] or 0 for x in commands),cutoff='2026-10-19T23:44:38Z',original_closeout_deadline='2026-10-19T23:54:38Z',overall_N_deadline='2026-10-20T00:38:51Z',artifact_closeout_overrun=True,graph='UNKNOWN: worktree repository not registered; detect_changes failed; no installation attempted',supplement='no recorded functional coverage; outer original-cutoff censor',resource=resource)
(V/'qualification.json').write_text(json.dumps(summary,indent=2))
(V/'gitnexus.txt').write_text('detect_changes(repo=/Users/spensermcconnell/.codex/worktrees/file-native-n-macos-20261019/handbook,scope=all) failed: Repository not found. Graph UNKNOWN. No bootstrap attempted. Manual scope: only new native-macos report/harness/evidence; all 16 core Rust/Cargo hashes unchanged against 4bde156.\n')
(V/'platform-attempts.json').write_text(json.dumps(dict(initial_sparse='Operation not permitted locking config.worktree; approved retry succeeded',initial_branch='Operation not permitted creating branch lock; approved retry succeeded',initial_evidence_mkdir='PermissionError Operation not permitted; approved retry succeeded',supplement_approval='requested before original cutoff; completion observed at 2026-10-20T01:36:14Z after original and overall deadlines; no new functional tests authorized or run afterwards',no_6e66_tests=True),indent=2))
report=f"""# N native macOS qualification: censored original allocation

Final source: `4bde156323812ef18c0dea6798cfc2108aaad933`. Engine source is unchanged. All 14 Rust source files plus Cargo.toml/Cargo.lock match the pinned Git objects; exact hashes and additional consumer/recovery harness bindings are in evidence. Native debug=0 executable SHA-256: `1de6432fc24922f10a6e4a1eddb95e07f1e236cf4435723e4fd2320e5dbdc786`.

Fixed start 2026-10-19 22:54:38 UTC; testing cutoff 23:44:38 UTC; original closeout deadline 23:54:38 UTC. Overall N deadline was 2026-10-20 00:38:51 UTC. Platform approval waits consumed most of the allocation and later carried artifact closeout beyond both deadlines. No clock was reset. No remainder-stage GO arrived; no functional work occurred after cutoff.

Locked offline workspace tests passed (2 tests), and native build passed with -j2, debuginfo 0 and incremental disabled. Full focused invocation began 23:43:03.005118 UTC and was SIGKILLed after 95.000179 seconds at the original cutoff. Its entire process group was killed. All 39 focused oracle checks and 153 engine events were durably recorded. The adapted recovery capsule recorded 83 operations, ending during main-B-128, action dump. Neither completion summary exists; full recovery qualification is INCOMPLETE. The capsule records cases only in its finalizer, so an exact completed-case count was not durably recorded and is not invented here.

Achieved focused checks include native A/B grouped output, payload/batch/label dirtiness, byte split/merge, ordered typed duplicate adjacency, collapse, target location, deletion, A external-edit preservation/interruption, no-navigation self-describing sources, packet/checkpoint/replay, completeness loss/restoration, mixed-source whitespace/key-order stability, frozen profile-2 candidates/pending work and packaged profile-1 ordinary/packet upgrades. Each completed check ran the full independent graph/index/YAML/exact-authored-byte oracle. See qualification.json for all 39 names and raw events for actual metrics.

Separately requested supplements were delayed by approval and censored at the already expired original cutoff. They have no recorded coverage. Missing: full recovery completion, growing-edit/no-sibling-scan supplement, ordinary Bash discovery/copy, explicit historical profile-3 README preservation and dedicated A/B genesis README supplement. The focused candidate code does check profile-4 README absence, but no broader README preservation claim is made. No performance matrix ran.

Sampled aggregate peak across build/test/checker descendants: {summary['aggregate_sampled_peak_rss_bytes']} bytes, below 3 GiB. Recorded engine peak: {summary['sampled_engine_rss_max_bytes']} bytes; {summary['engine_missing_rss_samples']} engine samples were missing (retained as null). Core Mac /proc peak is unavailable/null. External ps sampling was used; /usr/bin/time was not run. The focused supervisor capped the whole run at the earlier original cutoff; per-child harness limits were 120 seconds and 256 MiB point / 2 GiB other. The unused supplement supervisor did not provide independent RSS receipts; it supplies no qualifying results.

Closeout observed total owned checkout including source/build/fixtures/raw evidence plus conservative shared Git growth: {resource['owned_plus_shared_git_growth_bytes']} bytes / {resource['owned_plus_shared_git_growth_files']} files, within 768 MiB / 75,000 files. Free space: {resource['free_bytes']} bytes, above 20 GiB. Full initial checkout was 258,508,273 bytes / 4,536 files before sparse checkout. Continuous disk peak is missing; no zero or maximum claim replaces it. Baseline and closeout census explicitly retain Q roots and existing npm tool-cache scope. Shared Git growth is conservatively charged even if another chat caused it. Prior Q artifacts were never edited or removed.

Graph status UNKNOWN: existing detect_changes tool rejected the unregistered worktree. No package-manager/bootstrap retry. Manual scope tracing finds only new native-macos harness/report/evidence; all core source bytes remain pinned. Git diff review is the scope authority for this artifact-only return. This is a censored partial result, not admission of P or production adoption.

Raw gzip event files preserve original bytes and all recorded expected failures/interruption exits. The unmodified fixture tree and native build remain under ignored runtime/ in this attached worktree; no build output or dataset is included in Git. Original permission failures, command times, censors and absent summaries remain explicit in evidence.
"""
(O/'REPORT.md').write_text(report)
manifest={str(p.relative_to(O)):dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in O.rglob('*') if p.is_file() and 'runtime' not in p.relative_to(O).parts and '__pycache__' not in p.relative_to(O).parts and p.name!='manifest.json'}
(V/'manifest.json').write_text(json.dumps(manifest,indent=2));print(json.dumps(resource,indent=2));print('raw focused',len(events),'cases',len(cases),'recovery',len(recovery))
