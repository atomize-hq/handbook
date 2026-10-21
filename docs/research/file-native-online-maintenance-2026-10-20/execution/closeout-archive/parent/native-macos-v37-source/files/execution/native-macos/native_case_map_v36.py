"""Private final-native concrete-provider map. No imports of engine source.
Existing case identity bytes remain authority. Provider names bind actual shared
constructors, never recipe steps or asserted PASS labels. Missing bindings refuse.
"""
import hashlib,json,re
INVENTORY_SHA256='cd96e0f212c481257127aec3f601bf852156cdcc853fb8ea21de402106298146'
FAMILIES={'legacy-packing':52,'legacy-recovery':24,'strict':24,'focused':8,'new-focused':43,'workspace':5,'ordinary':1,'activation':1}
BASE=('native_binding','source_binding','positive_closure','independent_oracle','read_only_oracle')
NEW={'S-A-zstd': ('small', ()), 'S-B-zstd': ('small', ()), 'C-P0-small': ('matched_small_P0', ()), 'D-partial-fixed-cutoff': ('partial_interval', ()), 'D-reservation-late-eligible': ('reservation_late', ()), 'D-cutoff-Q-stale-CAS': ('cutoff_and_stale_cas', ()), 'F1-terminal-retired-bootstrap-delta': ('terminal_retirement', ()), 'F2-auto-decline-later-admit': ('automatic_policy', ()), 'D-stable-base-segment-pressure': ('stable_base', ()), 'S-persistent-owners': ('persistent_owners', ()), 'S-two-maintainers-semantic-crossing': ('contenders', ()), 'S-reader-error': ('reader_error', ()), 'S-reader-cancel': ('reader_cancel', ()), 'S-reader-death': ('reader_death', ()), 'S-maintainer-death': ('maintainer_death', ()), 'S-path-reuse-fresh-birth': ('fresh_birth_path_reuse', ()), 'D-terminal-oversize': ('immutable_terminal_oversize', ()), 'D-deferred128-and-state32KiB': ('deferred_boundaries', ()), 'D-segment4-and-collateral64MiB': ('segment_boundaries', ()), 'Q-raw8MiB': ('raw_boundaries', ()), 'Q-pack64MiB-index16MiB-record65536': ('pack_boundaries', ()), 'Q-catalog64-Work128KiB-cache64MiB': ('metadata_boundaries', ()), 'CP3-six-outcome-categories': ('paged_outcomes', ()), 'CP3-authenticated-negative-all-segments': ('paged_negative', ()), 'CP3-touched-route-leaf-raw': ('paged_touched_corruption', ()), 'CP3-untouched-full-audit': ('paged_untouched_corruption', ()), 'CP3-duplicates-segments': ('paged_duplicates', ()), 'CP3-candidate-full-verification': ('paged_candidate', ()), 'Compat-STORE1-STORE2-to-new': ('legacy_compatibility', ()), 'Compat-STORE3-to-final': ('v3_compatibility', ()), 'Backup-durable-pin-locks': ('durable_pin_backup', ()), 'Backup-shallow-final-restore-successor': ('restore_successor', ()), 'Final-two-pass-drain': ('drain_two_passes', ())}
CRASH_PHASES=('pack-reservation','pack-install','pack-before-catalog','physical-before-selection','physical-after-selection','pack-accounting','pack-loose-delete','pack-old-delete','physical-after-owned-file-delete','physical-progress-plan-installed')
def need(ok,message):
 if not ok:raise RuntimeError(message)
