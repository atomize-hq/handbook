use super::store::{self, Inventory, WorkLock};
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
        lock: &WorkLock<'_>,
        name: &str,
        bytes: &[u8],
        proofs: &mut std::collections::BTreeSet<Vec<u8>>,
    ) -> Result<Rc<ExpectedRecords>, LocalAuthorityRefusalV1> {
        if let Some(value) = self.records.get(bytes) {
            validate_original_context(lock, name, &value.intent)?;
            return Ok(Rc::clone(value));
        }
        let intent: Intent = decode(bytes)?;
        let item = if let Operation::Item(locator) = &intent.operation {
            Some(locator.validated()?)
        } else {
            intent.operation.validate()?;
            None
        };
        validate_original_context(lock, name, &intent)?;
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
    lock: &WorkLock<'_>,
    name: &str,
    intent: &Intent,
) -> Result<(), LocalAuthorityRefusalV1> {
    if intent.schema != intent.operation.schema()
        || name.strip_prefix("accept-").unwrap_or(name) != intent.operation.name()
        || intent.owner_identity != lock.owner.identity
        || intent.operation.project() != lock.owner.project
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
    lock: &WorkLock<'_>,
    name: &str,
    files: &BTreeMap<String, Vec<u8>>,
    artifacts: &mut Artifacts,
    proofs: &mut std::collections::BTreeSet<Vec<u8>>,
) -> Result<Parsed, LocalAuthorityRefusalV1> {
    let expected = artifacts.expected(
        lock,
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
fn scan(lock: &WorkLock<'_>) -> Result<(Inventory, Vec<Parsed>), LocalAuthorityRefusalV1> {
    scan_with_artifacts(lock, &mut Artifacts::default())
}
fn scan_with_artifacts(
    lock: &WorkLock<'_>,
    artifacts: &mut Artifacts,
) -> Result<(Inventory, Vec<Parsed>), LocalAuthorityRefusalV1> {
    let tree = lock
        .root
        .retain_work_history_tree_guard_v1()
        .map_err(fs_error)?;
    let metadata = lock
        .root
        .retain_exact_files_checkpoint_v1(&[
            ".handbook-work-state-bootstrap.v1.json",
            ".handbook-work-state.v1.json",
        ])
        .map_err(fs_error)?;
    let inventory = lock.inventory()?;
    // Keep no prior version of an intent and no artifact for a disappeared row.
    // This limits storage to the admitted current history within this lock scope.
    artifacts.records.retain(|bytes, _| {
        inventory.transactions.values().any(|files| {
            files
                .get("intent.json")
                .is_some_and(|current| current == bytes)
        })
    });
    let mut rows = Vec::new();
    let mut proofs = std::collections::BTreeSet::new();
    for (name, files) in &inventory.transactions {
        rows.push(parsed(lock, name, files, artifacts, &mut proofs)?);
    }
    // Both an acceptance candidate and a visible request for the same key are
    // conflicting proof. Never choose whichever appears newer.
    for (i, row) in rows.iter().enumerate() {
        if rows[..i]
            .iter()
            .any(|p| p.intent.operation.name() == row.intent.operation.name())
        {
            return Err(refuse(
                "work_duplicate_request",
                "conflicting acceptance and request paths",
            ));
        }
    }
    for row in &rows {
        let required_grant = match &row.intent.operation {
            Operation::Kind(_) => None,
            Operation::Item(v) => Some(&v.grant.fingerprint),
            Operation::Revoke(v) => Some(&v.grant.fingerprint),
            Operation::Grant(v) => v.predecessor_grant_fingerprint.as_ref(),
        };
        if let Some(identity) = required_grant {
            if !rows.iter().any(|prior|prior.committed&&matches!(&prior.intent.operation,Operation::Grant(g) if &g.fingerprint==identity)){return Err(refuse("work_grant_history_missing","exact original committed activation proof required for retained dependent history"));}
        }
        if let Operation::Item(v) = &row.intent.operation {
            if let Some(provenance) = &v.kind_provenance {
                if !rows.iter().any(|prior| prior.committed && matches!(&prior.output, Output::Kind(receipt) if receipt == &provenance.establishment)) {
                    return Err(refuse("work_kind_history_missing", "exact committed kind definition/establishment/terminal dependency required"));
                }
            }
        }
    }
    for (index, row) in rows.iter().enumerate() {
        if !row.noncommit {
            if let Operation::Kind(kind) = &row.intent.operation {
                if rows[..index].iter().any(|prior| !prior.noncommit && matches!(&prior.intent.operation, Operation::Kind(v) if v.binding.reference == kind.binding.reference)) {
                    return Err(refuse("work_kind_version_conflict", "one exact committed or reserved kind version required"));
                }
            }
        }
    }
    validate_authority_history(&rows)?;
    tree.validate().map_err(fs_error)?;
    metadata.validate().map_err(fs_error)?;
    lock.validate()?;
    Ok((inventory, rows))
}
// Validate the closed chain using retained historical proof, independently of
// today's operating basis. No newest-file selection or union of prior scopes.
fn validate_authority_history(rows: &[Parsed]) -> Result<(), LocalAuthorityRefusalV1> {
    let grants = rows
        .iter()
        .filter(|r| r.committed && matches!(r.output, Output::Grant(_)))
        .collect::<Vec<_>>();
    let roots = grants
        .iter()
        .filter(|r| {
            r.intent
                .operation
                .grant()
                .predecessor_grant_fingerprint
                .is_none()
        })
        .collect::<Vec<_>>();
    if !grants.is_empty() && roots.len() != 1 {
        return Err(refuse(
            "work_authority_chain",
            "exactly one committed authority lineage required",
        ));
    }
    let mut visited = 0;
    let mut head = roots.first().map(|r| r.intent.operation.grant());
    while let Some(old) = head {
        visited += 1;
        let next = grants
            .iter()
            .filter(|r| {
                r.intent
                    .operation
                    .grant()
                    .predecessor_grant_fingerprint
                    .as_ref()
                    == Some(&old.fingerprint)
            })
            .collect::<Vec<_>>();
        if next.len() > 1 || visited > grants.len() {
            return Err(refuse(
                "work_authority_chain",
                "branched or cyclic authority is unsupported",
            ));
        }
        if let Some(row) = next.first() {
            if rows.iter().any(|r| {
                r.committed
                    && matches!(&r.intent.operation, Operation::Revoke(v) if v.grant == *old)
            }) {
                return Err(refuse(
                    "work_authority_chain",
                    "committed revocation cannot have a successor or restoration",
                ));
            }
            let new = row.intent.operation.grant();
            successor_compatible(
                new,
                old,
                row.intent.authority.relation_kind()?.as_str(),
                &relation_kind(&old.basis, rows, old)?,
            )?;
            head = Some(new);
        } else {
            break;
        }
    }
    if visited != grants.len() {
        return Err(refuse(
            "work_authority_chain",
            "detached committed authority history",
        ));
    }
    let pending = rows
        .iter()
        .filter(|r| {
            !r.committed && !r.noncommit && matches!(r.output, Output::Grant(_) | Output::Revoke(_))
        })
        .collect::<Vec<_>>();
    if pending.len() > 1 {
        return Err(refuse(
            "work_administration_reserved",
            "one unresolved authority operation required",
        ));
    }
    if let Some(row) = pending.first() {
        match (&row.intent.operation, head) {
            (Operation::Grant(g), None) if g.predecessor_grant_fingerprint.is_none() => {}
            (Operation::Grant(g), Some(old)) => {
                successor_compatible(
                    g,
                    old,
                    row.intent.authority.relation_kind()?.as_str(),
                    &relation_kind(&old.basis, rows, old)?,
                )?;
            }
            (Operation::Revoke(v), Some(old)) if v.grant == *old => {}
            _ => {
                return Err(refuse(
                    "work_authority_chain",
                    "pending administration must name the exact latest predecessor",
                ))
            }
        }
        if let Some(old) = head {
            if rows.iter().any(|r| {
                r.committed
                    && matches!(&r.intent.operation, Operation::Revoke(v) if v.grant == *old)
            }) {
                return Err(refuse(
                    "work_authority_chain",
                    "revoked head cannot be restored",
                ));
            }
        }
    }
    Ok(())
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
    Terminal,
    Reply,
}
pub struct NativeWorkStateV1;
impl NativeWorkStateV1 {
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
                let (_, rows) = scan(&lock)?;
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
        let (_, rows) = scan(&lock)?;
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
                    let (_, rows) = scan(&lock)?;
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
            let (_, rows) = scan(&lock)?;
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
            let (_, rows) = scan(&lock)?;
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
        let (_, rows) = scan(&lock)?;
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
        let (_, rows) = scan(&lock)?;
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
        let (_, rows) = scan(&lock)?;
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
        // I1 has already released its work lock. Only the held invocation's
        // I2–I4 observations share pure successful artifacts.
        let mut artifacts = Artifacts::default();
        let (inventory, mut rows) = scan_with_artifacts(&lock, &mut artifacts)?;
        reservation(&op, &rows)?;
        let mut paths = inventory
            .transactions
            .iter()
            .filter(|(name, _)| name.strip_prefix("accept-").unwrap_or(name) != op.name())
            .flat_map(|(name, files)| {
                files
                    .keys()
                    .map(move |file| format!("{WORK_STATE_ROOT_V1}/{name}/{file}"))
            })
            .chain([
                ".handbook-work-state.v1.json".into(),
                ".handbook-work-state-bootstrap.v1.json".into(),
                format!("{WORK_STATE_ROOT_V1}/owner.json"),
            ])
            .collect::<Vec<_>>();
        paths.sort();
        paths.dedup();
        let references = paths.iter().map(String::as_str).collect::<Vec<_>>();
        // Pin immutable original work grant/lineage and retained owner proof;
        // a work-lock alone does not prove those bytes remained exact.
        let work_proof = held
            .root()
            .retain_exact_files_checkpoint_v1(&references)
            .map_err(fs_error)?;
        if let Some(row) = rows.iter().find(|r| r.intent.operation == op) {
            if row.committed {
                return Ok(row.output.clone());
            }
            if row.noncommit {
                return Err(refuse(
                    "work_request_noncommit",
                    "original request is durably not committed",
                ));
            }
        }
        // Original pending activation/revocation cannot invalidate its own
        // establishment/revocation checks, but invalidates every other request.
        rows.retain(|r| r.intent.operation != op);
        match &op {
            Operation::Kind(v) => v.validate()?,
            Operation::Grant(g) => {
                activation_compatible(g, &rows, held.capture_proof()?.relation_kind()?.as_str())?
            }
            Operation::Item(v) => {
                current_grant(&v.grant, &rows)?;
                validate_kind_dependency(item.as_ref().expect("validated Item input"), &rows)?;
            }
            Operation::Revoke(v) => {
                current_grant(&v.grant, &rows)?;
            }
        }
        let name = op.name();
        let base = format!("{WORK_STATE_ROOT_V1}/{name}");
        let stage = format!("{WORK_STATE_ROOT_V1}/accept-{name}");
        let found = inventory
            .transactions
            .get(&name)
            .or_else(|| inventory.transactions.get(&format!("accept-{name}")));
        let stage_name = format!("accept-{name}");
        let expected = if let Some(files) = found {
            artifacts.expected(
                &lock,
                if inventory.transactions.contains_key(&name) {
                    &name
                } else {
                    &stage_name
                },
                files.get("intent.json").ok_or_else(|| {
                    refuse(
                        "work_partial_inventory",
                        "transaction lacks original intent",
                    )
                })?,
                &mut std::collections::BTreeSet::new(),
            )?
        } else {
            if inventory.transactions.len() >= store::CAPACITY
                || inventory.bytes + 8 * MAX > 128 * 1024 * 1024
            {
                return Err(refuse(
                    "work_capacity",
                    "reserve complete peak transaction and closure capacity before acceptance",
                ));
            }
            notify(WorkWriteBoundaryV1::BeforeIntent)?;
            let v = Intent {
                schema: op.schema().into(),
                owner_identity: lock.owner.identity.clone(),
                operation: op.clone(),
                authority: held.capture_proof()?,
            };
            // Every retained/derived record must fit before accepting the
            // original reservation, including fingerprint framing overhead.
            let intent_bytes = encode(&v)?;
            let expected = artifacts.expected(
                &lock,
                &name,
                &intent_bytes,
                &mut std::collections::BTreeSet::new(),
            )?;
            held.validate_current()?;
            lock.validate()?;
            lock.reserve_intent(&name, &intent_bytes, &effects)?;
            notify(WorkWriteBoundaryV1::Reservation)?;
            // Failure can follow mkdir application but precede its durable reply.
            effects.set(true);
            held.root()
                .create_directory_new_durable(&stage)
                .map_err(fs_error)?;
            notify(WorkWriteBoundaryV1::AcceptanceCandidate)?;
            store::write_exact(held.root(), &format!("{stage}/intent.json"), &intent_bytes)?;
            notify(WorkWriteBoundaryV1::StagedIntent)?;
            expected
        };
        // Stage publication atomically accepts intent plus positive never-attempted
        // checkpoint. Partial staging is reserved and only exact original resume.
        // A resumed acceptance may apply a checkpoint or rename before a
        // native durability error. Preserve uncertainty for that prefix too.
        effects.set(true);
        if !inventory.transactions.contains_key(&name) {
            store::write_exact(
                held.root(),
                &format!("{stage}/not-attempted.json"),
                &expected.publication,
            )?;
            notify(WorkWriteBoundaryV1::AcceptanceReady)?;
            held.root()
                .rename_directory_noreplace_durable(&stage, &base)
                .map_err(fs_error)?;
        }
        notify(WorkWriteBoundaryV1::Intent)?;
        store::write_exact(
            held.root(),
            &format!("{base}/decision.json"),
            &expected.decision,
        )?;
        notify(WorkWriteBoundaryV1::Decision)?;
        store::write_exact(
            held.root(),
            &format!("{base}/snapshot.json"),
            &expected.snapshot,
        )?;
        store::write_exact(
            held.root(),
            &format!("{base}/terminal.pending"),
            &expected.terminal,
        )?;
        notify(WorkWriteBoundaryV1::Snapshot)?;
        held.validate_current()?;
        lock.validate()?;
        let (current_inventory, mut current) = scan_with_artifacts(&lock, &mut artifacts)?;
        reservation(&op, &current)?;
        current.retain(|row| row.intent.operation != op);
        match &op {
            Operation::Kind(v) => v.validate()?,
            Operation::Item(v) => {
                current_grant(&v.grant, &current)?;
                validate_kind_dependency(item.as_ref().expect("validated Item input"), &current)?;
            }
            Operation::Grant(v) => {
                activation_compatible(v, &current, held.capture_proof()?.relation_kind()?.as_str())?
            }
            Operation::Revoke(v) => {
                current_grant(&v.grant, &current)?;
            }
        }
        let manifest_paths = current_inventory
            .manifest_paths
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let manifest_proof = held
            .root()
            .retain_exact_files_checkpoint_v1(&manifest_paths)
            .map_err(fs_error)?;
        // Persist publication attempt BEFORE invoking the native no-replace
        // installation. Never create a missing checkpoint by absence fallback.
        held.root()
            .rename_noreplace_durable(
                &format!("{base}/not-attempted.json"),
                &format!("{base}/attempted.json"),
            )
            .map_err(fs_error)?;
        notify(WorkWriteBoundaryV1::PublicationAttempt)?;
        held.validate_current()?;
        lock.validate()?;
        work_proof.validate().map_err(fs_error)?;
        manifest_proof.validate().map_err(fs_error)?;
        held.root()
            .rename_noreplace_durable(
                &format!("{base}/terminal.pending"),
                &format!("{base}/terminal.json"),
            )
            .map_err(fs_error)?;
        notify(WorkWriteBoundaryV1::Terminal)?;
        let (_, verified) = scan_with_artifacts(&lock, &mut artifacts)?;
        let proof = verified
            .into_iter()
            .find(|r| r.intent.operation == op && r.committed)
            .ok_or_else(|| {
                refuse(
                    "work_terminal_unproven",
                    "exact publication proof unavailable",
                )
            })?;
        held.validate_current()?;
        lock.validate()?;
        work_proof.validate().map_err(fs_error)?;
        manifest_proof.validate().map_err(fs_error)?;
        notify(WorkWriteBoundaryV1::Reply)?;
        Ok(proof.output)
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
        let (_, rows) = scan(&lock)?;
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
        store::write_exact(
            &root,
            &format!("{WORK_STATE_ROOT_V1}/{}/noncommit.json", op.name()),
            &noncommit(&row.intent)?,
        )?;
        let (_, proof) = scan(&lock)?;
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
