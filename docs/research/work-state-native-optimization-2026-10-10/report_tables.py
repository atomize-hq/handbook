"""Derive the report tables directly from the retained summary."""
import json,pathlib,sys
here=pathlib.Path(sys.argv[1]); data=json.loads((here/'summary.json').read_text())
def group(cohort,op,history,variant,on=False):
    return next(g for g in data['groups'] if (g['cohort'],g['operation'],g['history'],g['variant'],g['instrumented'])==(cohort,op,history,variant,on))
def ms(*args):return f"{group(*args)['median_ms']:.1f}"
def table(header,rows):
    return '\n'.join(['| '+' | '.join(header)+' |','| '+' | '.join(['---']*len(header))+' |']+
        ['| '+' | '.join(map(str,r))+' |' for r in rows])+'\n'
variants=['base','owner','binding','semantic','combined','direct','combined-direct']
parts=['All values are milliseconds unless marked otherwise. Current reads use four unprofiled observations per group; preparation uses six.\n']
parts.append(table(['Variant','Explicit current, 32','Kind/D→G current, 32','Explicit prepare, 31','Kind/D→G prepare, 16'],
    [[v,ms('explicit','current_item',32,v),ms('kind-DG','current_item',32,v),ms('explicit','preparation',31,v),ms('kind-DG','preparation',16,v)] for v in variants]))
base=next(r for r in data['matched_effects'] if r['variant']=='base')['paired_workflow_median_ms']
parts.append('Fresh explicit workflows: six matched entry sizes 25–30 per variant. Paired workflow is the median of each preparation plus its actual invocation.\n')
parts.append(table(['Variant','Preparation','Invocation','Paired workflow','Workflow reduction'],
    [[r['variant'],f"{r['preparation_median_ms']:.1f}",f"{r['invocation_median_ms']:.1f}",
      f"{r['paired_workflow_median_ms']:.1f}",f"{100*(1-r['paired_workflow_median_ms']/base):.1f}%"] for r in data['matched_effects']]))
parts.append('Full-store original/current-control operations: six unprofiled observations each.\n')
parts.append(table(['Cohort / operation','Base','Combined','Combined-direct (departure)'],
    [[f'{c} / {op}']+[ms(c,op,32,v) for v in ('base','combined','combined-direct')]
     for c in ('explicit','kind-DG') for op in ('historical_resolve','committed_recovery','current_grant','governance_basis')]))
parts.append('Illustrative profiled call counts (explicit current at sample 1; fresh invocation at entry 31). Repeated paths count again on every observation.\n')
selected=[]
for op in ('current_item','fresh_invocation'):
    for v in ('base','combined','combined-direct'):
        p=next(p for p in data['profiles'] if p['cohort']=='explicit' and p['operation']==op and p['history_slots_at_entry']==(32 if op=='current_item' else 31) and p['variant']==v and p['sample']==(1 if op=='current_item' else 6))
        selected.append([f'{op} / {v}',p['scan_passes'],p['directory_list_calls'],p['list_entries_observed'],
            p['directory_opens'],p['handle_information_calls'],p['canonical_calls']])
parts.append(table(['Operation / variant','Scans','Lists','Listed entries','Directory opens','Handle info','Canonical encodings'],selected))
(here/'report-tables.md').write_text('\n'.join(parts),encoding='utf-8')
print('report-tables.md generated')
