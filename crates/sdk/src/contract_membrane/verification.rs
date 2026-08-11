use super::catalog::{cursor_fingerprint, page_fingerprint, validate_page_local};
use super::{
    canonical_json_bytes, ArtifactLocator, ArtifactLocatorRef, ArtifactRef, BoundedString,
    BoundedVec, CatalogCursor, CatalogPage, CatalogRoot, ContractError, ContractLimits,
    ContractMember, ContractValue, ContractValueRef, CorrelationRef, Diagnostic,
    DiagnosticSeverity, DigestAlgorithm, DraftVersion, ExactBinding, ExactRef, Fingerprint,
    IdempotencyStage, IdempotencyState, IdempotencyStateRef, NextAction, NextActionKind, Omission,
    OmissionReason, Problem, ProblemBinding, ProblemCategory, ProofEffect, RecheckCondition,
    RetryDirective, SafeI64, SafeU64, SchemaManifestEntry, SchemaMediaType, SemVer, Sensitivity,
    SourceBinding, TerminalFields, TerminalStatus, UtcTimestamp, WriteReceipt,
};
use serde::Serialize;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerificationError {
    WitnessUnavailable,
    BindingRejected,
    SchemaRejected,
    ArtifactRejected,
    CatalogRejected,
    DerivedFingerprintMismatch,
    ContractInvariant,
}

impl fmt::Display for VerificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::WitnessUnavailable => "contract witness is unavailable",
            Self::BindingRejected => "contract binding was rejected",
            Self::SchemaRejected => "schema instance was rejected",
            Self::ArtifactRejected => "artifact was rejected",
            Self::CatalogRejected => "catalog was rejected",
            Self::DerivedFingerprintMismatch => "derived fingerprint does not match",
            Self::ContractInvariant => "verified contract invariant is invalid",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for VerificationError {}

#[doc(hidden)]
pub(crate) trait ContractWitnessSealed {}

/// A sealed owner capability for upgrading structural values to trusted ones.
///
/// Packet 0 deliberately supplies no production implementation. Downstream
/// crates cannot manufacture an always-success witness:
///
/// ```compile_fail
/// use handbook_sdk::contract_membrane::{
///     ArtifactRef, CatalogPage, CatalogRoot, ContractValue, ContractWitness,
///     ExactBinding, VerificationError,
/// };
///
/// struct AlwaysAllow;
/// impl ContractWitness for AlwaysAllow {
///     fn verify_binding(&self, _: &ExactBinding) -> Result<(), VerificationError> { Ok(()) }
///     fn verify_schema_instance(
///         &self,
///         _: &ExactBinding,
///         _: &ContractValue,
///     ) -> Result<(), VerificationError> { Ok(()) }
///     fn verify_artifact(&self, _: &ArtifactRef) -> Result<(), VerificationError> { Ok(()) }
///     fn verify_catalog_root(&self, _: &CatalogRoot) -> Result<(), VerificationError> { Ok(()) }
///     fn verify_catalog_page(&self, _: &CatalogPage) -> Result<(), VerificationError> { Ok(()) }
/// }
/// ```
#[allow(private_bounds)]
pub trait ContractWitness: ContractWitnessSealed {
    fn verify_binding(&self, binding: &ExactBinding) -> Result<(), VerificationError>;

    fn verify_schema_instance(
        &self,
        schema: &ExactBinding,
        instance: &ContractValue,
    ) -> Result<(), VerificationError>;

    fn verify_artifact(&self, artifact: &ArtifactRef) -> Result<(), VerificationError>;

    fn verify_catalog_root(&self, root: &CatalogRoot) -> Result<(), VerificationError>;

    fn verify_catalog_page(&self, page: &CatalogPage) -> Result<(), VerificationError>;
}