def pack_route(name):
 if name.startswith('operational-'):return 'packing_operational',{'mode':name[-1]},('default_payloads','operational','ordinary_exact')
 if name in ('orphan-unregistered-backup-restore','unselected-installed-backup-restore'):return 'packing_backup',{'state':name},('exact_backup','restore_successor','durable_pins')
 m=re.fullmatch(r'interruption-([AB])-(.+)-([13])',name)
 if m:return 'packing_interruption',{'mode':m[1],'phase':m[2],'occurrence':int(m[3])},('checkpoint_exact','correcting_successor','accepted_exact')
 m=re.fullmatch(r'protected-read-([AB])-(.+)',name)
 if m:return 'packing_protected_reader',{'mode':m[1],'phase':m[2]},('captured_identity','retained_while_live','reaped_after_join')
 for prefix,handler,checks in (
 ('writer-and-competing-maintenance-','packing_contenders',('semantic_advance','positive_work_overlap')),
 ('cancelled-drain-releases-barriers-','packing_cancelled_drain',('barriers_released','dependent_IO_joined')),
 ('repack-old-pack-deletion-interruption-','packing_old_delete',('actual_old_container_deleted','correcting_successor','fresh_birth_safe'))):
  if name.startswith(prefix):return handler,{'mode':name[-1]},checks
 return 'packing_malformed',{'fault':name},('exact_refusal','allocation_bounded')
def recovery_route(name):
 p=name.split('/')
 if p[0]=='navigation-and-operational-backup':return 'recovery_backup',{'mode':p[1],'N':int(p[2][1:])},('navigation_exact','operational_exact','backup_exact','restore_successor')
 if p[0]=='namespace-interruption-correcting-successor':return 'recovery_namespace',{'direction':p[1],'phase':p[2]},('checkpoint_exact','namespace_exact','correcting_successor')
 if p[0]=='checkpoint-transfer-retire-repeat-gc':return 'recovery_transfer',{'mode':p[1]},('checkpoint','transfer','retire','repeat_gc','retired_refusal')
 if p[0]=='half-preparation-backup-recovery':return 'recovery_half_prepare',{'mode':p[1]},('half_preparation','durable_pin_backup','recovery_exact')
 if p[0]=='impossible-final-namespace':return 'recovery_impossible',{},('impossible_namespace_refusal','no_partial_acceptance')
 need(p[0]=='skipped-or-selected-once','unknown-recovery')
 return 'recovery_selected_once',{'mode':p[1]},('selection_read_once','skipped_namespace_exact')
def strict_route(name):
 if name.startswith('strict-numeric-type-'):return 'strict_numeric',{'member':name[20:]},('bool_refused','fraction_refused','negative_refused','overflow_refused')
 if name in ('independent-oversize-catalog','independent-oversize-index'):return 'strict_oversize',{'structure':name.rsplit('-',1)[1]},('preallocation_refusal','exact_boundaries')
 if name=='physical-catalog-and-hint-cannot-grant-membership':return 'strict_membership',{},('semantic_membership_independent','physical_hint_no_authority')
 need(name.startswith('independent-'),'unknown-strict-case')
 return 'strict_malformed',{'fault':name[12:]},('independent_refusal','exact_refusal','allocation_bounded')
