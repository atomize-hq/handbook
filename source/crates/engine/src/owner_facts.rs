#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnerCausalDispositionV1 {
    Prerequisite,
    CallerRefusal,
    ImplementationFailure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnerPublicEffectDispositionV1 {
    CreatedIncomplete,
    InstalledDurabilityUnconfirmed,
    DurableCreated,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnerPublicEffectV1 {
    target_ref: String,
    content_fingerprint: Option<String>,
    disposition: OwnerPublicEffectDispositionV1,
}

impl OwnerPublicEffectV1 {
    pub(crate) fn created_incomplete(target_ref: impl Into<String>) -> Self {
        Self {
            target_ref: target_ref.into(),
            content_fingerprint: None,
            disposition: OwnerPublicEffectDispositionV1::CreatedIncomplete,
        }
    }

    pub(crate) fn installed_unconfirmed(
        target_ref: impl Into<String>,
        content_fingerprint: impl Into<String>,
    ) -> Self {
        Self {
            target_ref: target_ref.into(),
            content_fingerprint: Some(content_fingerprint.into()),
            disposition: OwnerPublicEffectDispositionV1::InstalledDurabilityUnconfirmed,
        }
    }

    pub fn durable_created(
        target_ref: impl Into<String>,
        content_fingerprint: impl Into<String>,
    ) -> Self {
        Self {
            target_ref: target_ref.into(),
            content_fingerprint: Some(content_fingerprint.into()),
            disposition: OwnerPublicEffectDispositionV1::DurableCreated,
        }
    }

    pub fn target_ref(&self) -> &str {
        &self.target_ref
    }

    pub fn content_fingerprint(&self) -> Option<&str> {
        self.content_fingerprint.as_deref()
    }

    pub fn disposition(&self) -> OwnerPublicEffectDispositionV1 {
        self.disposition
    }
}
