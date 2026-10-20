"""Artifact-only closeout; no engine or qualification calls."""
import json,gzip,hashlib,datetime,subprocess,shutil
from pathlib import Path
import followup as f
O=f.O;V=f.V;N=f.N;R=f.R
setup='First follow-up helper creation command failed before execution: Python SyntaxError: invalid syntax. Perhaps you forgot a comma? Nested triple-quoted placeholder closed the outer script literal. Followup.py was absent, and the attempted invocation returned file not found. No functional test or fixture was created. The corrected command and all functional execution share the same original follow-up dispatch/cutoffs.\n'
(V/'setup-attempt.txt').write_text(setup)
# Preserve raw recovery/supplement event bytes and ordinary shell listing/search/copy receipts.
raw=V/'raw';raw.mkdir(exist_ok=True)
paths=[R/'recovery-capsule/events.json',R/'recovery-capsule/summary.json',R/'supplement/events.json']
for d in ['shell-A','shell-B','shell-B-parser-retry']:
 paths+=list((R/'supplement'/d).glob('*'))
for p in paths:
 if p.is_file():
  target=raw/(str(p.relative_to(R)).replace('/','--')+'.gz')
  with target.open('wb') as out:
   with gzip.GzipFile(filename='',mode='wb',fileobj=out,mtime=0) as z:z.write(p.read_bytes())
for mode in ['A','B']:
 for profile in [1,2,3]:
  (raw/f'README-{mode}-profile{profile}.raw').write_bytes((R/f'supplement/readme-{mode}-{profile}'/('README.md' if profile==1 else 'disposable/store/README.md')).read_bytes())