class CaseMap:
 def __init__(self,inventory_bytes):
  need(hashlib.sha256(inventory_bytes).hexdigest()==INVENTORY_SHA256,'case-inventory-byte-binding')
  inv=json.loads(inventory_bytes);self.rows=inv['case_IDs_preserved'];need(len(self.rows)==158,'all-158-IDs')
  counts={k:sum(v['group']==k for v in self.rows.values()) for k in FAMILIES};need(counts==FAMILIES,'case-family-counts')
  self.plans={}
  for case_id,row in self.rows.items():
   group=row['group'];args={}
   if group=='legacy-packing':handler,args,checks=pack_route(case_id[2:])
   elif group=='legacy-recovery':handler,args,checks=recovery_route(case_id[2:])
   elif group=='strict':handler,args,checks=strict_route(case_id[2:])
   elif group=='focused':handler='generation_lifetime';args={'obligation':case_id[2:]};checks=('selected_once_actual_Bash','frozen_generation_exact','B_controls_retained','retired_refusal')
   elif group=='new-focused':
    if case_id.startswith('S-crash-'):
     phase=case_id[8:];need(phase in CRASH_PHASES,'crash-phase');handler='crash_extension';args={'phase':phase};checks=('checkpoint_exact','full_crash_oracle','correcting_successor')
     if 'delete' in phase:checks+=('actual_removal_identity',)
    else:
     need(case_id in NEW,'new-focused-ID');handler,checks=NEW[case_id]
     if case_id in ('S-A-zstd','S-B-zstd'):args={'mode':case_id[2]}
   elif group=='workspace':
    handler='workspace_slot';args={'slot':int(case_id.rsplit('-',1)[1])};checks=('exact_unit_manifest','all_units_in_slot','per_unit_raw_exit','no_broad_workspace_label')
   elif group=='ordinary':handler='ordinary_routes';checks=('all_17_original_routes','per_route_raw_result','selection_read_once','actual_Bash')
   else:need(group=='activation' and case_id=='X-native-pressure','unknown-activation');handler='native_pressure';args={'native_focused':True};checks=('actual_new_mapping','accepted_tail','separate_work_CAS','native_pressure_bound')
   self.plans[case_id]={'case_id':case_id,'group':group,'handler':handler,'args':args,'required_checks':BASE+checks,'obligation':row['obligation'],'reuse':row['reuse']}
 def __getitem__(self,case_id):
  need(case_id in self.plans,'unknown-case-ID');return self.plans[case_id]
 def provider(self,case_id):
  group=self[case_id]['group']
  if group=='legacy-packing':return ('suite_packing','packing_dispatch',case_id[2:])
  if group=='legacy-recovery':return ('suite_recovery','dispatch',case_id[2:])
  if group=='strict':return ('suite_packing','strict_case',case_id[2:])
  if group=='focused':return ('suite_packing','focused',case_id[2:])
  if group=='workspace':return ('native_suite_v36','workspace',None)
  if group=='ordinary':return ('suite_ordinary','run',None)
  return ('suite_extensions','EXTENSIONS',case_id)
 def bind_providers(self,cases,sdk):
  need(set(cases)==set(self.plans),'complete-exact158-provider-map')
  need(sdk['parent_reviewed'] is True and sdk['engine_code_imported'] is False,'reviewed-shared-SDK-closure')
  sources=sdk['modules']
  self.provider_bindings={}
  for case_id,plan in self.plans.items():
   row=cases[case_id];module,function,arg=self.provider(case_id);provider=row['provider']
   need(row['case_id']==case_id and row['handler']==plan['handler'] and row['source_pin']==sdk['source_pin'],'exact-case-handler-source')
   need(set(provider)=={'module','function','case_argument','source_sha256'},'closed-concrete-provider-record')
   need((provider['module'],provider['function'],provider['case_argument'])==(module,function,arg),'actual-shared-constructor-route:'+case_id)
   need(module in sources and provider['source_sha256']==sources[module]['sha256'],'constructor-source-file-binding')
   need(row['parent_released'] is True and row['obligations']==self.rows[case_id]['obligation'],'preserved-case-obligations')
   need('recipe' not in row and 'case_assertions' not in row,'no-recipe-or-label-substitute')
   self.provider_bindings[case_id]=dict(provider)
  return True
 def bind_modules(self,modules):
  need(hasattr(self,'provider_bindings'),'parent-provider-bindings-before-imports')
  need(set(modules)>=set(v['module'] for v in self.provider_bindings.values()),'all-provider-modules')
  for case_id,v in self.provider_bindings.items():
   module=modules[v['module']];target=getattr(module,v['function'])
   if v['function']=='EXTENSIONS':
    need(type(target) is dict and case_id in target,'actual-extension-constructor:'+case_id);target=target[case_id]
   need(callable(target) and getattr(target,'__code__',None) is not None,'actual-Python-constructor-body')
   need(module.__source_sha256__==v['source_sha256'],'loaded-provider-source-bytes')
  self.modules=modules
 def dispatch(self,case,spec):
  v=self.provider_bindings[case.id];module=self.modules[v['module']];fn=getattr(module,v['function'])
  if v['function']=='EXTENSIONS':return fn[case.id](case,spec)
  if v['case_argument'] is not None:return fn(case,v['case_argument'])
  return fn(case)
if __name__=='__main__':raise SystemExit('SOURCE_ONLY_PARENT_REVIEW_AND_FINAL_SOURCE_BINDINGS_REQUIRED')
