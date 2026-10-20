Apply these instructions throughout this session.

Look beyond the immediate task to understand the outcome I want it to produce. Treat my requested approach as a starting hypothesis about how to achieve that outcome.

For each substantial task:
- Identify the immediate deliverable, the outcome it serves, and the decision or action that should follow.
- Consider whether a higher-level approach would achieve that outcome more effectively.
- Use available context and evidence to identify what changed, why it matters, and what should happen next.
- Surface the most consequential decisions, with practical options, tradeoffs, and supporting evidence.
- When work recurs, consider whether a durable workflow, feedback loop, or monitoring system would serve the goal better than repeated one-off tasks.
- Identify the data, tools, access, and verification needed to make that workflow reliable.

Briefly explain any proposed change in approach. Continue within my authorized scope; broader outcomes do not grant permission for additional external actions or ongoing automation.

Respect explicit deliverables and creative intent. Raise the level of abstraction when doing so helps accomplish what I actually want.

---

We are already working on this project task.

Task/spec: Review the completed file-native document/graph storage findings and scope the next bounded experiment around online cleanup, rolling tail compaction, and fresh-process lookup. Deliver a decision-ready implementation brief before we authorize worker dispatch. The desired outcome is sustainable storage and low-cost concurrent local-agent operation without losing readable files, exact accepted outcomes, or recovery.

Current approach: Keep the standalone Rust immutable disk COW core and improve its physical catalog/reader/maintenance protocol. Inference from code and measurements: operation-wide cleanup exclusion, a deliberately frozen packing cohort, and full relevant-index authentication on first lookup are the immediate obstacles. Challenge that diagnosis and the higher-level approach where justified.

Project sources: Read the attached bounded-review-packet.md in full: invariants and prior state, completed native and performance evidence, actual partial source excerpts, then the proposed experiment and six requested outputs. This is the complete available packet; excluded repository code is unavailable. No other conversation history or Project sources are needed.

Evidence, blockers, constraints, and validation: The pinned final core is d70f3864f2888d46d9112213b4a03f8d82745a8e. Cleanup held the writer lock about 187 seconds; packed fresh-process lookups took about 18-45 ms versus loose 1.0-1.4 ms; later commits remain outside the frozen packing cohort. Long-history compression/file reduction passed, but small-history file-reduction gates failed. Final native Windows/Linux/macOS recovery and the later B comparisons completed. Preserve both A live-view and B immutable-generation browsing contracts, ordinary YAML/JSON payloads, permanent accepted outcomes, retirement protections, whole-publication conflicts, and native platform checks. Keep protected originals and honest resource admission. The packet specifies remaining limitations and proposed work-based gates. This is advisory planning only; implementation, dispatch, migrations and ongoing automation remain separate decisions.

Do not reduce the task or replace it with an easier alternative. Preserve the requested scope and project conventions.

Is the current approach the best way forward? Give a direct keep, adjust, or pivot recommendation; tradeoffs; what to keep/change first; inspections; validation; and pitfalls.

Please address all six requested outputs at the end of the attachment, including a minimal viable protocol, state machine and crash matrix; one staged experiment versus ordered bounded tasks; a small discriminating measurement matrix; explicit correctness and proposed performance/resource gates; consequential human decisions; and a concise draft implementation brief. Distinguish hard invariants, measured facts, inferences, proposed targets, and unresolved questions. Do not change the historical gate results or claim device-cold/power-loss qualification from process tests. Explain proposed scope/order changes and preserve all three requested outcomes.
