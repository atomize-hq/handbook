# Charter Authoring Method

Author only the canonical charter at `.handbook/charter/CHARTER.md`.

Use only the deterministic structured-input entrypoint:

- `handbook author charter --validate --from-inputs <path|->` validates without mutation.
- `handbook author charter --from-inputs <path|->` renders and writes the canonical charter.

Method rules:

- Gather concrete repository facts into normalized structured charter inputs.
- Refuse or record an explicit known unknown when a required fact cannot be established; do not invent an answer.
- Treat the completed structured input document as the source of truth for deterministic rendering.
- Validate the input before mutation.
- Refuse instead of overwriting an existing non-starter canonical charter.
- Write only `.handbook/charter/CHARTER.md`; do not persist derived mirrors.

The final charter should stay short, concrete, and reusable. It defines durable engineering decision defaults, not feature behavior.
