//! Finish only the preserved interrupted 3e4de SDK cohort. No population loop.
#[allow(dead_code)]
#[path = "../tests/local_contract_support/mod.rs"]
mod local_contract_support;
#[allow(dead_code)]
#[path = "../tests/work_kind_support/mod.rs"]
mod support;
#[allow(dead_code)]
#[path = "work_state_capacity.rs"]
mod original_consumer;
use handbook_sdk::{HandbookSdkV1, SetupMode, SetupRequest};
use handbook_sdk::work_state::*;
use handbook_sdk::local_setup::LocalInitialSetupLocatorV1;
use handbook_contracts::project_admission_governance::DeferralRequestLocatorV2;
use handbook_engine::{RetainedRepositoryRootV1, RepositoryReadV1};
use serde::{Deserialize, Serialize};
use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs::{File,OpenOptions},io::Write,path::{Path,PathBuf}};
type R<T> = Result<T,String>;
fn debug(e:impl std::fmt::Debug)->String {format!("{e:?}")}
fn digest(bytes:&[u8])->String {format!("{:x}",Sha256::digest(bytes))}
fn save(path:&Path,bytes:&[u8])->R<()> {
    let mut file=OpenOptions::new().create_new(true).write(true).open(path).map_err(debug)?;
    file.write_all(bytes).map_err(debug)?;file.sync_all().map_err(debug)
}
fn action(log:&mut File,api:&str,key:&str,result:serde_json::Value)->R<()> {
    writeln!(log,"{}",serde_json::json!({"api":api,"request_key":key,"result":result})).map_err(debug)?;
    log.sync_data().map_err(debug)
}
// Exact existing Contracts AnchorV2 projection; it only restores portable input.
// Native SDK/owner guards, not these serialized fields, establish authority.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RetainedDeferralAnchor {
    locator:DeferralRequestLocatorV2,
    governance_lock_fingerprint:String,
}
// External diagnostic reply reader only; the SDK execution wrapper is
// intentionally Serialize-only. This never constructs a prepared capability.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PriorExecution {
    disposition:String,
    refusal:Option<serde_json::Value>,
    outcome:Option<WorkRegistrationReceiptV1>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PriorRow {
    stage:String,
    request_key:String,
    execution:PriorExecution,
}
#[derive(Debug,Serialize)]
struct Counts {
    pages:u64, examined_original_records:u64,
    live_work_items:u64, retained_original_work_outcomes:u64,
    kind_outcomes:u64, grant_outcomes:u64, revocation_outcomes:u64,
    retained_owner_record_bytes:u64, verified_required_originals:u64,
    #[serde(skip)] work_keys:BTreeSet<String>,
}
fn recount(sdk:&HandbookSdkV1,required:&BTreeMap<String,WorkRegistrationReceiptV1>)->R<Counts> {
    sdk.with_retained_work_snapshot(|view|->R<Counts>{
        let mut counts=Counts{pages:0,examined_original_records:0,live_work_items:0,retained_original_work_outcomes:0,kind_outcomes:0,grant_outcomes:0,revocation_outcomes:0,retained_owner_record_bytes:0,verified_required_originals:0,work_keys:BTreeSet::new()};
        let mut names=BTreeSet::new();let mut identities=BTreeSet::new();let mut originals=BTreeSet::new();let mut verified=BTreeSet::new();
        let mut after:Option<String>=None;
        loop {
            let page=view.page(after.as_deref(),16).map_err(debug)?;counts.pages+=1;
            for record in page.records {
                if !names.insert(record.original_request_name.clone()){return Err("duplicate original in independent page traversal".into());}
                counts.examined_original_records+=1;counts.retained_owner_record_bytes+=record.retained_record_bytes;
                if record.disposition!=WorkDispositionV1::Replayed {return Err(format!("nonterminal/unresolved retained original {}: {:?}",record.original_request_name,record.disposition));}
                match record.outcome.ok_or("retained terminal missing outcome")? {
                    WorkRetainedOutcomeV1::Item(receipt)=>{
                        if receipt.snapshot.state!="open"{return Err("unexpected retained item state".into());}
                        identities.insert(receipt.locator.item_identity.clone());
                        if !originals.insert(receipt.locator.fingerprint.clone()){return Err("duplicate original Work fingerprint".into());}
                        let key=receipt.locator.input.request_key.clone();
                        if !counts.work_keys.insert(key.clone()){return Err("duplicate original Work key".into());}
                        if let Some(expected)=required.get(&key){
                            if expected!=&receipt{return Err(format!("exact retained original changed: {key}"));}
                            verified.insert(key);
                        }
                    },
                    WorkRetainedOutcomeV1::Kind(_)=>counts.kind_outcomes+=1,
                    WorkRetainedOutcomeV1::Grant(_)=>counts.grant_outcomes+=1,
                    WorkRetainedOutcomeV1::Revoke(_)=>counts.revocation_outcomes+=1,
                }
            }
            if page.complete{break}
            let next=page.next.ok_or("incomplete bounded page without continuation")?;
            if after.as_ref().is_some_and(|old|next<=*old){return Err("history continuation did not advance".into());}
            after=Some(next);
        }
        counts.live_work_items=identities.len()as u64;counts.retained_original_work_outcomes=originals.len()as u64;counts.verified_required_originals=verified.len()as u64;
        if verified.len()!=required.len(){return Err("required original omitted by independent traversal".into());}
        Ok(counts)
    }).map_err(debug)?
}
fn drain(sdk:&HandbookSdkV1,log:&mut File,phase:&str)->R<()> {
    loop {
        let result=sdk.maintain_work_storage(WorkMaintenanceBudgetV1{items:4096,bytes:64*1024*1024});
        action(log,"maintain_work_storage",phase,serde_json::json!({"reply":format!("{result:?}")}))?;
        let progress=result.map_err(debug)?;
        if progress.complete{return Ok(())}
    }
}
fn exact_original(sdk:&HandbookSdkV1,receipt:&WorkRegistrationReceiptV1,log:&mut File,recover:bool)->R<()> {
    let reply=if recover{sdk.recover_work_registration_request(&receipt.locator)}else{sdk.resolve_work_registration_request(&receipt.locator)};
    action(log,if recover{"recover_work_registration_request"}else{"resolve_work_registration_request"},&receipt.locator.input.request_key,serde_json::to_value(&reply).map_err(debug)?)?;
    if !matches!(reply.disposition,WorkDispositionV1::Committed|WorkDispositionV1::Replayed)||reply.outcome.as_ref()!=Some(receipt){return Err("exact original did not survive".into());}
    Ok(())
}
fn run(root:&Path,prior:&Path,proof:&Path)->R<()> {
    if root!=Path::new("/home/spenser/file-native-delivery-20261028-proof-runtime/principal-3e4de-sdk-100000")||!root.is_dir()||!proof.is_dir()||proof.starts_with(root)||root.starts_with(proof){return Err("exact existing cohort and precreated disjoint evidence required".into());}
    let original_bytes=std::fs::read(prior.join("current-original.json")).map_err(debug)?;
    let trace_bytes=std::fs::read(prior.join("original-transitions.jsonl")).map_err(debug)?;
    if digest(&original_bytes)!="25467067ca71558d543f248e78078dbbef95465ce8eb1852e721eb8d72f0dd3d"||digest(&trace_bytes)!="226e6c5e44840fe3d8b94da7dd40f4ebfeeffcc2972cee9f18f73c5ed98e5e47"{return Err("preserved original/trace binding changed".into());}
    save(&proof.join("pre-finish-original.json"),&original_bytes)?;
    save(&proof.join("pre-finish-transitions.jsonl"),&trace_bytes)?;
    let saved=WorkRequestLocatorV1::from_json_bytes(&original_bytes).map_err(debug)?;
    if saved.canonical_json_bytes().map_err(debug)?!=original_bytes{return Err("saved original bytes are not exact canonical input".into());}
    let mut required=BTreeMap::new();
    for line in trace_bytes.split(|b|*b==b'\n').filter(|line|!line.is_empty()){
        let row:PriorRow=serde_json::from_slice(line).map_err(debug)?;
        if row.stage!="invoke"||row.execution.disposition!="committed"||row.execution.refusal.is_some(){return Err("prior trace contradicts preserved successful prefixes".into());}
        let receipt=row.execution.outcome.ok_or("prior committed reply missing outcome")?;
        if receipt.locator.input.request_key!=row.request_key||required.insert(row.request_key,receipt).is_some(){return Err("prior trace duplicate/different original".into());}
    }
    let early:Vec<_>=required.values().take(3).cloned().collect();
    if early.is_empty(){return Err("no prior exact originals".into());}
    let mut log=OpenOptions::new().create_new(true).write(true).open(proof.join("api-actions.jsonl")).map_err(debug)?;
    let mut transitions=OpenOptions::new().create_new(true).write(true).open(proof.join("original-transitions.jsonl")).map_err(debug)?;
    let retained=RetainedRepositoryRootV1::open_exact(root).map_err(debug)?;
    let anchor_bytes=match retained.read_regular_bounded(".handbook/admission-governance-project.v2.json",1024*1024).map_err(debug)?{RepositoryReadV1::Found(b)=>b,RepositoryReadV1::Absent=>return Err("original deferral anchor missing".into())};
    if digest(&anchor_bytes)!="31d507dde0b18ef5617ad4b0b87ff9fb8c734dd82bbbb6ae1cdadd8a032142b9"{return Err("original deferral anchor provenance changed".into());}
    let anchor:RetainedDeferralAnchor=serde_json::from_slice(&anchor_bytes).map_err(debug)?;
    let deferral=DeferralRequestLocatorV2::from_json_bytes(&anchor.locator.canonical_json_bytes().map_err(debug)?).map_err(debug)?;
    let check=saved.grant.check.clone();
    let initial=LocalInitialSetupLocatorV1::from_json_bytes(&serde_json::to_vec(&serde_json::json!({"schema_id":"handbook.sdk.local-initial-setup-locator","schema_version":"1.0.0","deferral":deferral,"charter":check.predecessor.charter,"terms":check.predecessor})).map_err(debug)?).map_err(debug)?;
    save(&proof.join("exact-original-initial-setup-locator.json"),&initial.canonical_json_bytes().map_err(debug)?)?;
    action(&mut log,"restore_original_portable_locator","initial",serde_json::json!({"anchor_sha256":digest(&anchor_bytes),"observed_lock_fingerprint":anchor.governance_lock_fingerprint,"permission":"none; live native SDK guards remain required"}))?;
    drop(retained);
    let sdk=HandbookSdkV1::open(root);
    // FIRST SDK operation: resolve the exact saved original; never start a new key.
    let first=sdk.resolve_work_registration_request(&saved);
    action(&mut log,"resolve_work_registration_request",&saved.input.request_key,serde_json::to_value(&first).map_err(debug)?)?;
    action(&mut log,"finite_helper_entry_source",&saved.input.request_key,serde_json::json!({"entry_source":"resolve_work_registration_request","legacy_helper_first_tag":"invoke","fresh_invocation_claim":false}))?;
    let receipt=original_consumer::finish_same_original(&sdk,&saved,first,||{
        let prepared=sdk.prepare_work_registration(&initial,&saved.grant,saved.input.clone());
        action(&mut log,"prepare_work_registration_for_same_saved_original",&saved.input.request_key,serde_json::json!({"reply":format!("{prepared:?}")})).expect("retain actual retry preparation reply");
        prepared.expect("same-original preparation refused; stop")
    },&mut transitions)?;
    if required.insert(receipt.locator.input.request_key.clone(),receipt.clone()).is_some(){return Err("saved pending original unexpectedly duplicates prior trace".into());}
    let before=recount(&sdk,&required)?;
    if before.live_work_items<3000||before.retained_owner_record_bytes<=128*1024*1024{return Err("existing demonstrated range/owner truth not preserved".into());}
    action(&mut log,"independent_page16_recount","before_tail",serde_json::to_value(&before).map_err(debug)?)?;
    drain(&sdk,&mut log,"before_tail")?;
    for receipt in &early {
        exact_original(&sdk,receipt,&mut log,false)?;
        let current=sdk.resolve_current_work_item(&saved.grant,&receipt.locator.input.item_key);
        action(&mut log,"resolve_current_work_item",&receipt.locator.input.request_key,serde_json::json!({"reply":format!("{current:?}")}))?;
        if current.map_err(debug)?!=WorkCurrentItemV1::Open(receipt.snapshot.clone()){return Err("early current item changed".into());}
    }
    let admin=sdk.prepare_work_registration_terms(&initial,&check,support::terms(&check,"sdk-finish-fresh-admin"),WorkTermsTransitionV1::ScopeChange(&saved.grant)).map_err(debug)?;
    let successor=admin.locator().clone();save(&proof.join("fresh-admin-original.json"),&successor.canonical_json_bytes().map_err(debug)?)?;
    let activated=sdk.activate_work_registration_terms(admin);
    action(&mut log,"activate_work_registration_terms",&successor.terms.request_key,serde_json::to_value(&activated).map_err(debug)?)?;
    if activated.disposition!=WorkDispositionV1::Committed||activated.outcome.as_ref().map(|v|&v.locator)!=Some(&successor){return Err("fresh administration unresolved/refused; stop without generic recovery".into());}
    let mut fresh_work=0u64;
    for n in 0..32 {
        let key=format!("sdk-finish-beyond-{n}");
        if before.work_keys.contains(&key){return Err("fresh tail key already existed".into());}
        let input=WorkRegistrationInputV1{request_key:key.clone(),item_key:key.clone(),label:"Beyond demonstrated operating range".into(),question:"Continued authorized bounded publication".into(),actor:successor.terms.agent.clone(),expected_state:"absent".into(),composition:WorkCompositionV1::ordinary_leaf().into(),display_level:None};
        let prepared=sdk.prepare_work_registration(&initial,&successor,input.clone()).map_err(debug)?;
        let locator=prepared.locator().clone();save(&proof.join(format!("fresh-work-{n:02}-original.json")),&locator.canonical_json_bytes().map_err(debug)?)?;
        let first=sdk.register_work_item(prepared);
        action(&mut log,"register_work_item",&key,serde_json::to_value(&first).map_err(debug)?)?;
        let receipt=original_consumer::finish_same_original(&sdk,&locator,first,||sdk.prepare_work_registration(&initial,&successor,input.clone()).expect("same fresh original preparation refused"),&mut transitions)?;
        if required.insert(key,receipt).is_some(){return Err("fresh original duplicate".into());}fresh_work+=1;
    }
    let revoke=sdk.prepare_work_registration_revocation(&successor,"sdk-finish-fresh-revoke",&successor.terms.revocation_principal,"human.sdk-finish","Revoke after bounded SDK operating-range continuation").map_err(debug)?;
    save(&proof.join("fresh-revocation-original.json"),&revoke.canonical_json_bytes().map_err(debug)?)?;
    let revoked=sdk.revoke_work_registration_terms(&revoke);
    action(&mut log,"revoke_work_registration_terms","sdk-finish-fresh-revoke",serde_json::to_value(&revoked).map_err(debug)?)?;
    if revoked.disposition!=WorkDispositionV1::Committed||revoked.outcome.as_ref().map(|v|&v.locator)!=Some(&revoke){return Err("fresh revocation unresolved/refused; stop without generic recovery".into());}
    for r in early.iter().chain(std::iter::once(&receipt)){exact_original(&sdk,r,&mut log,true)?;}
    drain(&sdk,&mut log,"after_tail")?;
    let after=recount(&sdk,&required)?;
    if after.live_work_items!=before.live_work_items+fresh_work||after.retained_original_work_outcomes!=before.retained_original_work_outcomes+fresh_work||after.grant_outcomes!=before.grant_outcomes+1||after.revocation_outcomes!=before.revocation_outcomes+1{return Err("independent fresh Work/admin outcome deltas differ".into());}
    if fresh_work+2<=32{return Err("fresh accepted owner operations did not cross32".into());}
    action(&mut log,"independent_page16_recount","after_tail",serde_json::to_value(&after).map_err(debug)?)?;
    let reset=sdk.run_setup(&SetupRequest{mode:SetupMode::Refresh,rewrite:false,reset_state:true});
    action(&mut log,"run_setup_reset","reset",serde_json::json!({"reply":format!("{reset:?}")}))?;
    if !reset.map_err(debug)?.reset_applied{return Err("reset not applied".into());}
    drop(sdk);
    let reopened=HandbookSdkV1::open(root);
    for r in early.iter().chain(std::iter::once(&receipt)){exact_original(&reopened,r,&mut log,false)?;}
    // Exact original administration and revocation receipts, not only counts,
    // must survive reset and reopen without restoring operating permission.
    let grant_original=reopened.resolve_work_registration_terms_request(&successor);
    action(&mut log,"resolve_work_registration_terms_request",&successor.terms.request_key,serde_json::to_value(&grant_original).map_err(debug)?)?;
    if !matches!(grant_original.disposition,WorkDispositionV1::Committed|WorkDispositionV1::Replayed)
        ||grant_original.outcome.as_ref()!=activated.outcome.as_ref(){
        return Err("exact fresh administration receipt changed after reset/reopen".into());
    }
    let retained_grant=grant_original.outcome.as_ref().ok_or("original grant receipt missing")?;
    if retained_grant.locator.canonical_json_bytes().map_err(debug)?
        !=std::fs::read(proof.join("fresh-admin-original.json")).map_err(debug)?{
        return Err("fresh administration canonical original changed".into());
    }
    let revocation_original=reopened.resolve_work_registration_revocation_request(&revoke);
    action(&mut log,"resolve_work_registration_revocation_request","sdk-finish-fresh-revoke",serde_json::to_value(&revocation_original).map_err(debug)?)?;
    if !matches!(revocation_original.disposition,WorkDispositionV1::Committed|WorkDispositionV1::Replayed)
        ||revocation_original.outcome.as_ref()!=revoked.outcome.as_ref(){
        return Err("exact fresh revocation receipt changed after reset/reopen".into());
    }
    let retained_revocation=revocation_original.outcome.as_ref().ok_or("original revocation receipt missing")?;
    if retained_revocation.locator.canonical_json_bytes().map_err(debug)?
        !=std::fs::read(proof.join("fresh-revocation-original.json")).map_err(debug)?{
        return Err("fresh revocation canonical original changed".into());
    }
    let final_counts=recount(&reopened,&required)?;
    if serde_json::to_value(&final_counts).map_err(debug)?!=serde_json::to_value(&after).map_err(debug)?{return Err("independent Work/admin/owner truth changed across reset/reopen".into());}
    action(&mut log,"independent_page16_recount","after_reset_reopen",serde_json::to_value(&final_counts).map_err(debug)?)?;
    let result=serde_json::json!({"complete":true,"scope":"bounded existing-cohort typed SDK finish;100k Handbook/1GiB Handbook/performance UNPROVED","initial_recount":before,"final_recount":final_counts,"fresh_work_originals":fresh_work,"fresh_administration_originals":2,"fresh_accepted_owner_operations":fresh_work+2,"required_exact_originals_verified":required.len(),"prior_raw_unchanged":digest(&std::fs::read(prior.join("original-transitions.jsonl")).map_err(debug)?)=="226e6c5e44840fe3d8b94da7dd40f4ebfeeffcc2972cee9f18f73c5ed98e5e47"});
    save(&proof.join("finish-result.json"),&serde_json::to_vec_pretty(&result).map_err(debug)?)?;
    println!("{}",serde_json::to_string(&result).map_err(debug)?);Ok(())
}
fn main(){
    let args:Vec<_>=std::env::args().collect();
    if args.len()!=4{eprintln!("ROOT PRIOR_EVIDENCE NEW_FINISH_EVIDENCE required");std::process::exit(2)}
    let(root,prior,proof)=(PathBuf::from(&args[1]),PathBuf::from(&args[2]),PathBuf::from(&args[3]));
    if let Err(error)=run(&root,&prior,&proof){
        let failure=serde_json::json!({"complete":false,"error":error,"no_automatic_retry":true});
        let _=save(&proof.join("finish-failure.json"),&serde_json::to_vec_pretty(&failure).unwrap());
        eprintln!("{failure}");std::process::exit(101)
    }
}