pub trait VerifyContract {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Verified<T> {
    value: T,
}

impl<T> Verified<T> {
    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

pub fn verify<T: VerifyContract>(
    value: T,
    witness: &dyn ContractWitness,
    limits: &ContractLimits,
) -> Result<Verified<T>, VerificationError> {
    value.verify_contract(witness, limits)?;
    Ok(Verified { value })
}

impl<const N: usize> VerifyContract for BoundedString<N> {
    fn verify_contract(
        &self,
        _witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        if N > 65_536 || self.as_str().len() > limits.string_bytes() {
            return Err(VerificationError::ContractInvariant);
        }
        Ok(())
    }
}

impl<T: VerifyContract, const N: usize> VerifyContract for BoundedVec<T, N> {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        if N > 128 || self.as_slice().len() > limits.array_items() {
            return Err(VerificationError::ContractInvariant);
        }
        verify_each(self.as_slice(), witness, limits)
    }
}

macro_rules! verify_serialized_leaf {
    ($($type:ty),+ $(,)?) => {
        $(
            impl VerifyContract for $type {
                fn verify_contract(
                    &self,
                    _witness: &dyn ContractWitness,
                    limits: &ContractLimits,
                ) -> Result<(), VerificationError> {
                    verify_serialized(self, limits)
                }
            }
        )+
    };
}

verify_serialized_leaf!(
    SafeU64,
    SafeI64,
    UtcTimestamp,
    SemVer,
    ExactRef,
    Fingerprint,
    DigestAlgorithm,
    Sensitivity,
    ProblemCategory,
    DiagnosticSeverity,
    RetryDirective,
    NextActionKind,
    OmissionReason,
    ProofEffect,
    DraftVersion,
    SchemaMediaType,
    TerminalStatus,
    IdempotencyStage,
);

impl VerifyContract for ExactBinding {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        verify_serialized(self, limits)?;
        witness.verify_binding(self)
    }
}

impl VerifyContract for ContractMember {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        self.key().verify_contract(witness, limits)?;
        self.value().verify_contract(witness, limits)
    }
}

impl VerifyContract for ContractValue {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        verify_serialized(self, limits)?;
        match self.as_ref() {
            ContractValueRef::Array(values) => verify_each(values, witness, limits),
            ContractValueRef::Object(members) => verify_each(members, witness, limits),
            ContractValueRef::Null
            | ContractValueRef::Bool(_)
            | ContractValueRef::String(_)
            | ContractValueRef::Integer(_) => Ok(()),
        }
    }
}

impl VerifyContract for ArtifactLocator {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        verify_serialized(self, limits)?;
        match self.as_ref() {
            ArtifactLocatorRef::RepoRelative { .. } => Ok(()),
            ArtifactLocatorRef::ContentAddressed { store, .. } => {
                store.verify_contract(witness, limits)
            }
        }
    }
}

impl VerifyContract for ArtifactRef {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        ArtifactRef::new(
            self.artifact().clone(),
            self.media_type().clone(),
            self.byte_length(),
            self.sensitivity(),
            self.locator().clone(),
        )
        .map_err(contract_invariant)?;
        verify_serialized(self, limits)?;
        self.artifact().verify_contract(witness, limits)?;
        self.locator().verify_contract(witness, limits)?;
        witness.verify_artifact(self)
    }
}

impl VerifyContract for RecheckCondition {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        let recomputed = RecheckCondition::new(
            self.condition_schema().clone(),
            self.condition().clone(),
            limits,
        )
        .map_err(contract_invariant)?;
        if recomputed.condition_fingerprint() != self.condition_fingerprint() {
            return Err(VerificationError::DerivedFingerprintMismatch);
        }
        verify_schema_value(self.condition_schema(), self.condition(), witness, limits)
    }
}

