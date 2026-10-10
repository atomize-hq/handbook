# File-backed graph consultations

Date: 2026-10-10. Requested scope: two independent ChatGPT Pro consultations, one reviewing the current Handbook repository and proposed storage/verification directions, the other retrieving the user's historical graph ideas from accessible account context.

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Consultations and evidence

| Consultation | Chat | Local record |
| --- | --- | --- |
| Architecture and performance | [Review Filebacked Graph Direction](https://chatgpt.com/c/6aca7955-56fc-83ea-82bd-67fafb5b1307) | [Exact prompt and bundle provenance](architecture-prompt.md); [complete captured answer](architecture-answer.md) |
| Historical graph ideas | [Recover File Based Graph Ideas](https://chatgpt.com/c/6aca7908-5afc-83e9-bb54-8fd9b24cb882) | [Exact prompt](history-prompt.md); [complete captured answer](history-answer.md) |

Both conversations used the UI's GPT-6 model with Pro effort. The architecture conversation received a ZIP of 1,386 eligible tracked files from baseline `8013948ebd3fbef6725f5d72f3d29120f7617009`. The skill excluded 390 archived/cache/instruction/sensitive-path entries; unavailable untracked files were not included. The architecture prompt explicitly disclosed these exclusions. The historical conversation was separate and did not receive the repository ZIP.

The invoked workflows were [ChatGPT Pro](C:/Users/spmcc/.codex/skills/chatgpt-pro/SKILL.md), [consult](C:/Users/spmcc/.codex/skills/chatgpt-pro-consult/SKILL.md), and [change exchange](C:/Users/spmcc/.codex/skills/chatgpt-pro-change-exchange/SKILL.md). The change-exchange builder supplied broad repository context; the user requested advice, so no patch was requested or applied.

## Architecture result and local verification

The consultant recommends retaining the file-first direction and operation-specific verification, using the existing typed evidence relationships as the graph. Its recommended sequence starts with reducing repeated native path/validation work, then proving complete indexes and publication/recovery over individual files. Segmentation remains a physical-layout option; SQLite remains conditional on a demonstrated practical need and equivalent native semantics.

One important code claim was independently checked against this local baseline: `work_state/store.rs` routes required reads through `optional`, which calls `RetainedRepositoryRootV1::read_local_owner_optional_bounded`. That method enumerates each parent directory beneath `.handbook` before the final read. The Windows directory-listing implementation inspects all listed children and opens regular-file handles for identity checks. This supports investigating repeated ancestor/sibling work; it does not measure the benefit or establish a safe replacement.

The full architecture capture was compared to the browser's copied response after newline normalization and trimming: 42,857 characters; rolling checksum 3,272,996,083. This is a transcription check, not a security claim.

No new benchmark or test run was performed in this consultation task. The consultant also expressly did not execute the Rust workspace. Its other code claims, proposed performance targets, new index/publication formats, and staged work remain advisory pending local reconciliation and bounded authorization. This task does not select a new storage backend, change identity/format contracts, adopt a production SLA, or authorize implementation.

## Historical result and coverage limits

The historical consultation completed after a temporary ChatGPT connection interruption. It reports searching personal-context retrieval and accessible Library/project files, following discovered terminology, and reading original documents for `memsrc`, C3, Substrate semantic units, HCM Snapshot Memory, and support-depth. It distinguishes these documents from retrieved conversation summaries and from assistant proposals. It could not enumerate all chats, saved memories, or native Projects; some Substrate archives could not be read. This is a substantial retrieval result, not an exhaustive account export.

The strongest connections to the current question, as reported by the consultant, are:

- **Support-depth:** complete, content-bound dependency sets and selective invalidation; unrelated changes need not invalidate a claim. The recovered contract is explicitly a draft.
- **HCM Snapshot Memory:** immutable content-addressed records, derived heads/indexes, journaling, recovery and retention reachability. The consultant verified historical documentation, not the implementation.
- **C3:** authoritative typed items and generated forward/reverse relationships and indexes.
- **`memsrc` and the May file-canonical concept:** source files compiled into deterministic graph artifacts, with more complex storage required to earn its role. `memsrc` does not itself supply a finished incremental verification protocol.
- **Lift:** a snapshot boundary intended to avoid repeated downstream filesystem acquisition; this lead has weaker provenance because it came from retrieved summaries of assistant proposals.

The historical answer does not recover a single approved implementation combining every requirement of the proposed Handbook design. It also does not convert older database suggestions into the user's current preference or conflate similarly named historical projects.

The full history capture was compared to the browser's copied response after newline normalization and trimming: 35,810 characters; rolling checksum 698,567,219. ChatGPT's file citations copy as source labels, so the original conversation remains the source for their interactive provenance. The captured support-depth consultation URL is preserved, with the consultant's explicit note that it did not open that complete conversation.

Both Pro tabs were retained as deliverables. Prompts and answers are saved here as research records; product code and roadmap/contract selections were not changed.
