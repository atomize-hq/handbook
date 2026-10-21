"""Private final-native case crosswalk. No imports of shared/engine source.
Existing case identity bytes remain authority; final recipes add assertions,
not replacement case IDs. Missing source bindings prohibit execution.
"""
import hashlib,json,re
INVENTORY_SHA256='cd96e0f212c481257127aec3f601bf852156cdcc853fb8ea21de402106298146'
FAMILIES={'legacy-packing':52,'legacy-recovery':24,'strict':24,'focused':8,'new-focused':43,'workspace':5,'ordinary':1,'activation':1}
BASE=('native_binding','source_binding','positive_closure','independent_oracle','read_only_oracle')
NEW={
'S-A-zstd':('small',('default_payloads','three_cycles','accepted_each_cycle','positive_work_overlap','coupled_capture')),
'S-B-zstd':('small',('default_payloads','three_cycles','accepted_each_cycle','positive_work_overlap','coupled_capture','B_controls_retained')),
'C-P0-small':('matched_small_P0',('matched_inputs','no_maintenance_control','no_deep_B_clone')),
'D-partial-fixed-cutoff':('partial_interval',('fixed_target','partial_progress','restart_exact','holes_preserved')),
'D-reservation-late-eligible':('reservation_late',('genuine_reservation','late_eligibility','later_retry')),
'D-cutoff-Q-stale-CAS':('cutoff_and_stale_cas',('semantic_cutoff','metadata_CAS_distinct','stale_refusal','quota_bound')),
'F1-terminal-retired-bootstrap-delta':('terminal_retirement',('terminal_exclusion','bootstrap_cursor','delta_cursor','retired_refusal')),
'F2-auto-decline-later-admit':('automatic_policy',('genuine_decline','later_admission','no_manual_repair','bounded_bootstrap')),
'D-stable-base-segment-pressure':('stable_base',('three_deltas','segment_bound','shared_base_retained')),
'S-persistent-owners':('persistent_owners',('preparation_pin','packet_pin','snapshot_pin','retired_refusal')),
'S-two-maintainers-semantic-crossing':('contenders',('two_native_maintainers','semantic_advance','positive_work_overlap')),
'S-reader-error':('reader_error',('captured_identity','native_error','dependent_IO_joined','pin_release_order')),
'S-reader-cancel':('reader_cancel',('captured_identity','cancel_ack','dependent_IO_joined','pin_release_order')),
'S-reader-death':('reader_death',('captured_identity','native_death','dependent_IO_joined','durable_owners_retained')),
'S-maintainer-death':('maintainer_death',('selection_checkpoint','journal_checkpoint','native_death','correcting_successor')),
'S-path-reuse-fresh-birth':('fresh_birth_path_reuse',('same_content','new_instance','stale_journal_refusal','live_birth_retained')),
'D-terminal-oversize':('immutable_terminal_oversize',('immutable_oversize','terminal_exclusion','no_repair')),
'D-deferred128-and-state32KiB':('deferred_boundaries',('count_128_129','bytes_32768_32769','genuine_holes','later_release')),
'D-segment4-and-collateral64MiB':('segment_boundaries',('segments_4_5','consolidation_or_backpressure','collateral_64MiB')),
'Q-raw8MiB':('raw_boundaries',('raw_below_at_above_8MiB','default_payloads')),
'Q-pack64MiB-index16MiB-record65536':('pack_boundaries',('pack_64MiB','index_16MiB','records_65536','exact_refusal')),
'Q-catalog64-Work128KiB-cache64MiB':('metadata_boundaries',('catalog_64_65','work_128KiB','cache_transients_64MiB')),
'CP3-six-outcome-categories':('paged_outcomes',('early','middle','recent','missing','cancelled','lost_reply')),
'CP3-authenticated-negative-all-segments':('paged_negative',('all_segments','route_boundary_order','leaf_boundary_order','authenticated_absence')),
'CP3-touched-route-leaf-raw':('paged_touched_corruption',('route_refusal','leaf_refusal','raw_refusal','loose_duplicate_no_mask')),
'CP3-untouched-full-audit':('paged_untouched_corruption',('point_scope','full_audit_refusal')),
'CP3-duplicates-segments':('paged_duplicates',('all_segments','equal_raw_agrees','conflicting_identity_refuses')),
'CP3-candidate-full-verification':('paged_candidate',('full_order','full_coverage','all_raw','before_selection')),
'Compat-STORE1-STORE2-to-new':('legacy_compatibility',('old_quiescence','real_old_client','final_successor','no_mixed_safety')),
'Compat-STORE3-to-final':('v3_compatibility',('exact_version_map','old_client_refusal','restore_successor')),
'Backup-durable-pin-locks':('durable_pin_backup',('all_durable_pins','exact_exclusion_list','quiescent_backup')),
'Backup-shallow-final-restore-successor':('restore_successor',('existing_shallow_donor','exact_restore','accepted_successor')),
'Final-two-pass-drain':('drain_two_passes',('producers_ended','blockers_ended','exactly_two_cleanup_calls','complete_backlog'))
}
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
 def bind_recipes(self,recipes):
  need(set(recipes)==set(self.plans),'complete-exact-158-recipes')
  for case_id,plan in self.plans.items():
   recipe=recipes[case_id]
   need(recipe['case_id']==case_id and recipe['handler']==plan['handler'],'recipe-case-handler')
   need(recipe['recipe']['case_args']==plan['args'],'exact-case-constructor-parameters')
   checks=recipe['case_assertions'];need(set(checks)>=set(plan['required_checks']),'direct-case-assertions-required:'+case_id)
   need(all(type(v) is dict and v.get('evidence') for v in checks.values()),'assertion-evidence-selectors')
  return True
if __name__=='__main__':raise SystemExit('SOURCE_ONLY_PARENT_REVIEW_AND_FINAL_SOURCE_BINDINGS_REQUIRED')