impl VerifyContract for Problem {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        let recomputed = Problem::new(
            self.problem_id().clone(),
            self.code().clone(),
            self.category(),
            self.subject().clone(),
            self.rule().cloned(),
            self.details_schema().clone(),
            self.details().clone(),
            self.evidence().clone(),
            self.retry(),
            self.recheck().cloned(),
            self.correlation_ref().cloned(),
            limits,
        )
        .map_err(contract_invariant)?;
        if recomputed.details_fingerprint() != self.details_fingerprint()
            || recomputed.problem_fingerprint() != self.problem_fingerprint()
        {
            return Err(VerificationError::DerivedFingerprintMismatch);
        }

        self.subject().verify_contract(witness, limits)?;
        if let Some(rule) = self.rule() {
            rule.verify_contract(witness, limits)?;
        }
        verify_schema_value(self.details_schema(), self.details(), witness, limits)?;
        verify_each(self.evidence().as_slice(), witness, limits)?;
        if let Some(recheck) = self.recheck() {
            recheck.verify_contract(witness, limits)?;
        }
        Ok(())
    }
}

impl VerifyContract for Diagnostic {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        let recomputed = Diagnostic::new(
            self.code().clone(),
            self.severity(),
            self.subject().clone(),
            self.details_schema().clone(),
            self.details().clone(),
            self.display_message().cloned(),
            limits,
        )
        .map_err(contract_invariant)?;
        if recomputed.details_fingerprint() != self.details_fingerprint() {
            return Err(VerificationError::DerivedFingerprintMismatch);
        }
        self.subject().verify_contract(witness, limits)?;
        verify_schema_value(self.details_schema(), self.details(), witness, limits)
    }
}

impl VerifyContract for NextAction {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        NextAction::new(
            self.action_id().clone(),
            self.kind(),
            self.subject().clone(),
            self.parameter_schema().clone(),
            self.parameters().clone(),
            self.display_label().cloned(),
            limits,
        )
        .map_err(contract_invariant)?;
        self.subject().verify_contract(witness, limits)?;
        verify_schema_value(self.parameter_schema(), self.parameters(), witness, limits)
    }
}

impl VerifyContract for SourceBinding {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        verify_serialized(self, limits)?;
        self.source().verify_contract(witness, limits)?;
        self.captured_revision().verify_contract(witness, limits)?;
        self.adapter().verify_contract(witness, limits)
    }
}

impl VerifyContract for Omission {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        verify_serialized(self, limits)?;
        self.item().verify_contract(witness, limits)?;
        if let Some(source) = self.source() {
            source.verify_contract(witness, limits)?;
        }
        Ok(())
    }
}

impl VerifyContract for SchemaManifestEntry {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        verify_serialized(self, limits)?;
        self.schema().verify_contract(witness, limits)
    }
}

impl VerifyContract for CorrelationRef {
    fn verify_contract(
        &self,
        _witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        verify_serialized(self, limits)
    }
}

impl VerifyContract for WriteReceipt {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        let recomputed = WriteReceipt::new(
            self.record_kind().clone(),
            self.record().clone(),
            self.authority_class().clone(),
            self.condition().clone(),
            self.atomic_group().clone(),
            limits,
        )
        .map_err(contract_invariant)?;
        if recomputed.receipt_fingerprint() != self.receipt_fingerprint() {
            return Err(VerificationError::DerivedFingerprintMismatch);
        }
        self.record().verify_contract(witness, limits)?;
        self.condition().verify_contract(witness, limits)
    }
}

impl VerifyContract for ProblemBinding {
    fn verify_contract(
        &self,
        _witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        verify_serialized(self, limits)
    }
}

impl VerifyContract for CatalogRoot {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        verify_serialized(self, limits)?;
        self.catalog().verify_contract(witness, limits)?;
        self.entry_schema().verify_contract(witness, limits)?;
        self.canonical_sort().verify_contract(witness, limits)?;
        witness.verify_catalog_root(self)
    }
}

