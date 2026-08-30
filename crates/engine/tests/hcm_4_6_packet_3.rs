#![cfg(feature = "test-support")]

use handbook_engine::{
    DefinitionFingerprint, SnapshotBoundaryKindV1, SnapshotCaptureOutcomeV1,
    SnapshotCaptureRequestV1, SnapshotGroundingErrorKindV1, SnapshotGroundingOwnerV1,
    SnapshotSlotObservationV1, SnapshotSourceFamilyV1, SnapshotSourcePortErrorV1,
    SnapshotSourcePortV1,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use tempfile::TempDir;

const WORKSPACE_REF: &str = "handbook.workspace.packet-3-test@1.0.0";
const WORKSPACE_FINGERPRINT: &str =
    "sha256:3333333333333333333333333333333333333333333333333333333333333333";

#[test]
fn first_start_captures_every_slot_once_and_replay_never_recaptures() {
    let repo = TempDir::new().expect("temporary repository");
    let owner = SnapshotGroundingOwnerV1::new();
    let binding = owner
        .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
        .expect("repository binding");
    let stream = owner.bind_session_stream(&binding);
    let mut port = CompletePort::successful();
    let request = request("key-1", "session-1", SnapshotBoundaryKindV1::Start);

    let captured = owner
        .capture_boundary(&binding, &stream, &request, &mut port)
        .expect("first start capture");
    let SnapshotCaptureOutcomeV1::Started(started) = &captured else {
        panic!("start request must return typed started grounding")
    };
    assert!(started.is_first_start());
    assert_eq!(started.sequence(), 1);
    assert!(!started.replayed());
    assert!(started.delta_record().is_none());
    assert_eq!(
        port.observed.len(),
        42,
        "14 owner-port slots have before/after plus a final precommit recheck"
    );
    assert_eq!(
        port.reads.len(),
        11,
        "redacted/omitted/inapplicable slots are never read"
    );
    assert!(!port
        .reads
        .contains(&(SnapshotSourceFamilyV1::Git, "worktree")));
    assert!(!port
        .reads
        .contains(&(SnapshotSourceFamilyV1::Evidence, "dock_execution_candidate")));
    assert!(!port
        .reads
        .contains(&(SnapshotSourceFamilyV1::Work, "handoff")));

    let repository = owner
        .open_repository(&binding, &stream)
        .expect("repository reopen");
    let accounting: Value = serde_json::from_slice(
        &repository
            .read_record(started.repository_commit_for_testing().accounting_record())
            .expect("accounting bytes"),
    )
    .expect("accounting JSON");
    assert_eq!(
        accounting["slots"]
            .as_array()
            .expect("slot accounting")
            .len(),
        15
    );
    assert_eq!(
        accounting["rules"]
            .as_array()
            .expect("rule accounting")
            .len(),
        11
    );
    assert_eq!(accounting["complete_exactly_once"], Value::Bool(true));
    let persisted = String::from_utf8(
        repository
            .read_record(started.repository_commit_for_testing().grounding_record())
            .expect("grounding bytes"),
    )
    .expect("grounding UTF-8");
    assert!(!persisted.contains("RAW-WORKTREE-SECRET"));
    assert!(!persisted.contains("RAW-CANDIDATE-SECRET"));
    assert!(!persisted.contains("RAW-HANDOFF-SECRET"));
    drop(repository);

    port.reset_counters();
    port.fail_all_observations = true;
    let replay = owner
        .capture_boundary(&binding, &stream, &request, &mut port)
        .expect("exact replay");
    let SnapshotCaptureOutcomeV1::Started(replayed) = replay else {
        panic!("start replay remains typed started grounding")
    };
    assert!(replayed.replayed());
    assert_eq!(replayed.receipt_record(), started.receipt_record());
    assert!(port.observed.is_empty());
    assert!(port.reads.is_empty());
}

#[test]
fn start_after_prior_end_and_final_end_use_exact_compatible_pairs_and_handoff() {
    let repo = TempDir::new().expect("temporary repository");
    let owner = SnapshotGroundingOwnerV1::new();
    let binding = owner
        .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
        .expect("repository binding");
    let stream = owner.bind_session_stream(&binding);
    let mut port = CompletePort::successful();

    let first = owner
        .capture_boundary(
            &binding,
            &stream,
            &request("key-1", "session-1", SnapshotBoundaryKindV1::Start),
            &mut port,
        )
        .expect("first start");
    port.advance(
        "work",
        "handoff",
        "handbook.snapshot-observation.work.handoff-session-1@1.0.0",
        json!({"refs":["handoff/ref-1"]}),
    );
    let final_one = owner
        .capture_boundary(
            &binding,
            &stream,
            &request("key-2", "session-1", SnapshotBoundaryKindV1::End),
            &mut port,
        )
        .expect("first final");
    let SnapshotCaptureOutcomeV1::Finalized(final_one) = final_one else {
        panic!("end request must return typed final grounding")
    };
    assert_eq!(final_one.sequence(), 2);
    assert_eq!(final_one.handoff_refs(), &["handoff/ref-1"]);
    assert_eq!(
        final_one.predecessor_boundary_ref(),
        Some(first.boundary_record().exact_ref())
    );
    assert!(final_one.delta_record().is_some());

    port.advance(
        "git",
        "head",
        "handbook.snapshot-observation.git.head-2@1.0.0",
        json!({"ref":"handbook.snapshot-observation.git.head-2@1.0.0"}),
    );
    let next_start = owner
        .capture_boundary(
            &binding,
            &stream,
            &request("key-3", "session-2", SnapshotBoundaryKindV1::Start),
            &mut port,
        )
        .expect("next start");
    let SnapshotCaptureOutcomeV1::Started(next_start) = next_start else {
        panic!("next start remains typed start")
    };
    assert!(!next_start.is_first_start());
    assert_eq!(next_start.sequence(), 3);
    assert_eq!(
        next_start.predecessor_boundary_ref(),
        Some(final_one.boundary_record().exact_ref())
    );
    assert!(next_start.delta_record().is_some());

    let repository = owner
        .open_repository(&binding, &stream)
        .expect("repository reopen");
    let final_delta: Value = serde_json::from_slice(
        &repository
            .read_record(
                final_one
                    .repository_commit_for_testing()
                    .delta_record()
                    .unwrap(),
            )
            .expect("final delta bytes"),
    )
    .expect("final delta JSON");
    assert_eq!(final_delta["pair_kind"], "open_start_to_end");
    assert_eq!(final_delta["before_sequence"], 1);
    assert_eq!(final_delta["after_sequence"], 2);
    assert!(final_delta["before"]["ref"]
        .as_str()
        .unwrap()
        .starts_with("handbook.snapshot-capture@sha256:"));
    assert!(final_delta["after"]["ref"]
        .as_str()
        .unwrap()
        .starts_with("handbook.snapshot-capture@sha256:"));
    assert!(final_delta["before"]["fingerprint"]
        .as_str()
        .unwrap()
        .starts_with("sha256:"));
    assert!(final_delta["after"]["fingerprint"]
        .as_str()
        .unwrap()
        .starts_with("sha256:"));
    assert_eq!(
        final_delta["evaluations"]
            .as_array()
            .expect("complete delta evaluation")
            .len(),
        15
    );
    let next_delta: Value = serde_json::from_slice(
        &repository
            .read_record(
                next_start
                    .repository_commit_for_testing()
                    .delta_record()
                    .unwrap(),
            )
            .expect("next-start delta bytes"),
    )
    .expect("next-start delta JSON");
    assert_eq!(next_delta["pair_kind"], "prior_end_to_start");
    assert_eq!(next_delta["before_sequence"], 2);
    assert_eq!(next_delta["after_sequence"], 3);
}

#[test]
fn drift_substitution_incomplete_accounting_and_source_failures_write_nothing() {
    for scenario in ["drift", "substitution", "missing", "unavailable", "refused"] {
        let repo = TempDir::new().expect("temporary repository");
        let owner = SnapshotGroundingOwnerV1::new();
        let binding = owner
            .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
            .expect("repository binding");
        let stream = owner.bind_session_stream(&binding);
        let mut port = CompletePort::successful();
        match scenario {
            "drift" => port.drift_after = Some((SnapshotSourceFamilyV1::Git, "head")),
            "substitution" => {
                port.substitute_after = Some((SnapshotSourceFamilyV1::Handbook, "engine_authority"))
            }
            "missing" => {
                port.observations
                    .remove(&(SnapshotSourceFamilyV1::Evidence, "contracts_evidence_root"));
            }
            "unavailable" => {
                port.observations.insert(
                    (SnapshotSourceFamilyV1::Work, "flow_state"),
                    SnapshotSlotObservationV1::unavailable("required_stale_or_unavailable"),
                );
            }
            "refused" => {
                port.observations.insert(
                    (SnapshotSourceFamilyV1::Handbook, "engine_authority"),
                    SnapshotSlotObservationV1::refused("stale_substituted_or_unadmitted_authority"),
                );
            }
            _ => unreachable!(),
        }

        let failure = owner
            .capture_boundary(
                &binding,
                &stream,
                &request("key-1", "session-1", SnapshotBoundaryKindV1::Start),
                &mut port,
            )
            .expect_err("scenario must fail closed");
        let expected = match scenario {
            "drift" => SnapshotGroundingErrorKindV1::SourceDrift,
            "substitution" => SnapshotGroundingErrorKindV1::SubstitutedReference,
            "missing" => SnapshotGroundingErrorKindV1::IncompleteAccounting,
            "unavailable" => SnapshotGroundingErrorKindV1::RequiredSourceUnavailable,
            "refused" => SnapshotGroundingErrorKindV1::SubstitutedReference,
            _ => unreachable!(),
        };
        assert_eq!(failure.kind(), expected, "scenario {scenario}");
        assert_eq!(
            owner
                .open_repository(&binding, &stream)
                .expect("repository open")
                .head()
                .expect("head read"),
            None,
            "scenario {scenario} must write nothing"
        );
    }
}

#[test]
fn every_family_currentness_basis_fails_closed_on_a_changed_recheck() {
    let contracts = SnapshotGroundingOwnerV1::new()
        .source_contracts()
        .iter()
        .map(|contract| (contract.family(), contract.source_slots()[0]))
        .collect::<Vec<_>>();
    for (family, slot) in contracts {
        let repo = TempDir::new().expect("temporary repository");
        let owner = SnapshotGroundingOwnerV1::new();
        let binding = owner
            .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
            .expect("repository binding");
        let stream = owner.bind_session_stream(&binding);
        let mut port = CompletePort::successful();
        if family == SnapshotSourceFamilyV1::Session {
            let failure = owner
                .capture_boundary(
                    &binding,
                    &stream,
                    &request("key-1", "session-1", SnapshotBoundaryKindV1::End),
                    &mut port,
                )
                .expect_err("session currentness is the locked owner sequence");
            assert_eq!(
                failure.kind(),
                SnapshotGroundingErrorKindV1::SequenceConflict
            );
            assert!(port.observed.is_empty());
            continue;
        }
        port.drift_after = Some((family, slot));

        let failure = owner
            .capture_boundary(
                &binding,
                &stream,
                &request("key-1", "session-1", SnapshotBoundaryKindV1::Start),
                &mut port,
            )
            .expect_err("changed family currentness must fail closed");
        assert_eq!(failure.kind(), SnapshotGroundingErrorKindV1::SourceDrift);
        assert_eq!(failure.accounting().len(), 15);
        assert_eq!(
            owner
                .open_repository(&binding, &stream)
                .expect("repository open")
                .head()
                .expect("head read"),
            None
        );
    }
}

#[test]
fn every_closed_failure_rule_has_its_exact_outcome_and_accounting_cell() {
    let rules = SnapshotGroundingOwnerV1::new()
        .source_contracts()
        .iter()
        .flat_map(|contract| {
            contract.failure_rules().iter().map(move |rule| {
                (
                    contract.family(),
                    contract.source_slots()[0],
                    rule.condition(),
                    rule.outcome(),
                )
            })
        })
        .collect::<Vec<_>>();

    for (family, slot, condition, outcome) in rules {
        use handbook_engine::SnapshotAdapterFailureOutcomeV1 as Failure;
        if condition == "first_start" {
            continue;
        }
        if condition == "wrong_open_closed_or_replayed_conflict_sequence" {
            let repo = TempDir::new().expect("temporary repository");
            let owner = SnapshotGroundingOwnerV1::new();
            let binding = owner
                .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
                .expect("repository binding");
            let stream = owner.bind_session_stream(&binding);
            let mut port = CompletePort::successful();
            let failure = owner
                .capture_boundary(
                    &binding,
                    &stream,
                    &request("key-1", "session-1", SnapshotBoundaryKindV1::End),
                    &mut port,
                )
                .expect_err("end without start must fail closed");
            assert_eq!(
                failure.kind(),
                SnapshotGroundingErrorKindV1::SequenceConflict
            );
            assert!(port.observed.is_empty());
            continue;
        }

        let repo = TempDir::new().expect("temporary repository");
        let owner = SnapshotGroundingOwnerV1::new();
        let binding = owner
            .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
            .expect("repository binding");
        let stream = owner.bind_session_stream(&binding);
        let mut port = CompletePort::successful();
        let observation = match outcome {
            Failure::AccountedOmission => SnapshotSlotObservationV1::omitted(condition),
            Failure::Refused => SnapshotSlotObservationV1::refused(condition),
            Failure::Unavailable => SnapshotSlotObservationV1::unavailable(condition),
            Failure::ValidNoApplicablePrior => unreachable!(),
        };
        port.observations.insert((family, slot), observation);
        let result = owner.capture_boundary(
            &binding,
            &stream,
            &request("key-1", "session-1", SnapshotBoundaryKindV1::Start),
            &mut port,
        );
        match outcome {
            Failure::AccountedOmission => {
                let result = result.expect("accounted omission remains a valid capture");
                let SnapshotCaptureOutcomeV1::Started(started) = result else {
                    panic!("omission scenario remains a start")
                };
                let repository = owner
                    .open_repository(&binding, &stream)
                    .expect("repository reopen");
                let accounting: Value = serde_json::from_slice(
                    &repository
                        .read_record(started.repository_commit_for_testing().accounting_record())
                        .expect("accounting record"),
                )
                .expect("accounting JSON");
                assert!(accounting["rules"].as_array().unwrap().iter().any(|rule| {
                    rule["family"] == family.as_str()
                        && rule["condition"] == condition
                        && rule["result"] == "applied"
                }));
            }
            Failure::Refused => assert_eq!(
                result.expect_err("refusal must fail").kind(),
                SnapshotGroundingErrorKindV1::SubstitutedReference
            ),
            Failure::Unavailable => assert_eq!(
                result.expect_err("unavailability must fail").kind(),
                SnapshotGroundingErrorKindV1::RequiredSourceUnavailable
            ),
            Failure::ValidNoApplicablePrior => unreachable!(),
        }
    }
}

#[test]
fn final_replay_after_reopen_and_deterministic_vectors_preserve_exact_grounding() {
    let repo = TempDir::new().expect("temporary deterministic repository");
    let mut grounding_vectors = Vec::new();
    let mut delta_fingerprints = Vec::new();
    for iteration in 0..2 {
        let owner = SnapshotGroundingOwnerV1::new();
        let binding = owner
            .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
            .expect("repository binding");
        let stream = owner.bind_session_stream(&binding);
        let mut port = CompletePort::successful();
        owner
            .capture_boundary(
                &binding,
                &stream,
                &request("key-1", "session-1", SnapshotBoundaryKindV1::Start),
                &mut port,
            )
            .expect("start capture");
        port.advance(
            "work",
            "handoff",
            "handbook.snapshot-observation.work.handoff-final@1.0.0",
            json!({"refs":["handoff/ref-final"]}),
        );
        let end_request = request("key-2", "session-1", SnapshotBoundaryKindV1::End);
        let end = owner
            .capture_boundary(&binding, &stream, &end_request, &mut port)
            .expect("end capture");
        let SnapshotCaptureOutcomeV1::Finalized(end) = end else {
            panic!("typed final grounding")
        };
        let repository = owner
            .open_repository(&binding, &stream)
            .expect("repository reopen for deterministic vector");
        let mut grounding: Value = serde_json::from_slice(
            &repository
                .read_record(end.repository_commit_for_testing().grounding_record())
                .expect("grounding vector bytes"),
        )
        .expect("grounding vector JSON");
        grounding
            .as_object_mut()
            .expect("grounding object")
            .remove("predecessor");
        grounding_vectors.push(grounding);
        delta_fingerprints.push(end.delta_record().unwrap().fingerprint().to_owned());

        if iteration == 0 {
            port.reset_counters();
            port.fail_all_observations = true;
            let replay = owner
                .capture_boundary(&binding, &stream, &end_request, &mut port)
                .expect("end replay after repository reopen");
            let SnapshotCaptureOutcomeV1::Finalized(replay) = replay else {
                panic!("end replay stays finalized")
            };
            assert!(replay.replayed());
            assert_eq!(replay.receipt_record(), end.receipt_record());
            assert_eq!(replay.handoff_refs(), end.handoff_refs());
            assert!(port.observed.is_empty());
            assert!(port.reads.is_empty());
        }
        drop(repository);
        drop(binding);
        if iteration == 0 {
            std::fs::remove_dir_all(repo.path().join(".handbook"))
                .expect("test-only deterministic repository reset");
        }
    }
    assert_eq!(grounding_vectors[0], grounding_vectors[1]);
    assert_eq!(delta_fingerprints[0], delta_fingerprints[1]);
}

#[test]
fn projection_uses_only_owner_derived_resolution_and_never_persists_forbidden_payloads() {
    for scenario in [
        "missing_resolution",
        "forbidden_payload",
        "deep_payload",
        "invalid_handoff_ref",
    ] {
        let repo = TempDir::new().expect("temporary repository");
        let owner = SnapshotGroundingOwnerV1::new();
        let binding = owner
            .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
            .expect("repository binding");
        let stream = owner.bind_session_stream(&binding);
        let mut port = CompletePort::successful();
        let expected = match scenario {
            "missing_resolution" => {
                port.observations.insert(
                    (
                        SnapshotSourceFamilyV1::Handbook,
                        "schema_artifact_resolution",
                    ),
                    SnapshotSlotObservationV1::omitted("optional_absent"),
                );
                SnapshotGroundingErrorKindV1::InsufficientResolution
            }
            "forbidden_payload" => {
                port.advance(
                    "evidence",
                    "contracts_evidence_root",
                    "handbook.snapshot-observation.evidence.forbidden@1.0.0",
                    json!({"secret_value":"must-never-persist"}),
                );
                SnapshotGroundingErrorKindV1::RedactionUncertain
            }
            "deep_payload" => {
                let mut value = Value::String("bounded-leaf".to_owned());
                for _ in 0..65 {
                    value = Value::Array(vec![value]);
                }
                port.advance(
                    "git",
                    "index",
                    "handbook.snapshot-observation.git.deep-index@1.0.0",
                    value,
                );
                SnapshotGroundingErrorKindV1::RedactionUncertain
            }
            "invalid_handoff_ref" => {
                port.advance(
                    "work",
                    "handoff",
                    "handbook.snapshot-observation.work.invalid-handoff@1.0.0",
                    json!({"refs":["secret=value"]}),
                );
                SnapshotGroundingErrorKindV1::MalformedInput
            }
            _ => unreachable!(),
        };

        let failure = owner
            .capture_boundary(
                &binding,
                &stream,
                &request("key-1", "session-1", SnapshotBoundaryKindV1::Start),
                &mut port,
            )
            .expect_err("unsafe projection input must fail closed");
        assert_eq!(failure.kind(), expected);
        assert_eq!(failure.accounting().len(), 15);
        assert_eq!(
            owner
                .open_repository(&binding, &stream)
                .expect("repository open")
                .head()
                .expect("head read"),
            None
        );
        let state_root = repo.path().join(".handbook/state/snapshot-memory/v1");
        for entry in walk_files(&state_root) {
            let bytes = std::fs::read(entry).expect("state bytes");
            assert!(!String::from_utf8_lossy(&bytes).contains("must-never-persist"));
        }
    }
}

fn request(key: &str, session: &str, kind: SnapshotBoundaryKindV1) -> SnapshotCaptureRequestV1 {
    SnapshotCaptureRequestV1::new(key, session, kind).expect("bounded request")
}

#[derive(Clone)]
struct CompletePort {
    observations: BTreeMap<(SnapshotSourceFamilyV1, &'static str), SnapshotSlotObservationV1>,
    values: BTreeMap<(SnapshotSourceFamilyV1, &'static str), Value>,
    observed: Vec<(SnapshotSourceFamilyV1, &'static str)>,
    reads: BTreeSet<(SnapshotSourceFamilyV1, &'static str)>,
    drift_after: Option<(SnapshotSourceFamilyV1, &'static str)>,
    substitute_after: Option<(SnapshotSourceFamilyV1, &'static str)>,
    fail_all_observations: bool,
}

impl CompletePort {
    fn successful() -> Self {
        let owner = SnapshotGroundingOwnerV1::new();
        let mut observations = BTreeMap::new();
        let mut values = BTreeMap::new();
        for contract in owner.source_contracts() {
            for &slot in contract.source_slots() {
                let reference = format!(
                    "handbook.snapshot-observation.{}.{}@1.0.0",
                    contract.family().as_str(),
                    slot.replace('_', "-")
                );
                let value = json!({"ref":reference});
                let fingerprint = DefinitionFingerprint::from_json_value(&value)
                    .expect("fixture fingerprint")
                    .to_string();
                observations.insert(
                    (contract.family(), slot),
                    SnapshotSlotObservationV1::retained(&reference, &fingerprint)
                        .expect("retained observation"),
                );
                values.insert((contract.family(), slot), value);
            }
        }
        observations.insert(
            (SnapshotSourceFamilyV1::Git, "worktree"),
            SnapshotSlotObservationV1::redacted(
                "handbook.snapshot-observation.git.worktree@1.0.0",
                &fingerprint(json!({"redacted":"RAW-WORKTREE-SECRET"})),
            )
            .expect("redacted observation"),
        );
        observations.insert(
            (SnapshotSourceFamilyV1::Evidence, "dock_execution_candidate"),
            SnapshotSlotObservationV1::omitted("absent_or_inapplicable"),
        );
        observations.insert(
            (SnapshotSourceFamilyV1::Work, "handoff"),
            SnapshotSlotObservationV1::inapplicable("policy_inapplicable"),
        );
        let contracts_decision = values
            .get(&(SnapshotSourceFamilyV1::Evidence, "contracts_decision"))
            .expect("contracts decision fixture")
            .clone();
        observations.insert(
            (SnapshotSourceFamilyV1::Evidence, "contracts_decision"),
            SnapshotSlotObservationV1::derived(
                "handbook.snapshot-observation.evidence.contracts-decision@1.0.0",
                &fingerprint(contracts_decision),
            )
            .expect("derived bounded fact"),
        );
        let resolution = json!({
            "resolution": {
                "stack_fingerprint": "sha256:4444444444444444444444444444444444444444444444444444444444444444",
                "stack_ref": "handbook.context-resolution-stack.test@1.0.0",
                "values": ["task", "repository", "session", "bounded", "execution", "verified"]
            }
        });
        let resolution_fingerprint = fingerprint(resolution.clone());
        observations.insert(
            (
                SnapshotSourceFamilyV1::Handbook,
                "schema_artifact_resolution",
            ),
            SnapshotSlotObservationV1::retained(
                "handbook.snapshot-observation.handbook.schema-artifact-resolution@1.0.0",
                &resolution_fingerprint,
            )
            .expect("Resolution observation"),
        );
        values.insert(
            (
                SnapshotSourceFamilyV1::Handbook,
                "schema_artifact_resolution",
            ),
            resolution,
        );
        values.insert(
            (SnapshotSourceFamilyV1::Git, "worktree"),
            json!({"secret":"RAW-WORKTREE-SECRET"}),
        );
        values.insert(
            (SnapshotSourceFamilyV1::Evidence, "dock_execution_candidate"),
            json!({"candidate":"RAW-CANDIDATE-SECRET"}),
        );
        values.insert(
            (SnapshotSourceFamilyV1::Work, "handoff"),
            json!({"handoff":"RAW-HANDOFF-SECRET"}),
        );
        Self {
            observations,
            values,
            observed: Vec::new(),
            reads: BTreeSet::new(),
            drift_after: None,
            substitute_after: None,
            fail_all_observations: false,
        }
    }

    fn advance(&mut self, family: &str, slot: &'static str, reference: &str, value: Value) {
        let family = SnapshotSourceFamilyV1::ALL
            .into_iter()
            .find(|candidate| candidate.as_str() == family)
            .expect("fixture family");
        let fingerprint = fingerprint(value.clone());
        self.observations.insert(
            (family, slot),
            SnapshotSlotObservationV1::retained(reference, &fingerprint)
                .expect("advanced retained observation"),
        );
        self.values.insert((family, slot), value);
    }

    fn reset_counters(&mut self) {
        self.observed.clear();
        self.reads.clear();
    }
}

impl SnapshotSourcePortV1 for CompletePort {
    fn observe_slot(
        &mut self,
        family: SnapshotSourceFamilyV1,
        slot: &'static str,
    ) -> Result<SnapshotSlotObservationV1, SnapshotSourcePortErrorV1> {
        if self.fail_all_observations {
            return Err(SnapshotSourcePortErrorV1::unavailable());
        }
        self.observed.push((family, slot));
        let mut observation = self
            .observations
            .get(&(family, slot))
            .cloned()
            .ok_or_else(SnapshotSourcePortErrorV1::missing)?;
        let count = self
            .observed
            .iter()
            .filter(|candidate| **candidate == (family, slot))
            .count();
        if count == 2 && self.drift_after == Some((family, slot)) {
            observation = observation.with_currentness_nonce_for_testing("changed");
        }
        if count == 2 && self.substitute_after == Some((family, slot)) {
            observation = SnapshotSlotObservationV1::retained(
                "handbook.snapshot-observation.substituted@1.0.0",
                &fingerprint(json!({"substituted":true})),
            )
            .expect("substituted fixture");
        }
        Ok(observation)
    }

    fn read_bounded_slot(
        &mut self,
        family: SnapshotSourceFamilyV1,
        slot: &'static str,
    ) -> Result<Value, SnapshotSourcePortErrorV1> {
        self.reads.insert((family, slot));
        self.values
            .get(&(family, slot))
            .cloned()
            .ok_or_else(SnapshotSourcePortErrorV1::missing)
    }
}

fn fingerprint(value: Value) -> String {
    DefinitionFingerprint::from_json_value(&value)
        .expect("fixture fingerprint")
        .to_string()
}

fn walk_files(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    if !root.exists() {
        return Vec::new();
    }
    let mut files = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        for entry in std::fs::read_dir(path).expect("state directory") {
            let path = entry.expect("state entry").path();
            if path.is_dir() {
                pending.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files
}
