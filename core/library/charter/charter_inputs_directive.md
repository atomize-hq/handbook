# Charter Intake Envelope Directive

Prepare one YAML document matching the shipped `handbook.intake.charter@1.0.0` closed envelope.

- Select exactly one mode: `guided_adaptive`, `express`, or `agent_assisted`.
- Put the proposed canonical Charter object in `content`.
- Include every required coverage identifier exactly once.
- For each coverage item, preserve source kind, value reference, evidence references, confidence, freshness, sensitivity, contradictions, and waiver reference.
- Identify the caller in `consumer`, retain prompt-event references, and supply an RFC 3339 UTC finalization time.
- For an amendment, bind `expected_current_fingerprint` to the selected canonical source fingerprint.
- Do not invent evidence, erase contradictions, choose authority, or output Markdown.

Submit the envelope with the matching explicit CLI mode. The engine performs semantic validation and records immutable lineage; this directive has no authority to write canonical truth.
