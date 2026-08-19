#![cfg(feature = "contract-schema-generation")]

use handbook_sdk::contract_membrane::{
    operation_inventory, AdmissionReason, DiscoveryState, OperationTransport, ProductionWitness,
    UtcTimestamp,
};
use std::collections::BTreeSet;

const RETENTION: &str = "2030-01-01T00:00:00Z";
const CLI_JSON_OPERATIONS: [&str; 18] = [
    "artifact.kind.list@1.0.0",
    "artifact.instance.list@1.0.0",
    "artifact.read@1.0.0",
    "artifact.validate@1.0.0",
    "intake.definition.read@1.0.0",
    "intake.coverage.evaluate@1.0.0",
    "intake.record.append@1.0.0",
    "artifact.candidate.validate@1.0.0",
    "artifact.candidate.append@1.0.0",
    "artifact.candidate.promote@1.0.0",
    "flow.resolve@1.0.0",
    "pipeline.catalog.list@1.0.0",
    "pipeline.catalog.read@1.0.0",
    "pipeline.compile@1.0.0",
    "pipeline.capture.plan@1.0.0",
    "pipeline.capture.apply@1.0.0",
    "pipeline.handoff.emit@1.0.0",
    "pipeline.state.apply@1.0.0",
];
const CONTRACT_OPERATIONS: [&str; 4] = [
    "contract.definition.list@1.0.0",
    "contract.definition.read@1.0.0",
    "contract.definition.append@1.0.0",
    "contract.lifecycle.transition@1.0.0",
];
const USE_CASES: [&str; 6] = [
    "handbook.use-case.repository-setup@1.0.0",
    "handbook.use-case.repository-doctor@1.0.0",
    "handbook.use-case.author-charter@1.0.0",
    "handbook.use-case.author-project-context@1.0.0",
    "handbook.use-case.approver-administration@1.0.0",
    "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
];
const USE_CASE_SCHEMAS: [&str; 12] = [
    "handbook.use-case.repository-setup-result@1.0.0",
    "handbook.use-case.repository-setup-details@1.0.0",
    "handbook.use-case.repository-doctor-result@1.0.0",
    "handbook.use-case.repository-doctor-details@1.0.0",
    "handbook.use-case.author-charter-result@1.0.0",
    "handbook.use-case.author-charter-details@1.0.0",
    "handbook.use-case.author-project-context-result@1.0.0",
    "handbook.use-case.author-project-context-details@1.0.0",
    "handbook.use-case.approver-administration-result@1.0.0",
    "handbook.use-case.approver-administration-details@1.0.0",
    "handbook.use-case.pipeline-route-resolve-and-persist-result@1.0.0",
    "handbook.use-case.pipeline-route-resolve-and-persist-details@1.0.0",
];

#[test]
fn inventory_discovery_use_case_omission_and_schema_roots_close_exactly() {
    let rows = operation_inventory();
    assert_eq!(rows.len(), 62);
    assert_eq!(
        rows.iter()
            .map(|row| row.operation_ref().as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        62
    );

    let predecessor_cli = CLI_JSON_OPERATIONS.into_iter().collect::<BTreeSet<_>>();
    let expected_cli = predecessor_cli
        .iter()
        .copied()
        .chain(CONTRACT_OPERATIONS)
        .collect::<BTreeSet<_>>();
    let actual_cli = rows
        .iter()
        .filter(|row| {
            row.current_transports()
                .contains(&OperationTransport::CliJson)
        })
        .map(|row| row.operation_ref().as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_cli, expected_cli);
    assert_eq!(expected_cli.len(), 22);
    assert!(predecessor_cli.is_subset(&actual_cli));

    let expected_discoverable = CLI_JSON_OPERATIONS
        .into_iter()
        .chain(CONTRACT_OPERATIONS)
        .chain(["posture.transition.apply@1.0.0"])
        .collect::<BTreeSet<_>>();
    let actual_discoverable = rows
        .iter()
        .filter(|row| row.discovery() == DiscoveryState::Discoverable)
        .map(|row| row.operation_ref().as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_discoverable, expected_discoverable);
    assert_eq!(expected_discoverable.len(), 23);

    let route_resolve = rows
        .iter()
        .find(|row| row.operation_ref().as_str() == "pipeline.route.resolve@1.0.0")
        .unwrap();
    assert_eq!(
        route_resolve.admission_reason(),
        AdmissionReason::SemanticMismatch
    );
    assert!(route_resolve.current_transports().is_empty());
    assert_eq!(route_resolve.discovery(), DiscoveryState::Omit);

    let inventory_refs = rows
        .iter()
        .map(|row| row.operation_ref().as_str())
        .collect::<BTreeSet<_>>();
    assert!(USE_CASES
        .iter()
        .all(|use_case| !inventory_refs.contains(use_case)));

    let witness = ProductionWitness::from_current(UtcTimestamp::parse(RETENTION).unwrap()).unwrap();
    let capability_refs = witness
        .capabilities()
        .iter()
        .map(|entry| entry.operation().reference().as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(capability_refs, expected_discoverable);
    assert!(USE_CASES
        .iter()
        .all(|use_case| !capability_refs.contains(use_case)));

    let manifest_refs = witness
        .schema_manifest()
        .iter()
        .map(|entry| entry.schema().reference().as_str())
        .collect::<BTreeSet<_>>();
    for root in [
        "handbook.invocation-response@1.0.0",
        "handbook.use-case-response@1.0.0",
        "handbook.transport-failure@1.0.0",
        "handbook.transport-failure-details@1.0.0",
    ]
    .into_iter()
    .chain(USE_CASE_SCHEMAS)
    {
        assert!(
            manifest_refs.contains(root),
            "missing public schema root {root}"
        );
    }
    for capability in witness.capabilities().iter().filter(|entry| {
        entry
            .current_transports()
            .contains(&OperationTransport::CliJson)
    }) {
        for binding in [
            capability.request_schema(),
            capability.result_schema(),
            capability.outcome_schemas().blocker(),
            capability.outcome_schemas().refusal(),
            capability.outcome_schemas().error(),
        ] {
            assert!(
                manifest_refs.contains(binding.reference().as_str()),
                "missing schema {} for {}",
                binding.reference().as_str(),
                capability.operation().reference().as_str()
            );
        }
    }
}
