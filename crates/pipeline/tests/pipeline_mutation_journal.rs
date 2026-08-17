use handbook_pipeline::pipeline_mutation_journal::{
    PipelineMutationJournalLookup, PipelineMutationJournalSession,
};
use serde_json::json;

#[test]
fn journal_replays_only_the_same_semantic_request_without_retaining_raw_keys() {
    let repo = tempfile::tempdir().expect("temp repo");
    let raw_key = "packet-seven-secret-key";
    let request_fingerprint =
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let replay_payload = json!({"outcome":"applied","revision":1});

    let session = PipelineMutationJournalSession::open(
        repo.path(),
        "pipeline.state.apply@1.0.0",
        raw_key.as_bytes(),
    )
    .expect("journal session");
    assert!(matches!(
        session
            .lookup::<serde_json::Value>(request_fingerprint)
            .expect("lookup"),
        PipelineMutationJournalLookup::Miss
    ));
    session
        .persist(request_fingerprint, &replay_payload)
        .expect("persist");
    drop(session);

    let replay = PipelineMutationJournalSession::open(
        repo.path(),
        "pipeline.state.apply@1.0.0",
        raw_key.as_bytes(),
    )
    .expect("reopen");
    assert_eq!(
        replay
            .lookup::<serde_json::Value>(request_fingerprint)
            .expect("replay lookup"),
        PipelineMutationJournalLookup::Replay(replay_payload)
    );
    assert!(matches!(
        replay
            .lookup::<serde_json::Value>(
                "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
            )
            .expect("conflict lookup"),
        PipelineMutationJournalLookup::Conflict { .. }
    ));

    let journal_body = std::fs::read_dir(repo.path().join(".handbook/state/pipeline/idempotency"))
        .expect("journal root")
        .flat_map(|entry| std::fs::read_dir(entry.expect("operation dir").path()).expect("entries"))
        .map(|entry| std::fs::read_to_string(entry.expect("entry").path()).expect("journal body"))
        .collect::<String>();
    assert!(!journal_body.contains(raw_key));
    assert!(!journal_body.contains(repo.path().to_string_lossy().as_ref()));
}