recovery=json.loads((V/'recovery-result.json').read_bytes());supplement=json.loads((V/'supplement-cases.json').read_bytes());retry=json.loads((V/'bash-B-parser-retry-result.json').read_bytes());supervisor=json.loads((V/'supervisor.json').read_bytes());retryresource=json.loads((V/'bash-B-parser-retry-command.json').read_bytes());before=json.loads((V/'baseline.json').read_bytes());after=f.bindings();assert after==json.loads((V/'post-bindings.json').read_bytes());assert before['bindings']==after
assert len(recovery['cases'])==24 and recovery['events']==282 and len(supplement)==13 and retry['status']=='passed';assert supervisor['exit']==1 and supervisor['censor'] is None and not supervisor['remaining_process_group_after_cleanup'];assert retryresource['exit']==0 and not retryresource['remaining_process_group']
correct_end=datetime.datetime.fromisoformat(retry['time']).timestamp();cutoff=datetime.datetime.fromisoformat('2026-10-20T02:19:38+00:00').timestamp();assert correct_end<cutoff
# Verify each of the 29 original committed artifacts against its actual e6d1eda blob.
original={name:hashlib.sha256(subprocess.check_output(['git','show','e6d1eda80564ffee1092e353f6c50651e26ab8e6:docs/research/file-native-navigation-2026-10-19/native-macos/'+name],cwd=f.W)).hexdigest()==f.sha(N/name) for name in before['original_artifact_hashes']};assert len(original)==29 and all(original.values())
f.save('original-e6d1-preservation.json',original);f.save('final-bindings.json',after)
resources=json.loads((V/'child-resources.json').read_bytes());rss=json.loads((V/'rss-samples.json').read_bytes());disk=json.loads((V/'runtime-census-samples.json').read_bytes());snapshot=f.snapshot();runtime=f.census(R)
assert snapshot['owned_bytes']<805306368 and snapshot['owned_files']<75000 and snapshot['free_bytes']>20*1024**3 and runtime['logical_bytes']<268435456 and runtime['files']<25000
f.save('final-census.json',dict(**snapshot,followup_runtime=runtime,maximum_observed_runtime_bytes=max([runtime['logical_bytes']]+[r['logical_bytes'] for r in disk]),maximum_observed_runtime_files=max([runtime['files']]+[r['files'] for r in disk]),sampling='Runtime logical census every approximately two seconds; final includes all continuing source/build/old fixtures/new fixtures/returns plus conservative shared Git growth. Census snapshots do not measure every transient byte.'))
passed=supplement+[dict(retry,repaired_external_parser=True)]
summary=dict(status='REMAINING-QUALIFICATION-PASSED-AFTER-EXTERNAL-HARNESS-REPAIR',controlling_dispatch='2026-10-20T02:09:38Z',functional_cutoff='2026-10-20T02:19:38Z',closeout_deadline='2026-10-20T02:29:38Z',functional_finished=retry['time'],core_source='4bde156323812ef18c0dea6798cfc2108aaad933',exe_sha256=after['exe_sha256'],recovery_cases=24,recovery_events=282,recovery_seconds=recovery['seconds'],supplement_cases=passed,original_run_status='CENSORED unchanged',original_29_artifacts_unchanged=True,original_worker_exit=supervisor['exit'],external_harness_failure='Bash sed matched top-level and nested attempt; original failure preserved; B-only retry matched exact two-space top-level field on the same frozen fixture',aggregate_sampled_peak_rss_bytes=max(supervisor['aggregate_sampled_peak_rss_bytes'],(retryresource['shell_peak_rss_bytes'] or 0)+(retryresource['checker_peak_rss_bytes'] or 0)),checker_sampled_peak_rss_bytes=max([s['checker_rss_bytes'] or 0 for s in rss]+[retryresource['checker_peak_rss_bytes'] or 0]),ordinary_resource_rows=len(resources),ordinary_missing_rss_rows=sum(r['sampled_peak_rss_bytes'] is None for r in resources),ordinary_sampled_max_rss_bytes=max(r['sampled_peak_rss_bytes'] or 0 for r in resources),remaining_process_groups=[],new_dependencies=False,engine_edits=False,graph='UNKNOWN pending direct cached detect_changes attempt; no bootstrap',P='UNADMITTED pending parent review')
f.save('qualification.json',summary)
rows=[r for r in supplement if 'metrics' in r];table='\n'.join('| '+ ' | '.join(str(x) for x in [r['mode'],r['n'],r['groups'],r['affected_members'],r['metrics']['range_rows'].get('D',0),r['metrics']['range_calls'].get('E',0),r['metrics']['navigation_parts']])+' |' for r in rows)
report=f'''# Separate N macOS follow-up: remaining qualification passed

This human-authorized follow-up is separate from the original censored 60-minute native allocation and expired overall N run. Those clocks and the original `e6d1eda80564ffee1092e353f6c50651e26ab8e6` artifacts are unchanged. All 29 original committed files were rehashed against their exact Git blobs, including the 83-operation censored recovery prefix evidence. No original helper, report, or raw evidence was rewritten.

## Exact clock and source

Controlling dispatch: **2026-10-20 02:09:38 UTC**. Functional cutoff: **02:19:38 UTC**. Closeout deadline: **02:29:38 UTC**. The first clock read was 02:09:52; the initial helper used that 14-second-later value. Parent clarified the true dispatch; a separately retained guard clamped the existing worker group to 02:19:38. The B-only parser retry used the correct cutoff directly. All functional work finished **{retry['time']}**, before the controlling cutoff. Neither run was restarted or relabeled.

Source remained `4bde156323812ef18c0dea6798cfc2108aaad933`. All 14 Rust source files and Cargo.toml/Cargo.lock hashes match before/after. Native executable SHA-256 remained `{after['exe_sha256']}`. The previously passed locked tests/build and 39 focused checks were reused. No new build, engine edits or dependencies. Exact source, exe, fixture, oracle, capsule and external harness hashes are retained in baseline/final-bindings and manifest.

## Actual checks

The fresh actual recovery capsule completed **24 cases / 282 operations in {recovery['seconds']:.6f} seconds**, with the independent profile-4 graph/index/YAML/exact-byte oracle imported before capsule loading. This included operational capture/restore, accepted candidates, fences/pins/packets, namespace interruptions and correcting successors, external-byte preservation, checkpoint/accounting/retirement/repeated collection, half-prepared recovery, refusal of an impossible namespace, skipped A generations and retained B selected-once materializations. Source is the actual pinned recovery `capsule.py`; no state from the censored 83-operation run was resumed or forged.

All **14 supplemental cases** completed: historical profiles 1/2/3 README exact-byte preservation through upgrade and checkpoint in both A/B, new profile-4 A/B genesis README absence, growing edits at 128 and 1,024 documents in both modes, and ordinary native Bash listing/search/discovery/exact-copy in A/B. Profile 1 uses unmodified packaged historical stores because the current init switch cannot create profile 1. Every applicable current materialization was independently verified against the complete graph and authored bytes.

| Mode | Documents | Groups | Affected leaf members | Full D rows | E range calls | Generated navigation parts |
| --- | --- | --- | --- | --- | --- | --- |
{table}

The growing-edit proof has zero full header-range rows, edge range calls bounded by the one affected leaf, and exact equality of all sibling group FileSpecs. Manual source tracing confirms the growing-record branch skips the shrinking-only sibling loop. This is counter/byte evidence; no syscall open trace or performance matrix is claimed.

Bash reads SELECTED once into a retained copy and derives one B generation path from its publication. Listing/search/copy uses ordinary find/sort/grep/sed/cp/shasum commands; no engine operation runs during extraction. An independent checker then compares the copied file to the bound immutable original and SHA-256. Raw listings, search output, selector, chosen path, copied bytes and hash output are preserved.

## Preserved failures and repair

The first creation command had a Python nested-string SyntaxError before creating a helper or running tests; setup-attempt.txt retains it. The first full worker completed recovery and 13 supplements, then exited 1 on the B extraction harness: sed matched both top-level and nested `attempt` fields and formed a two-line generation path. Its supervisor, stderr, traceback and failed shell-B files remain intact. The only repair was a separately bound Bash parser restricted to the top-level field; a fresh B shell output directory passed in {retryresource['seconds']:.6f} seconds against the same frozen B store. No engine fix or rerun of successful cases. The original supervisor failure is never relabeled as exit 0.

## Resources and closure

Sampled aggregate peak for the worker/engine descendants: **{summary['aggregate_sampled_peak_rss_bytes']} bytes**, below 3 GiB. Checker sampled peak: **{summary['checker_sampled_peak_rss_bytes']} bytes**, below 2 GiB. Ordinary adapter recorded {len(resources)} rows, with **{summary['ordinary_missing_rss_rows']} missing RSS samples retained as null**; maximum observed ordinary RSS was {summary['ordinary_sampled_max_rss_bytes']} bytes. Point/inspect/resolve caps were 256 MiB; other children 2 GiB; ordinary timeout 120 seconds. The whole verifier watchdog was 900 seconds and capsule bound 1,800 seconds, both preempted by the fixed functional cutoff. Mac external ps sampling was used; no /proc substitute or time-l sampler. Both worker and B retry process groups were empty after completion.

Final precommit continuing ownership plus conservative shared Git growth: **{snapshot['owned_bytes']} bytes / {snapshot['owned_files']} files**. New follow-up runtime: **{runtime['logical_bytes']} bytes / {runtime['files']} files**, within the additional 256 MiB / 25,000-file prospective bound. All continuing source/build/censored fixtures/new fixtures/returns remain under the total 768 MiB / 75,000-file ceiling. Free space: **{snapshot['free_bytes']} bytes**, above 20 GiB. Prior Q roots were recensused and unchanged. No deletion or new tool-cache growth was used. Baseline/final and approximately two-second runtime censuses retain actual scopes and counts; transient disk peak is not asserted from snapshots.

Raw bounded evidence is committed; source/build/datasets remain ignored under the existing attached worktree's runtime/. Parent owns independent review/integration and P admission. **P remains unadmitted.**
'''
(O/'REPORT.md').write_text(report)
manifest={str(p.relative_to(O)):dict(bytes=p.stat().st_size,sha256=f.sha(p)) for p in O.rglob('*') if p.is_file() and '__pycache__' not in p.relative_to(O).parts and p.name!='manifest.json'}
f.save('manifest.json',manifest)
print(json.dumps(dict(status=summary['status'],source=summary['core_source'],exe=summary['exe_sha256'],recovery_cases=24,recovery_events=282,supplement_cases=len(passed),functional_finished=retry['time'],census=snapshot,followup_runtime=runtime),indent=2))
