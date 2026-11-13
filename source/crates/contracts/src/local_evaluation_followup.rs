//! Explicit fresh checks preserve their original closed truth in additive lineage.
use crate::canonical::{canonical_bytes, canonical_fingerprint};
use crate::*;
use handbook_engine::local_authority::local_check::LocalCheckCurrentBasisV1;
use handbook_engine::local_authority::local_evaluation::LocalEvaluationFormationRequestV1;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalEvaluationFollowupActionV1 {
    FreshRepeat,
    CorrectedInput,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationFollowupInputV1 {
    pub action: LocalEvaluationFollowupActionV1,
    pub original: LocalEvaluationResultLocatorV3,
    /// Complete exact inline captured input, including its offline schema closure.
    pub original_capture: Value,
    pub capture: Value,
    pub input: LocalEvaluationPrepareInputV3,
}
impl LocalEvaluationFollowupInputV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, ContractError> {
        let bytes = canonical_bytes(self)?;
        if bytes.len() > 1024 * 1024 {
            return Err(ContractError::BoundExceeded);
        }
        Ok(bytes)
    }
    /// One bounded duplicate-safe document, followed by exact canonical framing.
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, ContractError> {
        if bytes.len() > 1024 * 1024 {
            return Err(ContractError::BoundExceeded);
        }
        let raw = handbook_engine::parse_schema_json(bytes)
            .map_err(|_| ContractError::InvalidInvariant)?;
        let value: Self =
            serde_json::from_value(raw).map_err(|_| ContractError::InvalidInvariant)?;
        if value.canonical_json_bytes()? != bytes {
            return Err(ContractError::DerivedValueMismatch);
        }
        Ok(value)
    }
}

/// This preparation is semantic-owner validation, not execution authority.
#[derive(Clone, Debug)]
pub struct PreparedLocalEvaluationFollowupV1 {
    pub(crate) followup: LocalEvaluationFollowupInputV1,
    pub(crate) prepared: PreparedLocalEvaluationV3,
}
impl PreparedLocalEvaluationFollowupV1 {
    pub fn input(&self) -> &LocalEvaluationFollowupInputV1 {
        &self.followup
    }
    pub fn prepared(&self) -> &PreparedLocalEvaluationV3 {
        &self.prepared
    }
    pub fn formation_request(&self) -> &LocalEvaluationFormationRequestV1 {
        self.prepared.formation_request()
    }
    pub fn current_basis(&self) -> &LocalCheckCurrentBasisV1 {
        self.prepared.current_basis()
    }
    pub fn request_locator(&self) -> LocalEvaluationRequestLocatorV3 {
        self.prepared.request_locator()
    }
}