impl VerifyContract for CatalogCursor {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        verify_serialized(self, limits)?;
        let recomputed = cursor_fingerprint(self.catalog(), self.next_sort_key(), limits)
            .map_err(contract_invariant)?;
        if &recomputed != self.cursor_fingerprint() {
            return Err(VerificationError::DerivedFingerprintMismatch);
        }
        self.catalog().verify_contract(witness, limits)?;
        self.next_sort_key().verify_contract(witness, limits)
    }
}

impl VerifyContract for CatalogPage {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        verify_serialized(self, limits)?;
        validate_page_local(
            self.root(),
            self.entries().as_slice(),
            self.next_cursor(),
            limits,
        )
        .map_err(contract_invariant)?;
        let recomputed = page_fingerprint(self.root(), self.entries(), self.next_cursor(), limits)
            .map_err(contract_invariant)?;
        if &recomputed != self.page_fingerprint() {
            return Err(VerificationError::DerivedFingerprintMismatch);
        }

        self.root().verify_contract(witness, limits)?;
        for entry in self.entries().as_slice() {
            verify_schema_value(self.root().entry_schema(), entry, witness, limits)?;
        }
        if let Some(cursor) = self.next_cursor() {
            cursor.verify_contract(witness, limits)?;
        }
        witness.verify_catalog_page(self)
    }
}

impl<T: VerifyContract + Serialize> VerifyContract for TerminalFields<T> {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        verify_serialized(self, limits)?;
        if let Some(data) = self.data() {
            data.verify_contract(witness, limits)?;
        }
        verify_each(self.blockers(), witness, limits)?;
        verify_each(self.refusals(), witness, limits)?;
        verify_each(self.errors(), witness, limits)
    }
}

impl VerifyContract for IdempotencyState {
    fn verify_contract(
        &self,
        witness: &dyn ContractWitness,
        limits: &ContractLimits,
    ) -> Result<(), VerificationError> {
        verify_serialized(self, limits)?;
        match self.as_ref() {
            IdempotencyStateRef::NotApplicable => {}
            IdempotencyStateRef::NotEstablished {
                terminal_problem, ..
            } => terminal_problem.verify_contract(witness, limits)?,
            IdempotencyStateRef::Established {
                replayed,
                original_result_fingerprint,
                ..
            } => {
                if replayed != original_result_fingerprint.is_some() {
                    return Err(VerificationError::ContractInvariant);
                }
            }
        }
        Ok(())
    }
}

fn verify_schema_value(
    schema: &ExactBinding,
    value: &ContractValue,
    witness: &dyn ContractWitness,
    limits: &ContractLimits,
) -> Result<(), VerificationError> {
    schema.verify_contract(witness, limits)?;
    value.verify_contract(witness, limits)?;
    witness.verify_schema_instance(schema, value)
}

fn verify_each<T: VerifyContract>(
    values: &[T],
    witness: &dyn ContractWitness,
    limits: &ContractLimits,
) -> Result<(), VerificationError> {
    for value in values {
        value.verify_contract(witness, limits)?;
    }
    Ok(())
}

fn verify_serialized<T: Serialize>(
    value: &T,
    limits: &ContractLimits,
) -> Result<(), VerificationError> {
    canonical_json_bytes(value, limits)
        .map(|_| ())
        .map_err(contract_invariant)
}

