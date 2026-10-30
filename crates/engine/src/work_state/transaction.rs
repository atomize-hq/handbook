use super::store::{self, WorkLock};
use super::database::db_error;
use super::types::WorkRequestBinding;
use super::*;
use crate::local_authority::local_check::{LocalCheckCurrentBasisV1, LocalCheckLocatorV1};
use crate::local_authority::work_registration::{
    with_work_registration_current, with_work_registration_establishment,
    WorkRegistrationAuthorityProofV1, WorkRegistrationCurrentV1,
};
use crate::RetainedRepositoryRootV1;
use std::collections::BTreeMap;
use std::path::Path;
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(
    tag = "operation",
    content = "locator",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum Operation {
    Kind(WorkKindLocatorV1),
    Grant(WorkGrantLocatorV1),
    Item(WorkRequestLocatorV1),
    Revoke(WorkRevokeLocatorV1),
}
impl Operation {
    fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        match self {
            Self::Kind(v) => v.validate(),
            Self::Grant(v) => v.validate(),
            Self::Item(v) => v.validate(),
            Self::Revoke(v) => v.validate(),
        }
    }
    fn grant(&self) -> &WorkGrantLocatorV1 {
        match self {
            Self::Kind(_) => {
                unreachable!("grant access is restricted to grant administration rows")
            }
            Self::Grant(v) => v,
            Self::Item(v) => &v.grant,
            Self::Revoke(v) => &v.grant,
        }
    }
    fn check(&self) -> &LocalCheckLocatorV1 {
        match self {
            Self::Kind(v) => &v.check,
            _ => &self.grant().check,
        }
    }
    fn basis(&self) -> &LocalCheckCurrentBasisV1 {
        match self {
            Self::Kind(v) => &v.basis,
            _ => &self.grant().basis,
        }
    }
    fn project(&self) -> &str {
        match self {
            Self::Kind(v) => v.project(),
            _ => self.grant().project(),
        }
    }
    fn bootstrap(&self) -> Option<store::BootstrapOrigin> {
        match self {
            Self::Kind(v) => Some(store::BootstrapOrigin::Kind(v.clone())),
            Self::Grant(v) => Some(store::BootstrapOrigin::Grant(v.clone())),
            _ => None,
        }
    }
    fn schema(&self) -> &'static str {
        match self {
            Self::Kind(_) => "handbook.work-intent@2.0.0",
            Self::Item(v) if !v.input.composition.retained() => "handbook.work-intent@2.0.0",
            _ => "handbook.work-intent@1.0.0",
        }
    }
    fn name(&self) -> String {
        match self {
            Self::Kind(v) => format!("kind-{}", v.approval.request_key),
            Self::Grant(v) => format!("grant-{}", v.terms.request_key),
            Self::Item(v) => format!("item-{}", v.input.request_key),
            Self::Revoke(v) => format!("revoke-{}", v.request_key),
        }
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(
    tag = "kind",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum Output {
    Kind(WorkKindReceiptV1),
    Grant(WorkGrantV1),
    Item(WorkRegistrationReceiptV1),
    Revoke(WorkRevocationReceiptV1),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Intent {
    schema: String,
    owner_identity: String,
    operation: Operation,
    authority: WorkRegistrationAuthorityProofV1,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Decision {
    intent_fingerprint: String,
    output: Output,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Terminal {
    decision_fingerprint: String,
    effect: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Publication {
    intent_fingerprint: String,
    target: String,
}
struct Parsed {
    intent: Intent,
    output: Output,
    committed: bool,
    noncommit: bool,
    attempted: bool,
    staging: bool,
}

// Pure successful derivations only. No status, inventory, physical handles or
// current permission is retained here. Exact intent bytes are the cache key;
// path/owner/project bindings and every present record are checked on each scan.
struct ExpectedRecords {
    intent: Intent,
    intent_bytes_fingerprint: String,
    output: Output,
    decision: Vec<u8>,
    snapshot: Vec<u8>,
    terminal: Vec<u8>,
    publication: Vec<u8>,
    noncommit: Vec<u8>,
}
#[derive(Default)]
struct Artifacts {
    records: BTreeMap<Vec<u8>, Rc<ExpectedRecords>>,
}
impl Artifacts {
    fn expected(
        &mut self,
        owner: &store::Owner,
        name: &str,
        bytes: &[u8],
        proofs: &mut std::collections::BTreeSet<Vec<u8>>,
    ) -> Result<Rc<ExpectedRecords>, LocalAuthorityRefusalV1> {
        if let Some(value) = self.records.get(bytes) {
            validate_original_context(owner, name, &value.intent)?;
            return Ok(Rc::clone(value));
        }
        let intent: Intent = decode(bytes)?;
        let item = if let Operation::Item(locator) = &intent.operation {
            Some(locator.validated()?)
        } else {
            intent.operation.validate()?;
            None
        };
        validate_original_context(owner, name, &intent)?;
        // The proof itself contains its exact check and basis; equality above
        // ensures the same complete historical validation inputs on every hit.
        let proof_bytes = encode(&intent.authority)?;
        if !proofs.contains(&proof_bytes) {
            intent
                .authority
                .validate_historical(intent.operation.check(), intent.operation.basis())?;
            proofs.insert(proof_bytes);
        }
        let d = Decision {
            intent_fingerprint: fp(intent.operation.schema(), &intent)?,
            output: if let Some(item) = item {
                item_output(&item)?
            } else {
                output(&intent.operation)?
            },
        };
        let value = ExpectedRecords {
            intent_bytes_fingerprint:crate::DefinitionFingerprint::from_bytes(bytes).to_string(),
            publication: encode(&Publication {
                intent_fingerprint: d.intent_fingerprint.clone(),
                target: "terminal.json".into(),
            })?,
            noncommit: encode(&Publication {
                intent_fingerprint: d.intent_fingerprint.clone(),
                target: "positively_not_committed".into(),
            })?,
            intent,
            snapshot: encode(&d.output)?,
            terminal: encode(&terminal(&d)?)?,
            decision: encode(&d)?,
            output: d.output,
        };
        // Complete all bounds before making even the pure artifact reusable.
        let value = Rc::new(value);
        self.records.insert(bytes.to_vec(), Rc::clone(&value));
        Ok(value)
    }
}
fn validate_original_context(
    owner: &store::Owner,
    name: &str,
    intent: &Intent,
) -> Result<(), LocalAuthorityRefusalV1> {
    if intent.schema != intent.operation.schema()
        || name.strip_prefix("accept-").unwrap_or(name) != intent.operation.name()
        || intent.owner_identity != owner.identity
        || intent.operation.project() != owner.project
        || intent.authority.locator() != intent.operation.check()
        || intent.authority.current_basis() != intent.operation.basis()
    {
        return Err(refuse(
            "work_original_proof",
            "exact original owner/request/authority closure required",
        ));
    }
    Ok(())
}

fn output(op: &Operation) -> Result<Output, LocalAuthorityRefusalV1> {
    Ok(match op {
        Operation::Kind(locator) => Output::Kind(WorkKindReceiptV1::from_locator(locator)?),
        Operation::Grant(locator) => Output::Grant(WorkGrantV1 {
            locator: locator.clone(),
            assurance: "local_advisory_claims".into(),
            establishment_rule: WORK_REGISTRATION_RULE_V1.into(),
        }),
        Operation::Item(locator) => item_output(&locator.validated()?)?,
        Operation::Revoke(locator) => Output::Revoke(WorkRevocationReceiptV1 {
            locator: locator.clone(),
            effect: "revoke_work_registration".into(),
        }),
    })
}
fn item_output(
    item: &super::types::ValidatedWorkRequest<'_>,
) -> Result<Output, LocalAuthorityRefusalV1> {
    let locator = item.locator();
    let composition = item
        .effective_input()
        .composition
        .explicit_composition()
        .ok_or_else(|| codec("expanded composition"))?;
    let descriptor = composition.descriptor.clone();
    let descriptor_bytes = descriptor.canonical_json_bytes()?;
    let snapshot = WorkSnapshotV1 {
        version: if !locator.input.composition.retained() {
            "3"
        } else if composition.role.is_some() || locator.input.display_level.is_some() {
            "2"
        } else {
            "1"
        }
        .into(),
        project_identity: locator.grant.project().into(),
        item_identity: locator.item_identity.clone(),
        item_key: locator.input.item_key.clone(),
        label: locator.input.label.clone(),
        question: locator.input.question.clone(),
        display_level: locator.input.display_level.clone(),
        role: composition.role.clone(),
        admitted_kind: locator.input.composition.submitted_kind().cloned(),
        state: "open".into(),
        descriptor,
        descriptor_fingerprint: crate::DefinitionFingerprint::from_bytes(&descriptor_bytes)
            .to_string(),
        descriptor_bytes,
        registration_request_fingerprint: locator.fingerprint.clone(),
    };
    let receipt_fingerprint = fp(
        if locator.input.composition.retained() {
            "handbook.work-registration-receipt@1.0.0"
        } else {
            "handbook.work-registration-receipt@2.0.0"
        },
        &(
            locator,
            &snapshot,
            "register_open_leaf",
            "local_advisory_claims",
        ),
    )?;
    Ok(Output::Item(WorkRegistrationReceiptV1 {
        locator: locator.clone(),
        snapshot,
        effect: "register_open_leaf".into(),
        assurance: "local_advisory_claims".into(),
        receipt_fingerprint,
    }))
}

#[cfg(test)]
mod artifact_tests {
    use super::*;

    #[test]
    fn derived_item_output_preserves_original_retained_receipt_bytes() {
        let original = WorkRequestLocatorV1::from_json_bytes(include_bytes!(
            "../../../sdk/tests/fixtures/work_state_v1/committed-request.json"
        ))
        .unwrap();
        let Output::Item(receipt) = item_output(&original.validated().unwrap()).unwrap() else {
            panic!("Item derivation must produce its receipt");
        };
        assert_eq!(
            encode(&receipt).unwrap(),
            include_bytes!("../../../sdk/tests/fixtures/work_state_v1/receipt.json")
        );
    }
}
fn decision(intent: &Intent) -> Result<Decision, LocalAuthorityRefusalV1> {
    Ok(Decision {
        intent_fingerprint: fp(intent.operation.schema(), intent)?,
        output: output(&intent.operation)?,
    })
}
fn terminal(d: &Decision) -> Result<Terminal, LocalAuthorityRefusalV1> {
    Ok(Terminal {
        decision_fingerprint: fp(
            match &d.output {
                Output::Kind(_) => "handbook.work-decision@2.0.0",
                Output::Item(v) if !v.locator.input.composition.retained() => {
                    "handbook.work-decision@2.0.0"
                }
                _ => "handbook.work-decision@1.0.0",
            },
            d,
        )?,
        effect: "committed".into(),
    })
}
fn publication(intent: &Intent) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
    encode(&Publication {
        intent_fingerprint: fp(intent.operation.schema(), intent)?,
        target: "terminal.json".into(),
    })
}
fn noncommit(intent: &Intent) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
    encode(&Publication {
        intent_fingerprint: fp(intent.operation.schema(), intent)?,
        target: "positively_not_committed".into(),
    })
}
fn parsed(
    owner: &store::Owner,
    name: &str,
    files: &BTreeMap<String, Vec<u8>>,
    artifacts: &mut Artifacts,
    proofs: &mut std::collections::BTreeSet<Vec<u8>>,
) -> Result<Parsed, LocalAuthorityRefusalV1> {
    let expected = artifacts.expected(
        owner,
        name,
        files.get("intent.json").ok_or_else(|| {
            refuse(
                "work_partial_inventory",
                "transaction lacks original intent",
            )
        })?,
        proofs,
    )?;
    let intent = &expected.intent;
    let staging = name.starts_with("accept-");
    let attempted = files.contains_key("attempted.json");
    let never = files.contains_key("not-attempted.json");
    if attempted == never && !(staging && files.len() == 1) {
        return Err(refuse(
            "work_publication_proof",
            "missing/conflicting publication checkpoint is uncertainty, never proof of noncommit",
        ));
    }
    for (file, expected) in [
        ("not-attempted.json", &expected.publication),
        ("attempted.json", &expected.publication),
        ("decision.json", &expected.decision),
        ("snapshot.json", &expected.snapshot),
        ("terminal.pending", &expected.terminal),
        ("terminal.json", &expected.terminal),
        ("noncommit.json", &expected.noncommit),
    ] {
        if files.get(file).is_some_and(|v| v != expected) {
            return Err(refuse(
                "work_history_integrity",
                "exact retained decision/snapshot/receipt/terminal/checkpoint bytes changed",
            ));
        }
    }
    let committed = files.contains_key("terminal.json");
    let nc = files.contains_key("noncommit.json");
    if (files.contains_key("snapshot.json") || files.contains_key("terminal.pending") || attempted)
        && !files.contains_key("decision.json")
        || committed
            && (!attempted
                || !files.contains_key("snapshot.json")
                || files.contains_key("terminal.pending"))
        || nc && (attempted || committed)
        || staging && (attempted || committed || nc || files.contains_key("decision.json"))
    {
        return Err(refuse(
            "work_prefix_integrity",
            "unsupported/partial/conflicting durable prefix",
        ));
    }
    Ok(Parsed {
        intent: intent.clone(),
        output: expected.output.clone(),
        committed,
        noncommit: nc,
        attempted,
        staging,
    })
}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Header {name:String,intent_fingerprint:String,staging:bool,files:Vec<String>}
#[derive(Clone,Debug,Default,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkStorageStatisticsV1 {pub live_records:u64,pub retained_original_outcomes:u64,pub retained_record_bytes:u64,pub database_publications:u64}
#[derive(Clone)]
struct Stored {staging:bool,files:BTreeMap<String,Vec<u8>>}
struct Inventory {transactions:BTreeMap<String,BTreeMap<String,Vec<u8>>>}
fn index_kind(binding:&WorkKindBindingV1)->Result<String,LocalAuthorityRefusalV1>{Ok(format!("kind/{}",fp("handbook.work-kind-index@1",&binding.reference)?))}
fn snapshot_record(snapshot:&dyn file_native::ReadView,name:&str)->Result<Option<Stored>,LocalAuthorityRefusalV1>{
    let Some(bytes)=snapshot.get(&format!("request/{name}")).map_err(db_error)? else{return Ok(None)};
    let header:Header=decode(&bytes)?;
    if header.name!=name||header.files.len()>8||header.files.windows(2).any(|w|w[0]>=w[1]){return Err(refuse("work_original_proof","exact indexed request header required"));}
    let mut files=BTreeMap::new();
    for file in header.files {if !["intent.json","not-attempted.json","attempted.json","decision.json","snapshot.json","terminal.pending","terminal.json","noncommit.json"].contains(&file.as_str()){return Err(codec("closed record"));}
        let bytes=snapshot.get(&format!("record/{name}/{file}")).map_err(db_error)?.ok_or_else(||refuse("work_proof_missing","positively declared original record missing; loss is not absence"))?;
        files.insert(file,bytes);
    }
    let intent=files.get("intent.json").ok_or_else(||refuse("work_proof_missing","positive original intent required"))?;
    if crate::DefinitionFingerprint::from_bytes(intent).as_str()!=header.intent_fingerprint{return Err(refuse("work_original_proof","exact original intent identity required"));}
    Ok(Some(Stored {staging:header.staging,files}))
}
fn read_name(snapshot:&dyn file_native::ReadView,key:&str)->Result<Option<String>,LocalAuthorityRefusalV1>{
    snapshot.get(key).map_err(db_error)?.map(|b|String::from_utf8(b).map_err(codec)).transpose()
}
fn scan(lock:&WorkLock<'_>,op:&Operation)->Result<(Inventory,Vec<Parsed>),LocalAuthorityRefusalV1>{scan_with_artifacts(lock,op,&mut Artifacts::default())}
fn scan_kind(lock:&WorkLock<'_>,binding:&WorkKindBindingV1)->Result<Vec<Parsed>,LocalAuthorityRefusalV1>{
    lock.db.recover().map_err(db_error)?;
    let operation=lock.db.with_current(|snapshot| ->Result<Option<Operation>,LocalAuthorityRefusalV1>{
        let Some(name)=read_name(snapshot,&index_kind(binding)?)? else{return Ok(None)};
        let row=snapshot_record(snapshot,&name)?.ok_or_else(||refuse("work_kind_history_missing","positive version index has no original request"))?;
        let intent:Intent=decode(&row.files["intent.json"])?;Ok(Some(intent.operation))
    }).map_err(db_error)??;
    match operation {Some(op)=>Ok(scan(lock,&op)?.1),None=>Ok(Vec::new())}
}
fn scan_item(lock:&WorkLock<'_>,identity:&str)->Result<Vec<Parsed>,LocalAuthorityRefusalV1>{
    let operation=lock.db.with_current(|snapshot| ->Result<Option<Operation>,LocalAuthorityRefusalV1>{
        let Some(name)=read_name(snapshot,&format!("item/{identity}"))? else{return Ok(None)};
        let row=snapshot_record(snapshot,&name)?.ok_or_else(||refuse("work_item_history_missing","positive item index has no original request"))?;
        let intent:Intent=decode(row.files.get("intent.json").ok_or_else(||codec("intent"))?)?;Ok(Some(intent.operation))
    }).map_err(db_error)??;
    match operation {Some(op)=>Ok(scan(lock,&op)?.1),None=>Ok(Vec::new())}
}
fn scan_with_artifacts(lock:&WorkLock<'_>,op:&Operation,artifacts:&mut Artifacts)->Result<(Inventory,Vec<Parsed>),LocalAuthorityRefusalV1>{
    lock.validate()?;
    lock.db.recover().map_err(db_error)?;
    let result=lock.db.with_current(|snapshot| {
    let requested_name=op.name();
    let mut requested_item_route=None;
    let mut names=std::collections::BTreeSet::from([requested_name.clone()]);
    for key in ["admin/head","admin/pending"] {if let Some(name)=read_name(snapshot,key)? {names.insert(name);}}
    match op {
        Operation::Kind(v)=>{if let Some(name)=read_name(snapshot,&index_kind(&v.binding)?)?{names.insert(name);}},
        Operation::Item(v)=>{requested_item_route=read_name(snapshot,&format!("item/{}",v.item_identity))?;if let Some(name)=&requested_item_route{names.insert(name.clone());}if let Some(p)=&v.kind_provenance{names.insert(format!("kind-{}",p.establishment.locator.approval.request_key));}},
        Operation::Grant(v)=>{if let Some(prior)=&v.predecessor_grant_fingerprint {
            for key in [format!("grant/{prior}"),format!("successor/{prior}"),format!("revocation/{prior}")] {
                if let Some(name)=read_name(snapshot,&key)?{names.insert(name);}
            }
        }},
        _=>{},
    }
    let grant=match op {Operation::Kind(_)=>None,_=>Some(op.grant())};
    if let Some(grant)=grant {
        names.insert(format!("grant-{}",grant.terms.request_key));
        for key in [format!("successor/{}",grant.fingerprint),format!("revocation/{}",grant.fingerprint)]{if let Some(name)=read_name(snapshot,&key)?{names.insert(name);}}
    }
    // One bounded set of selected dependencies, not recursive historical
    // enumeration. Each grant's historical authority proof is self-contained;
    // admission verifies the exact selected predecessor before changing head.
    let mut records=BTreeMap::new();
    for name in &names {if let Some(record)=snapshot_record(snapshot,name)?{records.insert(name.clone(),record);}else if name==&requested_name && requested_item_route.as_deref()==Some(name.as_str()){
        return Err(refuse("work_item_history_missing","positive item index has no original request"));
    }}
    let mut dependencies=Vec::new();
    for record in records.values(){let intent:Intent=decode(&record.files["intent.json"])?;match &intent.operation{
        Operation::Item(v)=>{dependencies.push(format!("grant-{}",v.grant.terms.request_key));if let Some(p)=&v.kind_provenance{dependencies.push(format!("kind-{}",p.establishment.locator.approval.request_key));}},
        Operation::Revoke(v)=>dependencies.push(format!("grant-{}",v.grant.terms.request_key)),
        Operation::Grant(v)=>{if let Some(prior)=&v.predecessor_grant_fingerprint {if let Some(name)=read_name(snapshot,&format!("grant/{prior}"))?{dependencies.push(name);}}},_=>{}
    }}
    for name in dependencies {if !records.contains_key(&name){let record=snapshot_record(snapshot,&name)?.ok_or_else(||refuse("work_dependency_missing","positive exact original dependency required"))?;records.insert(name,record);}}
    let mut transactions=BTreeMap::new();let mut rows=Vec::new();let mut proofs=std::collections::BTreeSet::new();
    artifacts.records.retain(|intent,_|records.values().any(|record|record.files.get("intent.json")==Some(intent)));
    for (name,record) in records {let name=if record.staging{format!("accept-{name}")}else{name};rows.push(parsed(&lock.owner,&name,&record.files,artifacts,&mut proofs)?);transactions.insert(name,record.files);}
    // Validate exact dependencies for the requested operation. Selected routing
    // and immutable hashes preserve admitted lineage without a full-chain walk.
    if let Some(row)=rows.iter().find(|r|r.intent.operation==*op){match &row.intent.operation {
        Operation::Item(v)=>{if !rows.iter().any(|r|r.committed&&matches!(&r.intent.operation,Operation::Grant(g) if g==&v.grant)){return Err(refuse("work_grant_history_missing","exact original committed grant required"));}if let Some(p)=&v.kind_provenance{if !rows.iter().any(|r|r.committed&&matches!(&r.output,Output::Kind(k) if k==&p.establishment)){return Err(refuse("work_kind_history_missing","exact original committed kind required"));}}},
        Operation::Revoke(v)=>{if !rows.iter().any(|r|r.committed&&matches!(&r.intent.operation,Operation::Grant(g) if g==&v.grant)){return Err(refuse("work_grant_history_missing","exact original committed grant required"));}},_=>{}
    }}
    lock.validate()?;Ok((Inventory {transactions},rows))
    }).map_err(db_error)??;
    for (name,files) in &result.0.transactions {
        let expected=artifacts.expected(&lock.owner,name,&files["intent.json"],&mut std::collections::BTreeSet::new())?;
        validate_record_native(lock,&expected,files)?;
    }
    Ok(result)
}

// Exact historical evidence only. These bytes never authorize a new Work effect.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkNativeProof {
    schema_ref: String,
    work_owner_identity: String,
    work_physical_root: String,
    work_request_name: String,
    work_intent_identity: String,
    work_intent_bytes_fingerprint: String,
    checkpoint_role: String,
    checkpoint_utf8: String,
    logical_target: String,
    attempted_record_key: String,
    terminal_record_key: String,
    terminal_bytes_fingerprint: String,
    nonapplication_ref: Option<NonapplicationReference>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct NonapplicationReference {
    // A bounded reference to the positively excluded base attempt, never a chain
    // of failed noncommit attempts. The immediately observed head binds this CAS.
    supporting: file_native::EvidenceReference,
    head: file_native::EvidenceReference,
    observed: file_native::Version,
}
fn native_scope(owner: &store::Owner, expected: &ExpectedRecords) -> Result<String,LocalAuthorityRefusalV1> {
    fp("handbook.work-native-publication-scope@1",&(&owner.identity,expected.intent.operation.name(),fp(expected.intent.operation.schema(),&expected.intent)?))
}
fn checkpoint_proof(lock:&WorkLock<'_>, expected:&ExpectedRecords, role:&str, nonapplication_ref:Option<NonapplicationReference>)->Result<WorkNativeProof,LocalAuthorityRefusalV1> {
    let name=expected.intent.operation.name();
    let checkpoint=if role=="positive-noncommit"{&expected.noncommit}else{&expected.publication};
    Ok(WorkNativeProof {
        schema_ref:"handbook.work-native-checkpoint-proof@1.0.0".into(),
        work_owner_identity:lock.owner.identity.clone(),
        work_physical_root:lock.root.governance_physical_root_fingerprint_v1().map_err(fs_error)?,
        work_request_name:name.clone(),
        work_intent_identity:fp(expected.intent.operation.schema(),&expected.intent)?,
        work_intent_bytes_fingerprint:expected.intent_bytes_fingerprint.clone(),
        checkpoint_role:role.into(),
        checkpoint_utf8:String::from_utf8(checkpoint.clone()).map_err(codec)?,
        logical_target:if role=="positive-noncommit"{"positively_not_committed"}else{"terminal.json"}.into(),
        attempted_record_key:format!("record/{name}/attempted.json"),
        terminal_record_key:format!("record/{name}/terminal.json"),
        terminal_bytes_fingerprint:crate::DefinitionFingerprint::from_bytes(&expected.terminal).to_string(),
        nonapplication_ref,
    })
}
fn validate_native_payload(lock:&WorkLock<'_>,expected:&ExpectedRecords,original:&file_native::OriginalEvidence)->Result<WorkNativeProof,LocalAuthorityRefusalV1>{
    let proof:WorkNativeProof=decode(&original.evidence.payload)?;
    let binding=&original.evidence.binding;
    if !["prefix-not-attempted","terminal-attempt","positive-noncommit"].contains(&proof.checkpoint_role.as_str())
        || proof!=checkpoint_proof(lock,expected,&proof.checkpoint_role,proof.nonapplication_ref.clone())?
        || original.evidence.scope!=native_scope(&lock.owner,expected)?
        || binding.namespace!=lock.owner.identity
        || binding.expected!=binding.predecessor
        || binding.key!=format!("{}:{}",expected.intent.operation.name(),binding.expected.sequence)
        || original.reference.key!=binding.key || original.reference.fingerprint!=binding.fingerprint
        || (proof.checkpoint_role=="positive-noncommit")!=proof.nonapplication_ref.is_some()
    {return Err(refuse("work_native_proof","exact original checkpoint and native ownership binding required"));}
    Ok(proof)
}
fn native_observation(lock:&WorkLock<'_>,expected:&ExpectedRecords)->Result<(file_native::EvidenceObservation,WorkNativeProof),LocalAuthorityRefusalV1>{
    let scope=native_scope(&lock.owner,expected)?;
    let observation=lock.db.reconcile_evidence(&scope,None).map_err(db_error)?;
    let original=observation.original.as_ref().ok_or_else(||refuse("work_native_proof_missing","accepted original native evidence missing; loss is not nonacceptance"))?;
    let proof=validate_native_payload(lock,expected,original)?;
    if let Some(exclusion)=&proof.nonapplication_ref {
        if original.evidence.previous.as_ref()!=Some(&exclusion.head)||original.evidence.binding.expected!=exclusion.observed {
            return Err(refuse("work_native_proof","noncommit must bind exact observed native predecessor/head"));
        }
        let base=lock.db.reconcile_evidence(&scope,Some(&exclusion.supporting.key)).map_err(db_error)?;
        let base=base.original.as_ref().ok_or_else(||refuse("work_native_proof_missing","positive nonapplication original missing"))?;
        let base_proof=validate_native_payload(lock,expected,base)?;
        if base.reference!=exclusion.supporting
            || base_proof.nonapplication_ref.is_some()
            || !((base_proof.checkpoint_role=="prefix-not-attempted" && [file_native::OriginalDisposition::Selected,file_native::OriginalDisposition::NotSelected].contains(&base.outcome.disposition))
                || (base_proof.checkpoint_role=="terminal-attempt" && base.outcome.disposition==file_native::OriginalDisposition::NotSelected))
        {return Err(refuse("work_native_proof","complete proven nonapplication required"));}
    }
    Ok((observation,proof))
}
fn validate_record_native(lock:&WorkLock<'_>,expected:&ExpectedRecords,files:&BTreeMap<String,Vec<u8>>)->Result<(),LocalAuthorityRefusalV1>{
    let (observation,proof)=native_observation(lock,expected)?;
    let original=observation.original.as_ref().expect("required above");
    let selected=original.outcome.disposition==file_native::OriginalDisposition::Selected;
    let terminal=files.contains_key("terminal.json");let noncommit=files.contains_key("noncommit.json");
    // No fallback from unknown attempted history to absence/not-attempted.
    let valid=match proof.checkpoint_role.as_str(){
        "terminal-attempt"=>if selected {terminal && files.get("attempted.json")==Some(&expected.publication) && files.get("terminal.json")==Some(&expected.terminal)}else{!terminal&&!noncommit&&!files.contains_key("attempted.json")&&files.get("not-attempted.json")==Some(&expected.publication)},
        "positive-noncommit"=>if selected {noncommit&&!terminal&&!files.contains_key("attempted.json")}else{!noncommit&&!terminal&&!files.contains_key("attempted.json")},
        "prefix-not-attempted"=>!terminal&&!noncommit&&!files.contains_key("attempted.json"),
        _=>false,
    };
    if !valid{return Err(refuse("work_native_proof","selected Work record and exact retained native attempt differ"));}
    lock.validate()
}
fn persist_record(lock:&WorkLock<'_>,op:&Operation,record:&Stored,expected:&ExpectedRecords,guard:impl Fn()->Result<(),LocalAuthorityRefusalV1>,native:&dyn Fn(WorkWriteBoundaryV1)->Result<(),LocalAuthorityRefusalV1>)->Result<(),LocalAuthorityRefusalV1>{
    lock.db.recover().map_err(db_error)?;
    let name=op.name();
    let nonapplication=if record.files.contains_key("noncommit.json"){
        let (observed,proof)=native_observation(lock,expected)?;
        let head=observed.original.as_ref().expect("required");
        let supporting=if let Some(prior)=proof.nonapplication_ref {prior.supporting}
            else if (proof.checkpoint_role=="prefix-not-attempted" && [file_native::OriginalDisposition::Selected,file_native::OriginalDisposition::NotSelected].contains(&head.outcome.disposition))
                ||(proof.checkpoint_role=="terminal-attempt" && head.outcome.disposition==file_native::OriginalDisposition::NotSelected){head.reference.clone()}
            else{return Err(refuse("work_publication_uncertain","positive native nonapplication required"));};
        Some(NonapplicationReference {supporting,head:head.reference.clone(),observed:observed.at})
    }else{None};
    let (old,mut statistics,version)=lock.db.with_current(|snapshot| ->Result<_,LocalAuthorityRefusalV1>{
        let old=snapshot_record(snapshot,&name)?;
        let statistics:WorkStorageStatisticsV1=snapshot.get("statistics").map_err(db_error)?.map(|b|decode(&b)).transpose()?.unwrap_or_default();
        Ok((old,statistics,snapshot.version().clone()))
    }).map_err(db_error)??;
    if nonapplication.as_ref().is_some_and(|p|p.observed!=version){return Err(refuse("work_database_conflict","observed nonapplication selector changed")); }
    let old_committed=old.as_ref().is_some_and(|r|r.files.contains_key("terminal.json"));
    let previous_bytes=old.as_ref().map(|r|r.files.values().map(|b|b.len() as u64).sum::<u64>()).unwrap_or(0);
    let next_bytes=record.files.values().map(|b|b.len() as u64).sum::<u64>();
    statistics.retained_record_bytes=statistics.retained_record_bytes.checked_sub(previous_bytes).and_then(|n|n.checked_add(next_bytes)).ok_or_else(||codec("accounting"))?;
    let committed=record.files.contains_key("terminal.json");let noncommit=record.files.contains_key("noncommit.json");
    if committed&&!old_committed{statistics.retained_original_outcomes=statistics.retained_original_outcomes.checked_add(1).ok_or_else(||codec("outcome count overflow"))?;if matches!(op,Operation::Item(_)){statistics.live_records=statistics.live_records.checked_add(1).ok_or_else(||codec("live count overflow"))?;}}
    let mut tx=file_native::Transaction::new(version.clone(),format!("{}:{}",name,version.sequence),Vec::new());
    let role=if committed {"terminal-attempt"}else if noncommit{"positive-noncommit"}else{"prefix-not-attempted"};
    if committed && (record.files.get("attempted.json")!=Some(&expected.publication)||record.files.get("terminal.json")!=Some(&expected.terminal)){
        return Err(refuse("work_native_proof","actual terminal transaction requires exact attempted/terminal records"));
    }
    tx.set_evidence(native_scope(&lock.owner,expected)?,encode(&checkpoint_proof(lock,expected,role,nonapplication)?)?).map_err(db_error)?;
    let header=Header {name:name.clone(),intent_fingerprint:crate::DefinitionFingerprint::from_bytes(&record.files["intent.json"]).to_string(),staging:record.staging,files:record.files.keys().cloned().collect()};
    tx.put(format!("request/{name}"),encode(&header)?).map_err(db_error)?;
    for (file,bytes) in &record.files {if old.as_ref().and_then(|r|r.files.get(file))!=Some(bytes){tx.put(format!("record/{name}/{file}"),bytes.clone()).map_err(db_error)?;}}
    if let Some(old)=old {for file in old.files.keys(){if !record.files.contains_key(file){tx.delete(format!("record/{name}/{file}")).map_err(db_error)?;}}}
    let mut route=|key:String,retain:bool|->Result<(),LocalAuthorityRefusalV1>{if retain{tx.put(key,name.as_bytes().to_vec()).map_err(db_error)}else{tx.delete(key).map_err(db_error)}};
    match op {
        Operation::Item(v)=>route(format!("item/{}",v.item_identity),!noncommit)?,
        Operation::Kind(v)=>route(index_kind(&v.binding)?,!noncommit)?,
        Operation::Grant(v)=>{route(format!("grant/{}",v.fingerprint),true)?;if let Some(prior)=&v.predecessor_grant_fingerprint {route(format!("successor/{prior}"),!noncommit)?;}if committed{route("admin/head".into(),true)?;}route("admin/pending".into(),!committed&&!noncommit)?;},
        Operation::Revoke(v)=>{route(format!("revocation/{}",v.grant.fingerprint),!noncommit)?;route("admin/pending".into(),!committed&&!noncommit)?;},
    }
    tx.put("statistics",encode(&statistics)?).map_err(db_error)?;
    lock.validate()?;
    let publication_guard=|| {guard()?;lock.validate()?;Ok(())};
    #[cfg(feature="test-support")]
    let result=lock.db.publish_guarded_with_boundary(tx,publication_guard,|at| {
        use file_native::PublicationBoundary as N;
        let at=match at {N::BeforeOwnership=>WorkWriteBoundaryV1::NativeBeforeOwnership,N::Owned=>WorkWriteBoundaryV1::NativeOwned,N::Outputs=>WorkWriteBoundaryV1::NativeOutputs,N::BeforeSelection=>WorkWriteBoundaryV1::NativeBeforeSelection,N::SelectorDisplaced=>WorkWriteBoundaryV1::NativeSelectorDisplaced,_=>return Ok(())};
        native(at)?;
        // PublicationAttempt is notified only AFTER complete native Owned proof,
        // BEFORE any user candidate output. Initial staging cannot noncommit.
        if committed && matches!(at,WorkWriteBoundaryV1::NativeOwned){native(WorkWriteBoundaryV1::PublicationAttempt)?;}
        Ok(())
    });
    #[cfg(not(feature="test-support"))]
    let result={let _=native;lock.db.publish_guarded(tx,publication_guard)};
    let result=result.map_err(|e|match e.downcast::<LocalAuthorityRefusalV1>(){Ok(refusal)=>*refusal,Err(e)=>db_error(e)})?;
    if result.disposition!=file_native::OriginalDisposition::Selected{return Err(refuse("work_database_conflict","exact selected native parent required; no automatic semantic retry"));}
    lock.validate()
}
fn reservation(op: &Operation, rows: &[Parsed]) -> Result<(), LocalAuthorityRefusalV1> {
    for row in rows {
        if row.intent.operation.name() == op.name() {
            if row.intent.operation != *op {
                return Err(refuse(
                    "work_request_conflict",
                    "original request key content differs",
                ));
            }
            continue;
        }
        // Positive noncommit releases the item/head reservation, never the
        // immutable request key or its original content.
        if row.noncommit {
            continue;
        }
        match (op, &row.intent.operation) {
            (Operation::Kind(v), Operation::Kind(old))
                if v.binding.reference == old.binding.reference =>
            {
                return Err(refuse("work_kind_version_reserved", "exact kind version belongs to its original request; reuse committed binding or recover original"));
            }
            (Operation::Item(v), Operation::Item(old)) if v.item_identity == old.item_identity => {
                return Err(refuse(
                    "work_item_reserved",
                    "item already committed or reserved by exact original request",
                ))
            }
            (Operation::Grant(_), Operation::Grant(_)) if !row.committed => {
                return Err(refuse(
                    "work_grant_reserved",
                    "recover pending original activation before another grant",
                ))
            }
            (
                Operation::Grant(_) | Operation::Revoke(_),
                Operation::Grant(_) | Operation::Revoke(_),
            ) if !row.committed => {
                return Err(refuse(
                    "work_administration_reserved",
                    "recover original pending authority operation before competing administration",
                ));
            }
            (Operation::Revoke(v), Operation::Revoke(old))
                if v.grant.fingerprint == old.grant.fingerprint =>
            {
                return Err(refuse(
                    "work_revoke_reserved",
                    "grant revocation belongs to original request",
                ))
            }
            _ => {}
        }
    }
    Ok(())
}
fn current_grant<'a>(
    grant: &WorkGrantLocatorV1,
    rows: &'a [Parsed],
) -> Result<&'a WorkGrantV1, LocalAuthorityRefusalV1> {
    let selected = rows
        .iter()
        .find_map(|r| match &r.output {
            Output::Grant(g) if r.committed && g.locator == *grant => Some(g),
            _ => None,
        })
        .ok_or_else(|| {
            refuse(
                "work_grant_unactivated",
                "local-check permission and candidate work terms do not authorize registration",
            )
        })?;
    if rows.iter().any(|r| {
        !r.noncommit
            && match &r.intent.operation {
                Operation::Grant(g) => {
                    g.predecessor_grant_fingerprint.as_ref() == Some(&grant.fingerprint)
                }
                Operation::Revoke(v) => v.grant.fingerprint == grant.fingerprint,
                _ => false,
            }
    }) {
        return Err(refuse(
            "work_grant_revoked_or_superseded",
            "revocation or explicit successor makes original grant unavailable for new effects",
        ));
    }
    Ok(selected)
}
fn kind_row<'a>(
    binding: &WorkKindBindingV1,
    rows: &'a [Parsed],
) -> Result<Option<&'a Parsed>, LocalAuthorityRefusalV1> {
    binding.validate()?;
    let row = rows.iter().find(|row| !row.noncommit && matches!(&row.output, Output::Kind(receipt) if receipt.binding.reference == binding.reference));
    if let Some(row) = row {
        if !matches!(&row.output, Output::Kind(receipt) if &receipt.binding == binding) {
            return Err(refuse(
                "work_kind_content_mismatch",
                "exact retained project/name/version/full-content binding required",
            ));
        }
    }
    Ok(row)
}
fn committed_kind<'a>(
    binding: &WorkKindBindingV1,
    rows: &'a [Parsed],
) -> Result<&'a WorkKindReceiptV1, LocalAuthorityRefusalV1> {
    match kind_row(binding, rows)? {
        Some(row) if row.committed => match &row.output {
            Output::Kind(receipt) => Ok(receipt),
            _ => unreachable!(),
        },
        _ => Err(refuse(
            "work_kind_uncommitted",
            "only exact retained terminal-backed definition can expand new work",
        )),
    }
}
fn validate_kind_dependency(
    request: &super::types::ValidatedWorkRequest<'_>,
    rows: &[Parsed],
) -> Result<(), LocalAuthorityRefusalV1> {
    let locator = request.locator();
    if let Some(provenance) = &locator.kind_provenance {
        if committed_kind(&provenance.establishment.binding, rows)? != &provenance.establishment {
            return Err(refuse(
                "work_kind_provenance",
                "original establishment receipt and retained dependency differ",
            ));
        }
    }
    Ok(())
}
fn preflight(intent: &Intent) -> Result<(), LocalAuthorityRefusalV1> {
    encode(intent)?;
    let candidate = decision(intent)?;
    encode(&candidate)?;
    encode(&candidate.output)?;
    encode(&terminal(&candidate)?)?;
    publication(intent)?;
    noncommit(intent)?;
    Ok(())
}
fn fixed_terms_equal(a: &WorkRegistrationTermsV1, b: &WorkRegistrationTermsV1) -> bool {
    a.rule_ref == b.rule_ref
        && a.mode == b.mode
        && a.principal == b.principal
        && a.agent == b.agent
        && a.delegation == b.delegation
        && a.effect == b.effect
        && a.owner_write_reach == b.owner_write_reach
        && a.owner_read_reach == b.owner_read_reach
        && a.reserved_decisions == b.reserved_decisions
        && a.revocation_principal == b.revocation_principal
        && a.retention == b.retention
}
fn successor_compatible(
    grant: &WorkGrantLocatorV1,
    old: &WorkGrantLocatorV1,
    new_kind: &str,
    old_kind: &str,
) -> Result<(), LocalAuthorityRefusalV1> {
    if grant.predecessor_grant_fingerprint.as_ref() != Some(&old.fingerprint)
        || grant.check != old.check
        || !fixed_terms_equal(&old.terms, &grant.terms)
    {
        return Err(refuse(
            "work_successor_binding",
            "exact predecessor, original lineage and fixed authority terms required",
        ));
    }
    match grant.change {
        Some(WorkAuthorityChangeV1::ScopeChange) if old.basis == grant.basis && old_kind == new_kind => Ok(()),
        Some(WorkAuthorityChangeV1::EstablishFromDeferred) | None
            if old.basis != grant.basis && old_kind == "deferred" && new_kind == "established_from_deferred"
                && old.terms.scope.equivalent(&grant.terms.scope) => Ok(()),
        _ => Err(refuse("work_reestablishment", "same-basis explicit scope change or unchanged-scope explicit D-to-G required; no combined change/restoration")),
    }
}
fn activation_compatible(
    grant: &WorkGrantLocatorV1,
    rows: &[Parsed],
    kind: &str,
) -> Result<(), LocalAuthorityRefusalV1> {
    match &grant.predecessor_grant_fingerprint {
        None if !rows
            .iter()
            .any(|r| r.committed && matches!(r.output, Output::Grant(_)))
            && matches!(grant.change, Some(WorkAuthorityChangeV1::Initial) | None) =>
        {
            Ok(())
        }
        Some(previous) => {
            let old = rows
                .iter()
                .find_map(|r| match &r.output {
                    Output::Grant(g) if r.committed && &g.locator.fingerprint == previous => {
                        Some(&g.locator)
                    }
                    _ => None,
                })
                .ok_or_else(|| {
                    refuse(
                        "work_prior_grant_missing",
                        "exact committed predecessor required",
                    )
                })?;
            current_grant(old, rows)?;
            successor_compatible(grant, old, kind, &relation_kind(&old.basis, rows, old)?)
        }
        _ => Err(refuse(
            "work_replacement_unsupported",
            "initial authority already exists; explicit latest predecessor required",
        )),
    }
} // The retained proof reports the native scope relation's actual fixed tag.
fn relation_kind(
    _basis: &LocalCheckCurrentBasisV1,
    rows: &[Parsed],
    grant: &WorkGrantLocatorV1,
) -> Result<String, LocalAuthorityRefusalV1> {
    rows.iter()
        .find(|r| matches!(&r.intent.operation,Operation::Grant(g) if g==grant))
        .ok_or_else(|| refuse("work_prior_proof", "prior grant proof missing"))?
        .intent
        .authority
        .relation_kind()
        .map(|k| k.as_str().to_owned())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkWriteBoundaryV1 {
    BootstrapIntent,
    BeforeIntent,
    Reservation,
    AcceptanceCandidate,
    StagedIntent,
    AcceptanceReady,
    Intent,
    Decision,
    Snapshot,
    PublicationAttempt,
    NativeBeforeOwnership,
    NativeOwned,
    NativeOutputs,
    NativeBeforeSelection,
    NativeSelectorDisplaced,
    Terminal,
    Reply,
}
pub struct NativeWorkStateV1;
impl NativeWorkStateV1 {
    /// Bounded same-WorkFs evidence inspection for native interruption proof.
    /// No ambient backend, permission, or production semantic entrypoint.
    #[cfg(any(test,feature="test-support"))]
    #[doc(hidden)]
    pub fn checkpoint_evidence_for_test(repo:impl AsRef<Path>,locator:&WorkRequestLocatorV1,key:Option<&str>)->Result<serde_json::Value,LocalAuthorityRefusalV1>{
        locator.validate()?;
        let root=RetainedRepositoryRootV1::open_exact(repo).map_err(fs_error)?;
        let lock=store::open(&root)?;lock.db.recover().map_err(db_error)?;
        let record=lock.db.with_current(|view|snapshot_record(view,&Operation::Item(locator.clone()).name())).map_err(db_error)??.ok_or_else(||refuse("work_proof_missing","original Work record required"))?;
        let expected=Artifacts::default().expected(&lock.owner,&Operation::Item(locator.clone()).name(),&record.files["intent.json"],&mut std::collections::BTreeSet::new())?;
        if expected.intent.operation!=Operation::Item(locator.clone()){return Err(refuse("work_request_conflict","exact saved original required"));}
        let observed=lock.db.reconcile_evidence(&native_scope(&lock.owner,&expected)?,key).map_err(db_error)?;
        let original=observed.original.as_ref().ok_or_else(||refuse("work_native_proof_missing","original native evidence required"))?;
        validate_native_payload(&lock,&expected,original)?;lock.validate()?;
        Ok(serde_json::json!({"at":observed.at,"reference":original.reference,"outcome":original.outcome,"evidence":original.evidence}))
    }
    /// Consumes a historical view while retaining its exact native lease/root.
    /// The work-owner lock is released before callback execution. This view
    /// cannot escape and supplies no current authority or permission.
    pub fn with_storage_snapshot<R>(repo:impl AsRef<Path>,read:impl FnOnce(&WorkStorageSnapshotV1<'_>)->R)->Result<R,LocalAuthorityRefusalV1>{
        let root=RetainedRepositoryRootV1::open_exact(repo).map_err(fs_error)?;
        let lock=store::open(&root)?;let snapshot=lock.db.snapshot().map_err(db_error)?;let owner=lock.owner.clone();let owner_proof=store::owner_proof(&root)?;lock.validate()?;drop(lock);
        let view=WorkStorageSnapshotV1 {snapshot,root:&root,owner,owner_proof};
        let result=read(&view);view.owner_proof.validate().map_err(fs_error)?;store::validate_reader_owner(&root,&view.owner)?;Ok(result)
    }
    /// Counts actual committed Work originals and live immutable items,
    /// separately from the database's internal prefix publications.
    pub fn storage_statistics(repo: impl AsRef<Path>) -> Result<WorkStorageStatisticsV1,LocalAuthorityRefusalV1> {
        let root=RetainedRepositoryRootV1::open_exact(repo).map_err(fs_error)?;
        let lock=store::open(&root)?;let mut result:WorkStorageStatisticsV1=lock.db.get("statistics").map_err(db_error)?.map(|b|decode(&b)).transpose()?.unwrap_or_default();
        result.database_publications=lock.db.version().map_err(db_error)?.sequence;
        lock.validate()?;Ok(result)
    }
    /// Advances physical retirement only; never changes a Work request, grant,
    /// original outcome or pinned reader's selected meaning.
    pub fn maintain_storage(repo: impl AsRef<Path>,budget:file_native::MaintenanceBudget) -> Result<file_native::MaintenanceProgress,LocalAuthorityRefusalV1> {
        let root=RetainedRepositoryRootV1::open_exact(repo).map_err(fs_error)?;
        let lock=store::open(&root)?;lock.db.recover().map_err(db_error)?;let progress=lock.db.maintain(budget).map_err(db_error)?;lock.validate()?;Ok(progress)
    }
    pub fn prepare_kind(
        repo: impl AsRef<Path>,
        check: &LocalCheckLocatorV1,
        basis: LocalCheckCurrentBasisV1,
        definition: WorkKindDefinitionV1,
        approval: WorkKindApprovalV1,
    ) -> Result<PreparedWorkKindV1, LocalAuthorityRefusalV1> {
        let project = check
            .predecessor
            .charter
            .binding
            .initialization_claim
            .proposed_repository_identity();
        let binding = definition.binding(project)?;
        let mut locator = WorkKindLocatorV1 {
            check: check.clone(),
            basis,
            definition,
            approval,
            binding,
            fingerprint: String::new(),
        };
        locator.fingerprint = locator.identity()?;
        locator.validate()?;
        with_work_registration_current(repo, check, &locator.basis, |held| {
            let operation = Operation::Kind(locator.clone());
            let owner_identity =
                store::owner_identity(&store::BootstrapOrigin::Kind(locator.clone()))?;
            preflight(&Intent {
                schema: operation.schema().into(),
                owner_identity,
                operation: operation.clone(),
                authority: held.capture_proof()?,
            })?;
            if store::optional(held.root(), ".handbook-work-state.v1.json")?.is_some() {
                let lock = store::open(held.root())?;
                let (_, rows) = scan(&lock,&operation)?;
                reservation(&operation, &rows)?;
            } else if store::optional(held.root(), ".handbook-work-state-bootstrap.v1.json")?
                .is_some()
                || held
                    .root()
                    .list_directory_bounded(".handbook", 4096)
                    .map_err(fs_error)?
                    .iter()
                    .any(|e| matches!(e.name(), "work-state" | "work-state-bootstrap"))
            {
                return Err(refuse(
                    "work_bootstrap_reserved",
                    "positive original owner/bootstrap footprint requires original recovery",
                ));
            }
            Ok(())
        })?;
        Ok(PreparedWorkKindV1 { locator })
    }
    pub fn establish_kind(
        repo: impl AsRef<Path>,
        prepared: PreparedWorkKindV1,
    ) -> WorkExecutionV1<WorkKindReceiptV1> {
        project_kind(execute(
            repo.as_ref(),
            Operation::Kind(prepared.locator),
            false,
            &mut |_| Ok(()),
        ))
    }
    pub fn resolve_kind_request(
        repo: impl AsRef<Path>,
        locator: &WorkKindLocatorV1,
    ) -> WorkExecutionV1<WorkKindReceiptV1> {
        project_kind(resolve(repo.as_ref(), Operation::Kind(locator.clone())))
    }
    pub fn recover_kind_request(
        repo: impl AsRef<Path>,
        locator: &WorkKindLocatorV1,
    ) -> WorkExecutionV1<WorkKindReceiptV1> {
        project_kind(execute(
            repo.as_ref(),
            Operation::Kind(locator.clone()),
            true,
            &mut |_| Ok(()),
        ))
    }
    pub fn current_kind(
        repo: impl AsRef<Path>,
        binding: &WorkKindBindingV1,
    ) -> Result<WorkCurrentKindV1, LocalAuthorityRefusalV1> {
        binding.validate()?;
        let root = RetainedRepositoryRootV1::open_exact(repo).map_err(fs_error)?;
        let lock = store::open(&root)?;
        if lock.owner.project != binding.project_identity {
            return Err(refuse(
                "work_kind_project",
                "exact retained same-project kind required",
            ));
        }
        let rows = scan_kind(&lock,binding)?;
        let result = match kind_row(binding, &rows)? {
            None => WorkCurrentKindV1::Absent,
            Some(row) => match &row.output {
                Output::Kind(receipt) if row.committed => {
                    WorkCurrentKindV1::Committed(receipt.clone())
                }
                Output::Kind(receipt) => WorkCurrentKindV1::Pending {
                    original_request: receipt.locator.clone(),
                },
                _ => unreachable!(),
            },
        };
        lock.validate()?;
        Ok(result)
    }
    #[cfg(any(test, feature = "test-support"))]
    #[doc(hidden)]
    pub fn establish_kind_with_boundary_for_test(
        repo: impl AsRef<Path>,
        prepared: PreparedWorkKindV1,
        mut boundary: impl FnMut(WorkWriteBoundaryV1) -> Result<(), LocalAuthorityRefusalV1>,
    ) -> WorkExecutionV1<WorkKindReceiptV1> {
        project_kind(execute(
            repo.as_ref(),
            Operation::Kind(prepared.locator),
            false,
            &mut boundary,
        ))
    }
    #[cfg(any(test, feature = "test-support"))]
    #[doc(hidden)]
    pub fn recover_kind_with_boundary_for_test(
        repo: impl AsRef<Path>,
        locator: &WorkKindLocatorV1,
        mut boundary: impl FnMut(WorkWriteBoundaryV1) -> Result<(), LocalAuthorityRefusalV1>,
    ) -> WorkExecutionV1<WorkKindReceiptV1> {
        project_kind(execute(
            repo.as_ref(),
            Operation::Kind(locator.clone()),
            true,
            &mut boundary,
        ))
    }
    pub fn prepare_terms(
        repo: impl AsRef<Path>,
        check: &LocalCheckLocatorV1,
        basis: LocalCheckCurrentBasisV1,
        terms: WorkRegistrationTermsV1,
        transition: WorkTermsTransitionV1<'_>,
    ) -> Result<PreparedWorkGrantV1, LocalAuthorityRefusalV1> {
        if !matches!(terms.scope, WorkRegistrationScopeV1::Explicit(_)) {
            return Err(refuse("work_explicit_scope_required", "new human approval must supply finite explicit scope; retained singleton is decoding only"));
        }
        let (change, predecessor) = match transition {
            WorkTermsTransitionV1::Initial => (WorkAuthorityChangeV1::Initial, None),
            WorkTermsTransitionV1::ScopeChange(g) => (WorkAuthorityChangeV1::ScopeChange, Some(g)),
            WorkTermsTransitionV1::EstablishFromDeferred(g) => {
                (WorkAuthorityChangeV1::EstablishFromDeferred, Some(g))
            }
        };
        let mut locator = WorkGrantLocatorV1 {
            check: check.clone(),
            basis,
            terms,
            predecessor_grant_fingerprint: predecessor.map(|g| g.fingerprint.clone()),
            change: Some(change),
            fingerprint: String::new(),
        };
        locator.fingerprint = locator.identity()?;
        locator.validate()?;
        with_work_registration_current(repo, check, &locator.basis, |held| {
            let proof = held.capture_proof()?;
            let kind = proof.relation_kind()?;
            match store::optional(held.root(), ".handbook-work-state.v1.json")? {
                Some(_) => {
                    let lock = store::open(held.root())?;
                    let (_, rows) = scan(&lock,&Operation::Grant(locator.clone()))?;
                    reservation(&Operation::Grant(locator.clone()), &rows)?;
                    activation_compatible(&locator, &rows, kind.as_str())?;
                }
                None => {
                    if predecessor.is_some() {
                        return Err(refuse(
                            "work_prior_grant_missing",
                            "prior work grant requires retained owner truth",
                        ));
                    }
                    if store::optional(held.root(), ".handbook-work-state-bootstrap.v1.json")?
                        .is_some()
                        || held
                            .root()
                            .list_directory_bounded(".handbook", 4096)
                            .map_err(fs_error)?
                            .iter()
                            .any(|e| matches!(e.name(), "work-state" | "work-state-bootstrap"))
                    {
                        return Err(refuse("work_bootstrap_reserved", "positive prior bootstrap/owner footprint requires original recovery; never fresh establishment"));
                    }
                }
            }
            Ok(())
        })?;
        Ok(PreparedWorkGrantV1 { locator })
    }
    pub fn activate_terms(
        repo: impl AsRef<Path>,
        prepared: PreparedWorkGrantV1,
    ) -> WorkExecutionV1<WorkGrantV1> {
        project_grant(execute(
            repo.as_ref(),
            Operation::Grant(prepared.locator),
            false,
            &mut |_| Ok(()),
        ))
    }
    pub fn resolve_terms_request(
        repo: impl AsRef<Path>,
        locator: &WorkGrantLocatorV1,
    ) -> WorkExecutionV1<WorkGrantV1> {
        project_grant(resolve(repo.as_ref(), Operation::Grant(locator.clone())))
    }
    pub fn recover_terms_request(
        repo: impl AsRef<Path>,
        locator: &WorkGrantLocatorV1,
    ) -> WorkExecutionV1<WorkGrantV1> {
        project_grant(execute(
            repo.as_ref(),
            Operation::Grant(locator.clone()),
            true,
            &mut |_| Ok(()),
        ))
    }
    pub fn resolve_current_terms(
        repo: impl AsRef<Path>,
        locator: &WorkGrantLocatorV1,
        basis: &LocalCheckCurrentBasisV1,
    ) -> Result<WorkGrantV1, LocalAuthorityRefusalV1> {
        locator.validate()?;
        if basis != &locator.basis {
            return Err(refuse(
                "work_basis_stale",
                "explicit new G activation required; no automatic carry-forward",
            ));
        }
        with_work_registration_current(repo, &locator.check, basis, |held| {
            let lock = store::open(held.root())?;
            let (_, rows) = scan(&lock,&Operation::Grant(locator.clone()))?;
            let out = current_grant(locator, &rows)?.clone();
            held.validate_current()?;
            lock.validate()?;
            Ok(out)
        })
    }
    /// Binds exact submitted meaning and a complete original recovery locator.
    /// Explicit input is pure; kind input reads one retained committed closure.
    /// Current authority, conflicts and capacity are checked at invocation.
    pub fn prepare_registration(
        repo: impl AsRef<Path>,
        grant: &WorkGrantLocatorV1,
        basis: &LocalCheckCurrentBasisV1,
        input: WorkRegistrationInputV1,
    ) -> Result<PreparedWorkRegistrationV1, LocalAuthorityRefusalV1> {
        let bound = WorkRequestBinding::new(grant, basis, input)?;
        let mut kind_provenance = None;
        if let Some(binding) = bound.submitted_kind() {
            if binding.project_identity != bound.project() {
                return Err(refuse(
                    "work_kind_provenance",
                    "exact same-project committed kind and expansion required",
                ));
            }
            let root = RetainedRepositoryRootV1::open_exact(repo).map_err(fs_error)?;
            let lock = store::open(&root)?;
            let rows = scan_kind(&lock,binding)?;
            let receipt = committed_kind(binding, &rows)?;
            kind_provenance = Some(WorkKindProvenanceV1 {
                establishment: receipt.clone(),
                expanded: WorkCompositionV1 {
                    descriptor: receipt.definition.template.descriptor.clone(),
                    kind: None,
                    role: bound.role().cloned(),
                },
            });
            lock.validate()?;
        }
        bound.finish(kind_provenance)
    }
    pub fn register(
        repo: impl AsRef<Path>,
        prepared: PreparedWorkRegistrationV1,
    ) -> WorkExecutionV1<WorkRegistrationReceiptV1> {
        project_item(execute(
            repo.as_ref(),
            Operation::Item(prepared.locator),
            false,
            &mut |_| Ok(()),
        ))
    }
    pub fn resolve_request(
        repo: impl AsRef<Path>,
        locator: &WorkRequestLocatorV1,
    ) -> WorkExecutionV1<WorkRegistrationReceiptV1> {
        project_item(resolve(repo.as_ref(), Operation::Item(locator.clone())))
    }
    pub fn recover_request(
        repo: impl AsRef<Path>,
        locator: &WorkRequestLocatorV1,
    ) -> WorkExecutionV1<WorkRegistrationReceiptV1> {
        project_item(execute(
            repo.as_ref(),
            Operation::Item(locator.clone()),
            true,
            &mut |_| Ok(()),
        ))
    }
    pub fn current_item(
        repo: impl AsRef<Path>,
        grant: &WorkGrantLocatorV1,
        item_key: &str,
    ) -> Result<WorkCurrentItemV1, LocalAuthorityRefusalV1> {
        grant.validate()?;
        let identity = work_item_identity_v1(grant.project(), item_key)?;
        let root = RetainedRepositoryRootV1::open_exact(repo).map_err(fs_error)?;
        let lock = store::open(&root)?;
        if lock.owner.project != grant.project() {
            return Err(refuse("work_project", "exact registered project required"));
        }
        let rows = scan_item(&lock,&identity)?;
        let matches = rows
            .iter()
            .filter(|r| {
                !r.noncommit
                    && matches!(&r.output,Output::Item(v) if v.snapshot.item_identity==identity)
            })
            .collect::<Vec<_>>();
        if matches.len() > 1 {
            return Err(refuse(
                "work_item_conflict",
                "more than one authoritative registration for item",
            ));
        }
        lock.validate()?;
        Ok(match matches.first() {
            None => WorkCurrentItemV1::Absent,
            Some(row) => match &row.output {
                Output::Item(v) if row.committed => WorkCurrentItemV1::Open(v.snapshot.clone()),
                Output::Item(v) => WorkCurrentItemV1::Pending {
                    original_request: v.locator.clone(),
                },
                _ => unreachable!(),
            },
        })
    }
    pub fn completion_support(
        repo: impl AsRef<Path>,
        locator: &WorkRequestLocatorV1,
    ) -> Result<WorkCompletionSupportV1, LocalAuthorityRefusalV1> {
        let out = Self::resolve_request(repo, locator);
        let receipt = out.outcome.ok_or_else(|| {
            out.refusal.unwrap_or_else(|| {
                refuse(
                    "work_completion_pending",
                    "exact committed item required; completion is unsupported",
                )
            })
        })?;
        Ok(WorkCompletionSupportV1 {
            item: receipt.snapshot,
            completion_support: "unsupported".into(),
            assessment: "not_assessed".into(),
        })
    }
    pub fn prepare_revocation(
        repo: impl AsRef<Path>,
        grant: &WorkGrantLocatorV1,
        key: impl Into<String>,
        principal: impl Into<String>,
        source: impl Into<String>,
        instruction: impl Into<String>,
    ) -> Result<WorkRevokeLocatorV1, LocalAuthorityRefusalV1> {
        let mut v = WorkRevokeLocatorV1 {
            grant: grant.clone(),
            request_key: key.into(),
            principal: principal.into(),
            source_ref: source.into(),
            instruction: instruction.into(),
            fingerprint: String::new(),
        };
        v.fingerprint = v.identity()?;
        v.validate()?;
        let root = RetainedRepositoryRootV1::open_exact(repo).map_err(fs_error)?;
        let lock = store::open(&root)?;
        let (_, rows) = scan(&lock,&Operation::Revoke(v.clone()))?;
        reservation(&Operation::Revoke(v.clone()), &rows)?;
        current_grant(grant, &rows)?;
        Ok(v)
    }
    pub fn revoke_terms(
        repo: impl AsRef<Path>,
        locator: &WorkRevokeLocatorV1,
    ) -> WorkExecutionV1<WorkRevocationReceiptV1> {
        project_revoke(execute(
            repo.as_ref(),
            Operation::Revoke(locator.clone()),
            false,
            &mut |_| Ok(()),
        ))
    }
    pub fn resolve_revocation_request(
        repo: impl AsRef<Path>,
        locator: &WorkRevokeLocatorV1,
    ) -> WorkExecutionV1<WorkRevocationReceiptV1> {
        project_revoke(resolve(repo.as_ref(), Operation::Revoke(locator.clone())))
    }
    pub fn recover_revocation_request(
        repo: impl AsRef<Path>,
        locator: &WorkRevokeLocatorV1,
    ) -> WorkExecutionV1<WorkRevocationReceiptV1> {
        project_revoke(execute(
            repo.as_ref(),
            Operation::Revoke(locator.clone()),
            true,
            &mut |_| Ok(()),
        ))
    }
    #[cfg(any(test, feature = "test-support"))]
    #[doc(hidden)]
    pub fn register_with_boundary_for_test(
        repo: impl AsRef<Path>,
        prepared: PreparedWorkRegistrationV1,
        mut boundary: impl FnMut(WorkWriteBoundaryV1) -> Result<(), LocalAuthorityRefusalV1>,
    ) -> WorkExecutionV1<WorkRegistrationReceiptV1> {
        project_item(execute(
            repo.as_ref(),
            Operation::Item(prepared.locator),
            false,
            &mut boundary,
        ))
    }
    #[cfg(any(test, feature = "test-support"))]
    #[doc(hidden)]
    pub fn recover_with_boundary_for_test(
        repo: impl AsRef<Path>,
        locator: &WorkRequestLocatorV1,
        mut boundary: impl FnMut(WorkWriteBoundaryV1) -> Result<(), LocalAuthorityRefusalV1>,
    ) -> WorkExecutionV1<WorkRegistrationReceiptV1> {
        project_item(execute(repo.as_ref(),Operation::Item(locator.clone()),true,&mut boundary))
    }
    #[cfg(any(test, feature = "test-support"))]
    #[doc(hidden)]
    pub fn activate_with_boundary_for_test(
        repo: impl AsRef<Path>,
        prepared: PreparedWorkGrantV1,
        mut boundary: impl FnMut(WorkWriteBoundaryV1) -> Result<(), LocalAuthorityRefusalV1>,
    ) -> WorkExecutionV1<WorkGrantV1> {
        project_grant(execute(
            repo.as_ref(),
            Operation::Grant(prepared.locator),
            false,
            &mut boundary,
        ))
    }
    #[cfg(any(test, feature = "test-support"))]
    #[doc(hidden)]
    pub fn recover_terms_with_boundary_for_test(
        repo: impl AsRef<Path>,
        locator: &WorkGrantLocatorV1,
        mut boundary: impl FnMut(WorkWriteBoundaryV1) -> Result<(), LocalAuthorityRefusalV1>,
    ) -> WorkExecutionV1<WorkGrantV1> {
        project_grant(execute(
            repo.as_ref(),
            Operation::Grant(locator.clone()),
            true,
            &mut boundary,
        ))
    }
    #[cfg(any(test, feature = "test-support"))]
    #[doc(hidden)]
    pub fn revoke_with_boundary_for_test(
        repo: impl AsRef<Path>,
        locator: &WorkRevokeLocatorV1,
        mut boundary: impl FnMut(WorkWriteBoundaryV1) -> Result<(), LocalAuthorityRefusalV1>,
    ) -> WorkExecutionV1<WorkRevocationReceiptV1> {
        project_revoke(execute(
            repo.as_ref(),
            Operation::Revoke(locator.clone()),
            false,
            &mut boundary,
        ))
    }
}

fn resolve(repo: &Path, op: Operation) -> WorkExecutionV1<Output> {
    let result = (|| {
        op.validate()?;
        let root = RetainedRepositoryRootV1::open_exact(repo).map_err(fs_error)?;
        let lock = store::open(&root)?;
        let (_, rows) = scan(&lock,&op)?;
        if rows
            .iter()
            .any(|r| r.intent.operation.name() == op.name() && r.intent.operation != op)
        {
            return Ok(WorkExecutionV1::error(
                WorkDispositionV1::Refused,
                refuse(
                    "work_request_conflict",
                    "original request key content differs",
                ),
            ));
        }
        let row = rows.into_iter().find(|r| r.intent.operation == op);
        lock.validate()?;
        Ok(match row{None=>WorkExecutionV1::success(WorkDispositionV1::Unaccepted,None),Some(r) if r.committed=>WorkExecutionV1::success(WorkDispositionV1::Replayed,Some(r.output)),Some(r) if r.noncommit=>WorkExecutionV1::success(WorkDispositionV1::NonCommitted,None),Some(r) if r.attempted=>WorkExecutionV1::error(WorkDispositionV1::Uncertain,refuse("work_publication_uncertain","publication was attempted without exact terminal proof; retain original reservation")),Some(_)=>WorkExecutionV1::success(WorkDispositionV1::Pending,None)})
    })();
    result.unwrap_or_else(|e| WorkExecutionV1::error(WorkDispositionV1::Uncertain, e))
}
fn execute(
    repo: &Path,
    op: Operation,
    recover: bool,
    boundary: &mut dyn FnMut(WorkWriteBoundaryV1) -> Result<(), LocalAuthorityRefusalV1>,
) -> WorkExecutionV1<Output> {
    let item = match &op {
        Operation::Item(locator) => match locator.validated() {
            Ok(value) => Some(value),
            Err(e) => return WorkExecutionV1::error(WorkDispositionV1::Refused, e),
        },
        _ => {
            if let Err(e) = op.validate() {
                return WorkExecutionV1::error(WorkDispositionV1::Refused, e);
            }
            None
        }
    };
    // Exact committed/noncommit history has priority over current permission.
    // An absent owner is eligible only for original live grant establishment.
    let root = match RetainedRepositoryRootV1::open_exact(repo) {
        Ok(r) => r,
        Err(e) => return WorkExecutionV1::error(WorkDispositionV1::Refused, fs_error(e)),
    };
    let existed = match store::optional(&root, ".handbook-work-state.v1.json") {
        Ok(v) => v.is_some(),
        Err(e) => return WorkExecutionV1::error(WorkDispositionV1::Uncertain, e),
    };
    if recover && !existed {
        // Recovery cannot accept a new request. An unpublished owner bootstrap
        // may continue only from its exact already-retained original locator.
        match store::optional(&root, ".handbook-work-state-bootstrap.v1.json") {
            Ok(Some(bytes))
                if op
                    .bootstrap()
                    .is_some_and(|origin| encode(&origin).is_ok_and(|v| v == bytes)) => {}
            Ok(None) => {
                return WorkExecutionV1::error(
                    WorkDispositionV1::Refused,
                    refuse(
                        "work_owner_unestablished",
                        "recovery requires positive original acceptance/bootstrap proof",
                    ),
                )
            }
            Ok(Some(_)) => {
                return WorkExecutionV1::error(
                    WorkDispositionV1::Uncertain,
                    refuse(
                        "work_bootstrap_reserved",
                        "bootstrap belongs to exact original request",
                    ),
                )
            }
            Err(e) => return WorkExecutionV1::error(WorkDispositionV1::Uncertain, e),
        }
    }
    drop(root);
    if existed {
        if recover {
            // Physical recovery can select only metadata preserving the exact
            // predecessor user data and negative attempt proof, or finish an
            // already-selected candidate. It cannot install an unselected Work effect.
            let physical=(|| {let root=RetainedRepositoryRootV1::open_exact(repo).map_err(fs_error)?;let lock=store::open(&root)?;lock.db.recover().map_err(db_error)?;lock.validate()})();
            if let Err(e)=physical{return WorkExecutionV1::error(WorkDispositionV1::Uncertain,e);}
        }
        let history = resolve(repo, op.clone());
        if matches!(
            history.disposition,
            WorkDispositionV1::Replayed
                | WorkDispositionV1::NonCommitted
                | WorkDispositionV1::Uncertain
                | WorkDispositionV1::Refused
        ) {
            return history;
        }
        if recover && history.disposition == WorkDispositionV1::Unaccepted {
            return history;
        }
    } else if op.bootstrap().is_none() {
        return WorkExecutionV1::error(
            WorkDispositionV1::Refused,
            refuse(
                "work_owner_unestablished",
                "registration needs positive owner and activated work terms",
            ),
        );
    }
    let check = op.check().clone();
    let basis = op.basis().clone();
    let effects = std::cell::Cell::new(false);
    let boundary = std::cell::RefCell::new(boundary);
    let notify = |stage| boundary.borrow_mut()(stage);
    let consume = |held: &WorkRegistrationCurrentV1<'_>| {
        let lock = store::open(held.root())?;
        if recover {lock.db.recover().map_err(db_error)?;}
        let mut artifacts=Artifacts::default();
        let (inventory,mut rows)=scan_with_artifacts(&lock,&op,&mut artifacts)?;
        reservation(&op,&rows)?;
        if let Some(row)=rows.iter().find(|r|r.intent.operation==op){if row.committed{return Ok(row.output.clone());}if row.noncommit{return Err(refuse("work_request_noncommit","original request is durably not committed"));}if row.attempted{return Err(refuse("work_publication_uncertain","original attempted without terminal remains uncertain"));}}
        rows.retain(|r|r.intent.operation!=op);
        let validate=|rows:&[Parsed]|->Result<(),LocalAuthorityRefusalV1>{match &op {
            Operation::Kind(v)=>v.validate(),
            Operation::Grant(g)=>activation_compatible(g,rows,held.capture_proof()?.relation_kind()?.as_str()),
            Operation::Item(v)=>{current_grant(&v.grant,rows)?;validate_kind_dependency(item.as_ref().expect("validated item"),rows)},
            Operation::Revoke(v)=>{current_grant(&v.grant,rows)?;Ok(())}
        }};
        validate(&rows)?;
        let name=op.name();let stage_name=format!("accept-{name}");
        let existing=inventory.transactions.get(&name).or_else(||inventory.transactions.get(&stage_name));
        let expected=if let Some(files)=existing {artifacts.expected(&lock.owner,if inventory.transactions.contains_key(&name){&name}else{&stage_name},&files["intent.json"],&mut std::collections::BTreeSet::new())?}
        else {
            notify(WorkWriteBoundaryV1::BeforeIntent)?;
            let intent=Intent {schema:op.schema().into(),owner_identity:lock.owner.identity.clone(),operation:op.clone(),authority:held.capture_proof()?};
            let bytes=encode(&intent)?;let expected=artifacts.expected(&lock.owner,&name,&bytes,&mut std::collections::BTreeSet::new())?;
            held.validate_current()?;lock.validate()?;effects.set(true);
            persist_record(&lock,&op,&Stored {staging:true,files:BTreeMap::from([("intent.json".into(),bytes)])},&expected,||held.validate_current(),&notify)?;
            notify(WorkWriteBoundaryV1::Reservation)?;notify(WorkWriteBoundaryV1::AcceptanceCandidate)?;notify(WorkWriteBoundaryV1::StagedIntent)?;expected
        };
        effects.set(true);
        let mut record=Stored {staging:!inventory.transactions.contains_key(&name),files:existing.cloned().unwrap_or_else(||BTreeMap::from([("intent.json".into(),encode(&expected.intent).expect("preflighted intent"))]))};
        if record.staging {
            record.files.insert("not-attempted.json".into(),expected.publication.clone());
            notify(WorkWriteBoundaryV1::AcceptanceReady)?;
            record.staging=false;persist_record(&lock,&op,&record,&expected,||held.validate_current(),&notify)?;
        }
        notify(WorkWriteBoundaryV1::Intent)?;
        if !record.files.contains_key("decision.json"){record.files.insert("decision.json".into(),expected.decision.clone());persist_record(&lock,&op,&record,&expected,||held.validate_current(),&notify)?;}
        notify(WorkWriteBoundaryV1::Decision)?;
        record.files.insert("snapshot.json".into(),expected.snapshot.clone());record.files.insert("terminal.pending".into(),expected.terminal.clone());persist_record(&lock,&op,&record,&expected,||held.validate_current(),&notify)?;
        notify(WorkWriteBoundaryV1::Snapshot)?;
        held.validate_current()?;lock.validate()?;
        let (_,mut current)=scan_with_artifacts(&lock,&op,&mut artifacts)?;reservation(&op,&current)?;current.retain(|r|r.intent.operation!=op);validate(&current)?;
        record.files.remove("not-attempted.json");record.files.insert("attempted.json".into(),expected.publication.clone());
        record.files.remove("terminal.pending");record.files.insert("terminal.json".into(),expected.terminal.clone());
        persist_record(&lock,&op,&record,&expected,||held.validate_current(),&notify)?;
        notify(WorkWriteBoundaryV1::Terminal)?;
        let (_,verified)=scan_with_artifacts(&lock,&op,&mut artifacts)?;
        let proof=verified.into_iter().find(|r|r.intent.operation==op&&r.committed).ok_or_else(||refuse("work_terminal_unproven","exact selected terminal proof required"))?;
        held.validate_current()?;lock.validate()?;notify(WorkWriteBoundaryV1::Reply)?;Ok(proof.output)
    };
    let result = if matches!(&op, Operation::Item(_)) {
        with_work_registration_current(repo, &check, &basis, consume)
    } else {
        with_work_registration_establishment(
            repo,
            &check,
            &basis,
            |root| {
                if let Some(origin) = op.bootstrap() {
                    drop(store::establish(root, &origin, &effects, &mut |stage| {
                        notify(stage)
                    })?);
                }
                Ok(())
            },
            consume,
        )
    };
    match result {
        Ok(v) => WorkExecutionV1::success(WorkDispositionV1::Committed, Some(v)),
        Err(e) if recover && existed => close_noncommit(repo, &op, e),
        Err(e) => WorkExecutionV1::error(
            if effects.get()
                || matches!(
                    e.code.as_str(),
                    "work_bootstrap_uncertain" | "work_owner_lost"
                )
            {
                WorkDispositionV1::Uncertain
            } else {
                WorkDispositionV1::Refused
            },
            e,
        ),
    }
}
fn close_noncommit(
    repo: &Path,
    op: &Operation,
    original: LocalAuthorityRefusalV1,
) -> WorkExecutionV1<Output> {
    let result = (|| {
        let root = RetainedRepositoryRootV1::open_exact(repo).map_err(fs_error)?;
        let lock = store::open(&root)?;
        let (_, rows) = scan(&lock,&op)?;
        reservation(op, &rows)?;
        let Some(row) = rows.into_iter().find(|r| r.intent.operation == *op) else {
            return Ok(WorkExecutionV1::error(
                WorkDispositionV1::Refused,
                original.clone(),
            ));
        };
        if row.committed {
            return Ok(WorkExecutionV1::success(
                WorkDispositionV1::Replayed,
                Some(row.output),
            ));
        }
        if row.attempted || row.staging {
            return Ok(WorkExecutionV1::error(
                WorkDispositionV1::Uncertain,
                original.clone(),
            ));
        }
        lock.validate()?;
        let mut record=lock.db.with_current(|snapshot| snapshot_record(snapshot,&op.name())).map_err(db_error)??.ok_or_else(||codec("original request"))?;
        let expected=Artifacts::default().expected(&lock.owner,&op.name(),&record.files["intent.json"],&mut std::collections::BTreeSet::new())?;
        record.files.insert("noncommit.json".into(),expected.noncommit.clone());
        persist_record(&lock,op,&record,&expected,||Ok(()),&|_|Ok(()))?;
        let (_,proof)=scan(&lock,op)?;
        if !proof
            .iter()
            .any(|r| r.intent.operation == *op && r.noncommit)
        {
            return Err(refuse(
                "work_noncommit_unproven",
                "noncommit closure unproven",
            ));
        }
        lock.validate()?;
        Ok(WorkExecutionV1::success(
            WorkDispositionV1::NonCommitted,
            None,
        ))
    })();
    result.unwrap_or_else(|e| WorkExecutionV1::error(WorkDispositionV1::Uncertain, e))
}
fn project_grant(v: WorkExecutionV1<Output>) -> WorkExecutionV1<WorkGrantV1> {
    WorkExecutionV1 {
        disposition: v.disposition,
        refusal: v.refusal,
        outcome: v.outcome.and_then(|o| match o {
            Output::Grant(g) => Some(g),
            _ => None,
        }),
    }
}
fn project_kind(v: WorkExecutionV1<Output>) -> WorkExecutionV1<WorkKindReceiptV1> {
    WorkExecutionV1 {
        disposition: v.disposition,
        refusal: v.refusal,
        outcome: v.outcome.and_then(|output| match output {
            Output::Kind(receipt) => Some(receipt),
            _ => None,
        }),
    }
}
fn project_item(v: WorkExecutionV1<Output>) -> WorkExecutionV1<WorkRegistrationReceiptV1> {
    WorkExecutionV1 {
        disposition: v.disposition,
        refusal: v.refusal,
        outcome: v.outcome.and_then(|o| match o {
            Output::Item(g) => Some(g),
            _ => None,
        }),
    }
}
fn project_revoke(v: WorkExecutionV1<Output>) -> WorkExecutionV1<WorkRevocationReceiptV1> {
    WorkExecutionV1 {
        disposition: v.disposition,
        refusal: v.refusal,
        outcome: v.outcome.and_then(|o| match o {
            Output::Revoke(g) => Some(g),
            _ => None,
        }),
    }
}

#[derive(Clone,Debug)]
pub enum WorkRetainedOutcomeV1 {Kind(WorkKindReceiptV1),Grant(WorkGrantV1),Item(WorkRegistrationReceiptV1),Revoke(WorkRevocationReceiptV1)}
#[derive(Clone,Debug)]
pub struct WorkHistoryRecordV1 {pub original_request_name:String,pub disposition:WorkDispositionV1,pub retained_record_bytes:u64,pub outcome:Option<WorkRetainedOutcomeV1>}
#[derive(Clone,Debug)]
pub struct WorkHistoryPageV1 {pub records:Vec<WorkHistoryRecordV1>,pub next:Option<String>,pub complete:bool}
pub struct WorkStorageSnapshotV1<'a> {snapshot:file_native::Snapshot<super::database::WorkFs<'a>>,root:&'a RetainedRepositoryRootV1,owner:store::Owner,owner_proof:crate::RepositoryExactFilesCheckpointV1<'a>}
impl WorkStorageSnapshotV1<'_> {
    fn row(&self,name:&str,artifacts:&mut Artifacts,proofs:&mut std::collections::BTreeSet<Vec<u8>>)->Result<Option<(Parsed,u64)>,LocalAuthorityRefusalV1>{
        let Some(record)=snapshot_record(&self.snapshot,name)? else{return Ok(None)};
        let bytes=record.files.values().map(|b|b.len() as u64).sum();
        let row=parsed(&self.owner,&if record.staging{format!("accept-{name}")}else{name.into()},&record.files,artifacts,proofs)?;
        Ok(Some((row,bytes)))
    }
    fn dependencies(&self,row:&Parsed,artifacts:&mut Artifacts,proofs:&mut std::collections::BTreeSet<Vec<u8>>)->Result<(),LocalAuthorityRefusalV1>{
        match &row.intent.operation {
            Operation::Item(v)=>{
                let (g,_)=self.row(&format!("grant-{}",v.grant.terms.request_key),artifacts,proofs)?.ok_or_else(||refuse("work_grant_history_missing","exact original grant required"))?;
                if !g.committed||!matches!(&g.intent.operation,Operation::Grant(grant) if grant==&v.grant){return Err(refuse("work_grant_history_missing","exact committed original grant required"));}
                if let Some(p)=&v.kind_provenance {let (k,_)=self.row(&format!("kind-{}",p.establishment.locator.approval.request_key),artifacts,proofs)?.ok_or_else(||refuse("work_kind_history_missing","exact original kind required"))?;if !k.committed||!matches!(&k.output,Output::Kind(k) if k==&p.establishment){return Err(refuse("work_kind_history_missing","exact committed original kind required"));}}
            },
            Operation::Revoke(v)=>{let(g,_)=self.row(&format!("grant-{}",v.grant.terms.request_key),artifacts,proofs)?.ok_or_else(||refuse("work_grant_history_missing","original grant required"))?;if !g.committed||!matches!(&g.intent.operation,Operation::Grant(grant) if grant==&v.grant){return Err(refuse("work_grant_history_missing","exact original grant required"));}},_=>{}
        }
        Ok(())
    }
    pub fn original(&self,locator:&WorkRequestLocatorV1)->WorkExecutionV1<WorkRegistrationReceiptV1>{
        let result=(||{self.owner_proof.validate().map_err(fs_error)?;store::validate_reader_owner(self.root,&self.owner)?;locator.validate()?;
            let mut artifacts=Artifacts::default();let mut proofs=std::collections::BTreeSet::new();
            let name=format!("item-{}",locator.input.request_key);
            let Some((row,_))=self.row(&name,&mut artifacts,&mut proofs)? else{
                if read_name(&self.snapshot,&format!("item/{}",locator.item_identity))?.as_deref()==Some(name.as_str()){
                    return Err(refuse("work_item_history_missing","positive item index has no original request"));
                }
                return Ok(WorkExecutionV1::success(WorkDispositionV1::Unaccepted,None))
            };
            if row.intent.operation!=Operation::Item(locator.clone()){return Err(refuse("work_request_conflict","original request content differs"));}
            self.dependencies(&row,&mut artifacts,&mut proofs)?;
            Ok(if row.committed {project_item(WorkExecutionV1::success(WorkDispositionV1::Replayed,Some(row.output)))}else if row.noncommit{WorkExecutionV1::success(WorkDispositionV1::NonCommitted,None)}else if row.attempted{WorkExecutionV1::error(WorkDispositionV1::Uncertain,refuse("work_publication_uncertain","attempted original has no exact terminal"))}else{WorkExecutionV1::success(WorkDispositionV1::Pending,None)})
        })();if let Err(e)=self.owner_proof.validate().map_err(fs_error){return WorkExecutionV1::error(WorkDispositionV1::Uncertain,e);}result.unwrap_or_else(|e|WorkExecutionV1::error(WorkDispositionV1::Uncertain,e))
    }
    /// Bounded exact original history as of this retained view. The continuation
    /// counts examined keys even if an outcome remains pending/noncommitted.
    pub fn page(&self,after:Option<&str>,limit:usize)->Result<WorkHistoryPageV1,LocalAuthorityRefusalV1>{
        if limit==0||limit>16{return Err(refuse("work_history_page_bound","history page requires 1..16 originals"));}
        self.owner_proof.validate().map_err(fs_error)?;store::validate_reader_owner(self.root,&self.owner)?;
        let after=after.map(|name|format!("request/{name}")).unwrap_or_else(||"request/".into());
        let page=self.snapshot.page(Some(&after),limit).map_err(db_error)?;
        let mut complete=page.complete;let mut next=None;let mut records=Vec::new();let mut artifacts=Artifacts::default();let mut proofs=std::collections::BTreeSet::new();
        for (key,_) in page.records {
            let Some(name)=key.strip_prefix("request/") else{complete=true;break};next=Some(name.to_owned());
            let (row,bytes)=self.row(name,&mut artifacts,&mut proofs)?.ok_or_else(||refuse("work_proof_missing","positive original history row required"))?;self.dependencies(&row,&mut artifacts,&mut proofs)?;
            let disposition=if row.committed{WorkDispositionV1::Replayed}else if row.noncommit{WorkDispositionV1::NonCommitted}else if row.attempted{WorkDispositionV1::Uncertain}else{WorkDispositionV1::Pending};
            let outcome=if row.committed{Some(match row.output {Output::Kind(v)=>WorkRetainedOutcomeV1::Kind(v),Output::Grant(v)=>WorkRetainedOutcomeV1::Grant(v),Output::Item(v)=>WorkRetainedOutcomeV1::Item(v),Output::Revoke(v)=>WorkRetainedOutcomeV1::Revoke(v)})}else{None};
            records.push(WorkHistoryRecordV1 {original_request_name:name.into(),disposition,retained_record_bytes:bytes,outcome});
        }
        self.owner_proof.validate().map_err(fs_error)?;store::validate_reader_owner(self.root,&self.owner)?;Ok(WorkHistoryPageV1 {records,next:if complete{None}else{next},complete})
    }
}