/// Actual refusal reproduction is completed before acquiring Engine's guard.
#[derive(Clone, Debug)]
pub struct PreparedLocalInputCorrectionCandidateV1 {
    pub(crate) followup: LocalEvaluationFollowupInputV1,
    pub(crate) rejected: LocalEvaluationPrepareInputV3,
    pub(crate) request: LocalEvaluationRequestLocatorV3,
    pub(crate) basis: LocalCheckCurrentBasisV1,
    pub(crate) original_state: LocalEvaluationSnapshotV3,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationLineageV1 {
    pub schema_ref: String,
    pub action: LocalEvaluationFollowupActionV1,
    pub original: LocalEvaluationResultLocatorV3,
    pub original_capture: Value,
    pub capture: Value,
    pub evaluation: LocalEvaluationLocatorV3,
    /// Exact retained decision and request data; attribution is local advisory.
    pub decision_provenance: Option<Value>,
    pub dependencies: Vec<LocalEvaluationRetentionDependencyV3>,
    pub retained_until: SemanticTimeV1,
    pub identity: ContentAddressedBindingV1,
}
impl LocalEvaluationLineageV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, ContractError> {
        self.validate_identity()?;
        let bytes = canonical_bytes(self)?;
        if bytes.len() > 1024 * 1024 {
            return Err(ContractError::BoundExceeded);
        }
        Ok(bytes)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, ContractError> {
        if bytes.len() > 1024 * 1024 {
            return Err(ContractError::BoundExceeded);
        }
        let raw = handbook_engine::parse_schema_json(bytes)
            .map_err(|_| ContractError::InvalidInvariant)?;
        let value: Self =
            serde_json::from_value(raw).map_err(|_| ContractError::InvalidInvariant)?;
        if value.canonical_json_bytes()? != bytes {
            return Err(ContractError::DerivedValueMismatch);
        }
        Ok(value)
    }
    pub(crate) fn validate_identity(&self) -> Result<(), ContractError> {
        if self.schema_ref != "handbook.contract-local-evaluation-lineage@1.0.0"
            || self.original.evaluation.occurrence == self.evaluation.occurrence
            || self.original.evaluation.request_key == self.evaluation.request_key
            || self.original.evaluation.owner_identity != self.evaluation.owner_identity
            || self.original.evaluation.lock_generation != self.evaluation.lock_generation
            || (self.action == LocalEvaluationFollowupActionV1::FreshRepeat)
                != (canonical_fingerprint(&self.capture)?
                    == canonical_fingerprint(&self.original_capture)?)
            || (self.action == LocalEvaluationFollowupActionV1::FreshRepeat
                && self.decision_provenance.is_some())
            || self
                .dependencies
                .iter()
                .any(|d| d.retained_until < self.retained_until)
            || self.identity
                != identified(
                    "handbook.contract-local-evaluation-lineage@1.0.0",
                    &without_identity(self)?,
                )?
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(())
    }
}

/// Reproduce the selected native capture binding without depending on Dock.
pub(crate) fn validate_followup_capture(
    capture: &Value,
    subject: &EvaluationSubjectV1,
    case: &EvaluationCaseV1,
    dependencies: &LocalDefinitionDependenciesV2,
) -> Result<(), ContractError> {
    let object = capture.as_object().ok_or(ContractError::InvalidInvariant)?;
    if object.len() != 5
        || object.keys().any(|key| {
            ![
                "role",
                "media_type",
                "schema",
                "content",
                "content_fingerprint",
            ]
            .contains(&key.as_str())
        })
        || canonical_bytes(capture)?.len() > 8 * 1024 * 1024
    {
        return Err(ContractError::InvalidInvariant);
    }
    let role = capture["role"]
        .as_str()
        .ok_or(ContractError::InvalidInvariant)?;
    let media = capture["media_type"]
        .as_str()
        .ok_or(ContractError::InvalidInvariant)?;
    let schema: ExactBindingV1 = serde_json::from_value(capture["schema"].clone())
        .map_err(|_| ContractError::InvalidInvariant)?;
    let content = &capture["content"];
    if role != "handbook.input.json-schema-validation"
        || dependencies.validator.canonical_manifest["supported_inputs"]
            != serde_json::json!([{"role":role,"media_type":media,"schema":schema}])
        || capture["content_fingerprint"] != canonical_fingerprint(content)?.as_str()
        || serde_json::json!({"root_schema":content.get("root_schema"),
            "root_document":content.get("root_document"),"documents":content.get("documents"),
            "closure_fingerprint":content.get("closure_fingerprint")})
            != serde_json::to_value(&dependencies.schema_closure)
                .map_err(|_| ContractError::InvalidInvariant)?
        || content["instance"]["instance_fingerprint"]
            != canonical_fingerprint(&content["instance"]["value"])?.as_str()
    {
        return Err(ContractError::InvalidInvariant);
    }
    let fp = canonical_fingerprint(capture)?;
    let binding = ContentAddressedBindingV1::new(
        &format!(
            "handbook.dock.captured-input@1.0.0#role/{role}/{}",
            fp.as_str().trim_start_matches("sha256:")
        ),
        fp,
    )?;
    let matcher = [
        ("dock.input.role", role),
        ("dock.input.media-type", media),
        ("dock.input.schema", schema.reference().as_str()),
    ]
    .into_iter()
    .map(|(field, value)| MatcherInputValueV1::new(StableIdV1::parse(field)?, value))
    .collect::<Result<Vec<_>, _>>()?;
    if *subject != EvaluationSubjectV1::new(binding.clone(), matcher.clone())?
        || *case != EvaluationCaseV1::new(case.case_id().clone(), binding, matcher)?
    {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn binding(name: &str) -> ContentAddressedBindingV1 {
        ContentAddressedBindingV1::new(name, FingerprintV1::from_bytes(name.as_bytes())).unwrap()
    }
    fn lineage() -> LocalEvaluationLineageV1 {
        let original = LocalEvaluationLocatorV3 {
            request_key: "original".into(),
            request_fingerprint: FingerprintV1::from_bytes(b"original request"),
            occurrence: binding("handbook.contract-local-occurrence@3.0.0#original"),
            owner_identity: FingerprintV1::from_bytes(b"owner"),
            lock_generation: "original lock".into(),
        };
        let mut evaluation = original.clone();
        evaluation.request_key = "repeat".into();
        evaluation.request_fingerprint = FingerprintV1::from_bytes(b"repeat request");
        evaluation.occurrence = binding("handbook.contract-local-occurrence@3.0.0#repeat");
        let mut value = LocalEvaluationLineageV1 {
            schema_ref: "handbook.contract-local-evaluation-lineage@1.0.0".into(),
            action: LocalEvaluationFollowupActionV1::FreshRepeat,
            original: LocalEvaluationResultLocatorV3 {
                evaluation: original,
                result: binding("handbook.local-evaluation-result@3.0.0#original"),
            },
            original_capture: serde_json::json!({"nested":{"amount":-1}}),
            capture: serde_json::json!({"nested":{"amount":-1}}),
            evaluation,
            decision_provenance: None,
            dependencies: vec![],
            retained_until: SemanticTimeV1::parse("2026-11-07T00:00:00Z").unwrap(),
            identity: binding("handbook.contract-local-evaluation-lineage@1.0.0#temporary"),
        };
        value.identity = identified(
            "handbook.contract-local-evaluation-lineage@1.0.0",
            &without_identity(&value).unwrap(),
        )
        .unwrap();
        value
    }
    #[test]
    fn lineage_codec_preserves_exact_bytes_and_rejects_nested_duplicates_and_extra_documents() {
        let original = lineage();
        let bytes = original.canonical_json_bytes().unwrap();
        assert_eq!(
            LocalEvaluationLineageV1::from_json_bytes(&bytes).unwrap(),
            original
        );
        let json = std::str::from_utf8(&bytes).unwrap();
        let duplicated = json.replacen("\"amount\":-1", "\"amount\":-1,\"amount\":-1", 1);
        assert!(LocalEvaluationLineageV1::from_json_bytes(duplicated.as_bytes()).is_err());
        assert!(
            LocalEvaluationLineageV1::from_json_bytes(format!("{json} {{}}").as_bytes()).is_err()
        );
        assert!(LocalEvaluationLineageV1::from_json_bytes(&vec![b' '; 1024 * 1024 + 1]).is_err());
        let mut substituted = original;
        substituted.capture["nested"]["amount"] = serde_json::json!(125);
        assert!(substituted.canonical_json_bytes().is_err());
    }

    #[test]
    fn equal_canonical_capture_is_a_fresh_repeat_even_with_different_number_representation() {
        let original = lineage();
        let mut equal = original.clone();
        equal.capture["nested"]["amount"] = serde_json::json!(-1.0);
        equal.identity = identified(
            "handbook.contract-local-evaluation-lineage@1.0.0",
            &without_identity(&equal).unwrap(),
        )
        .unwrap();
        assert_eq!(
            equal.canonical_json_bytes().unwrap(),
            original.canonical_json_bytes().unwrap()
        );
        equal.action = LocalEvaluationFollowupActionV1::CorrectedInput;
        equal.identity = identified(
            "handbook.contract-local-evaluation-lineage@1.0.0",
            &without_identity(&equal).unwrap(),
        )
        .unwrap();
        assert!(equal.canonical_json_bytes().is_err());
    }
}