fn contract_invariant(_error: ContractError) -> VerificationError {
    VerificationError::ContractInvariant
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract_membrane::sha256_fingerprint;
    use std::cell::Cell;

    #[derive(Default)]
    struct FixtureWitness {
        reject_binding_identity: Option<&'static str>,
        reject_schema_identity: Option<&'static str>,
        reject_artifact_identity: Option<&'static str>,
        catalog_root_available: bool,
        catalog_page_available: bool,
    }

    impl ContractWitnessSealed for FixtureWitness {}

    impl ContractWitness for FixtureWitness {
        fn verify_binding(&self, binding: &ExactBinding) -> Result<(), VerificationError> {
            if self.reject_binding_identity == Some(binding.reference().identity()) {
                Err(VerificationError::BindingRejected)
            } else {
                Ok(())
            }
        }

        fn verify_schema_instance(
            &self,
            schema: &ExactBinding,
            _instance: &ContractValue,
        ) -> Result<(), VerificationError> {
            if self.reject_schema_identity == Some(schema.reference().identity()) {
                Err(VerificationError::SchemaRejected)
            } else {
                Ok(())
            }
        }

        fn verify_artifact(&self, artifact: &ArtifactRef) -> Result<(), VerificationError> {
            if self.reject_artifact_identity == Some(artifact.artifact().reference().identity()) {
                Err(VerificationError::ArtifactRejected)
            } else {
                Ok(())
            }
        }

        fn verify_catalog_root(&self, _root: &CatalogRoot) -> Result<(), VerificationError> {
            if self.catalog_root_available {
                Ok(())
            } else {
                Err(VerificationError::WitnessUnavailable)
            }
        }

        fn verify_catalog_page(&self, _page: &CatalogPage) -> Result<(), VerificationError> {
            if self.catalog_page_available {
                Ok(())
            } else {
                Err(VerificationError::WitnessUnavailable)
            }
        }
    }

    struct ExactFixtureWitness;

    impl ContractWitnessSealed for ExactFixtureWitness {}

    impl ContractWitness for ExactFixtureWitness {
        fn verify_binding(&self, binding: &ExactBinding) -> Result<(), VerificationError> {
            if binding.reference().identity() == "missing.target"
                || binding.reference().version().as_str() != "1.0.0"
                || binding.fingerprint()
                    != &sha256_fingerprint(binding.reference().identity().as_bytes())
            {
                return Err(VerificationError::BindingRejected);
            }
            Ok(())
        }

        fn verify_schema_instance(
            &self,
            schema: &ExactBinding,
            instance: &ContractValue,
        ) -> Result<(), VerificationError> {
            let matches_fixture = match schema.reference().identity() {
                "details.schema" => matches!(instance.as_ref(), ContractValueRef::Null),
                "condition.schema" => {
                    matches!(instance.as_ref(), ContractValueRef::Bool(true))
                }
                _ => false,
            };
            if matches_fixture {
                Ok(())
            } else {
                Err(VerificationError::SchemaRejected)
            }
        }

        fn verify_artifact(&self, _artifact: &ArtifactRef) -> Result<(), VerificationError> {
            Ok(())
        }

        fn verify_catalog_root(&self, _root: &CatalogRoot) -> Result<(), VerificationError> {
            Err(VerificationError::WitnessUnavailable)
        }

        fn verify_catalog_page(&self, _page: &CatalogPage) -> Result<(), VerificationError> {
            Err(VerificationError::WitnessUnavailable)
        }
    }

    #[derive(Default)]
    struct AcceptingWitness {
        calls: Cell<usize>,
    }

    impl ContractWitnessSealed for AcceptingWitness {}

    impl ContractWitness for AcceptingWitness {
        fn verify_binding(&self, _binding: &ExactBinding) -> Result<(), VerificationError> {
            self.calls.set(self.calls.get() + 1);
            Ok(())
        }

        fn verify_schema_instance(
            &self,
            _schema: &ExactBinding,
            _instance: &ContractValue,
        ) -> Result<(), VerificationError> {
            self.calls.set(self.calls.get() + 1);
            Ok(())
        }

        fn verify_artifact(&self, _artifact: &ArtifactRef) -> Result<(), VerificationError> {
            self.calls.set(self.calls.get() + 1);
            Ok(())
        }

        fn verify_catalog_root(&self, _root: &CatalogRoot) -> Result<(), VerificationError> {
            self.calls.set(self.calls.get() + 1);
            Ok(())
        }

        fn verify_catalog_page(&self, _page: &CatalogPage) -> Result<(), VerificationError> {
            self.calls.set(self.calls.get() + 1);
            Ok(())
        }
    }

    fn binding(identity: &str) -> ExactBinding {
        ExactBinding::new(
            ExactRef::parse(&format!("{identity}@1.0.0")).unwrap(),
            sha256_fingerprint(identity.as_bytes()),
        )
    }

    fn bounded<const N: usize>(value: &str) -> BoundedString<N> {
        BoundedString::new(value.to_owned()).unwrap()
    }

    #[test]
    fn object_safe_witness_recurses_and_preserves_rejection_classes() {
        fn accepts_trait_object(_: &dyn ContractWitness) {}

        let limits = ContractLimits::packet0();
        let diagnostic = Diagnostic::new(
            bounded("diagnostic.ready"),
            DiagnosticSeverity::Information,
            binding("subject.item"),
            binding("details.schema"),
            ContractValue::null(),
            None,
            &limits,
        )
        .unwrap();
        let witness = FixtureWitness {
            reject_schema_identity: Some("details.schema"),
            ..FixtureWitness::default()
        };
        accepts_trait_object(&witness);
        assert_eq!(
            verify(diagnostic, &witness, &limits),
            Err(VerificationError::SchemaRejected)
        );
    }

    #[test]
    fn nested_binding_artifact_and_unavailable_catalog_fail_closed() {
        let limits = ContractLimits::packet0();
        let artifact = ArtifactRef::new(
            binding("artifact.one"),
            bounded("application/json"),
            SafeU64::new(1).unwrap(),
            Sensitivity::Internal,
            ArtifactLocator::repo_relative(sha256_fingerprint(b"repo"), bounded("proof/item"))
                .unwrap(),
        )
        .unwrap();
        let witness = FixtureWitness {
            reject_artifact_identity: Some("artifact.one"),
            ..FixtureWitness::default()
        };
        assert_eq!(
            verify(artifact, &witness, &limits),
            Err(VerificationError::ArtifactRejected)
        );

        let root = CatalogRoot::new(
            binding("catalog.root"),
            binding("entry.schema"),
            SafeU64::new(0).unwrap(),
            binding("sort.definition"),
            UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
        )
        .unwrap();
        assert_eq!(
            verify(root.clone(), &FixtureWitness::default(), &limits),
            Err(VerificationError::WitnessUnavailable)
        );

        let page = CatalogPage::new(
            CatalogRoot::new(
                root.catalog().clone(),
                root.entry_schema().clone(),
                SafeU64::new(1).unwrap(),
                root.canonical_sort().clone(),
                root.retention_until_utc().clone(),
            )
            .unwrap(),
            BoundedVec::new(vec![ContractValue::null()]).unwrap(),
            None,
            &limits,
        )
        .unwrap();
        let page_unavailable = FixtureWitness {
            catalog_root_available: true,
            catalog_page_available: false,
            ..FixtureWitness::default()
        };
        assert_eq!(
            verify(page, &page_unavailable, &limits),
            Err(VerificationError::WitnessUnavailable)
        );
    }

    #[test]
    fn stale_missing_substituted_and_transplanted_values_fail_closed() {
        let limits = ContractLimits::packet0();
        let stale = ExactBinding::new(
            ExactRef::parse("subject.item@0.9.0").unwrap(),
            sha256_fingerprint(b"subject.item"),
        );
        assert_eq!(
            verify(stale, &ExactFixtureWitness, &limits),
            Err(VerificationError::BindingRejected)
        );

        assert_eq!(
            verify(binding("missing.target"), &ExactFixtureWitness, &limits),
            Err(VerificationError::BindingRejected)
        );

        let substituted = ExactBinding::new(
            ExactRef::parse("subject.item@1.0.0").unwrap(),
            sha256_fingerprint(b"other bytes"),
        );
        assert_eq!(
            verify(substituted, &ExactFixtureWitness, &limits),
            Err(VerificationError::BindingRejected)
        );

        let transplanted = Diagnostic::new(
            bounded("diagnostic.ready"),
            DiagnosticSeverity::Information,
            binding("subject.item"),
            binding("transplanted.schema"),
            ContractValue::null(),
            None,
            &limits,
        )
        .unwrap();
        assert_eq!(
            verify(transplanted, &ExactFixtureWitness, &limits),
            Err(VerificationError::SchemaRejected)
        );

        let nested_mismatch = Problem::new(
            bounded("problem.wait"),
            bounded("prerequisite.wait"),
            ProblemCategory::Prerequisite,
            binding("subject.item"),
            None,
            binding("details.schema"),
            ContractValue::null(),
            BoundedVec::new(Vec::new()).unwrap(),
            RetryDirective::AfterRecheck,
            Some(
                RecheckCondition::new(
                    binding("condition.schema"),
                    ContractValue::boolean(false),
                    &limits,
                )
                .unwrap(),
            ),
            None,
            &limits,
        )
        .unwrap();
        assert_eq!(
            verify(nested_mismatch, &ExactFixtureWitness, &limits),
            Err(VerificationError::SchemaRejected)
        );
    }

    #[test]
    fn catalog_verification_bounds_complete_values_before_witness_calls() {
        #[derive(Serialize)]
        struct CursorPreimage<'a> {
            catalog: &'a ExactBinding,
            next_sort_key: &'a ContractValue,
        }

        #[derive(Serialize)]
        struct PagePreimage<'a> {
            root: &'a CatalogRoot,
            entries: &'a BoundedVec<ContractValue, 128>,
            next_cursor: Option<&'a CatalogCursor>,
        }

        let packet0 = ContractLimits::packet0();
        let cursor = CatalogCursor::new(
            binding("catalog.root"),
            ContractValue::string(bounded("next")),
            &packet0,
        )
        .unwrap();
        let cursor_preimage = CursorPreimage {
            catalog: cursor.catalog(),
            next_sort_key: cursor.next_sort_key(),
        };
        let cursor_preimage_bytes = canonical_json_bytes(&cursor_preimage, &packet0).unwrap();
        let cursor_limits = ContractLimits::new(
            cursor_preimage_bytes.len(),
            packet0.string_bytes(),
            packet0.array_items(),
            packet0.object_members(),
            packet0.nesting_depth(),
        )
        .unwrap();
        assert!(
            cursor_fingerprint(cursor.catalog(), cursor.next_sort_key(), &cursor_limits).is_ok()
        );
        let cursor_witness = AcceptingWitness::default();
        assert_eq!(
            verify(cursor, &cursor_witness, &cursor_limits),
            Err(VerificationError::ContractInvariant)
        );
        assert_eq!(cursor_witness.calls.get(), 0);

        let root = CatalogRoot::new(
            binding("catalog.root"),
            binding("entry.schema"),
            SafeU64::new(1).unwrap(),
            binding("sort.definition"),
            UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
        )
        .unwrap();
        let page = CatalogPage::new(
            root,
            BoundedVec::new(vec![ContractValue::null()]).unwrap(),
            None,
            &packet0,
        )
        .unwrap();
        let page_preimage = PagePreimage {
            root: page.root(),
            entries: page.entries(),
            next_cursor: page.next_cursor(),
        };
        let page_preimage_bytes = canonical_json_bytes(&page_preimage, &packet0).unwrap();
        let page_limits = ContractLimits::new(
            page_preimage_bytes.len(),
            packet0.string_bytes(),
            packet0.array_items(),
            packet0.object_members(),
            packet0.nesting_depth(),
        )
        .unwrap();
        assert!(page_fingerprint(
            page.root(),
            page.entries(),
            page.next_cursor(),
            &page_limits
        )
        .is_ok());
        let page_witness = AcceptingWitness::default();
        assert_eq!(
            verify(page, &page_witness, &page_limits),
            Err(VerificationError::ContractInvariant)
        );
        assert_eq!(page_witness.calls.get(), 0);
    }
}
