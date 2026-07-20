# Charter Authoring Method

The Charter is selected canonical YAML at `.handbook/project/charter.yaml`. The native engine owns its definitions, immutable lineage, review rendering, approvals, and atomic promotion.

Choose exactly one acquisition mode and submit one closed typed intake envelope:

- `handbook author charter --mode guided-adaptive --from-inputs <path|->`
- `handbook author charter --mode express --from-inputs <path|->`
- `handbook author charter --mode agent-assisted --from-inputs <path|->`

Acquisition records immutable intake and candidate lineage. It never selects or rewrites canonical truth. The three modes differ only in how caller-owned evidence is gathered; all converge on the same engine intake contract and validation.

Approval and promotion are separate native-authority operations:

- `handbook author charter --approve-candidate <candidate-ref> --approval-class <class> --authority-ref <authority-ref>`
- `handbook author charter --promote-candidate <candidate-ref> --approval-ref <approval-ref>`

Repeat `--accept-waiver-ref <waiver-ref>` on approval only when the exact candidate requires an accepted waiver. Use `--expected-current-fingerprint <sha256:...>` for amendment acquisition or promotion compare-and-swap intent.

Validate selected truth without mutation:

- `handbook author charter --validate`

Method rules:

- Gather concrete repository facts and evidence. Preserve explicit unknowns and contradictions.
- Treat the completed structured input document as the source of truth for caller intent only; engine-owned immutable lineage and selected canonical truth remain authoritative.
- Do not invoke a secondary model, run an interactive wizard, select an approver, or synthesize authority.
- Treat returned intake, candidate, approval, and promotion references as immutable identities.
- Require human approval for every required class before promotion.
- Never edit canonical Charter YAML or any rendered Markdown view directly.
- Use `--json` when consuming results programmatically and stop on typed refusals.

The rendered Charter is a deterministic review projection. It is never an authoring or persistence surface.
