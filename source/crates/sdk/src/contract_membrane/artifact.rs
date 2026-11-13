use super::{BoundedString, ContractError, ExactBinding, Fingerprint, SafeU64};
use serde::de::Error as _;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

macro_rules! closed_enum {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            $($variant),+
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[serde(rename_all = "snake_case")]
                enum Raw {
                    $($variant),+
                }

                let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
                Ok(match raw {
                    $(Raw::$variant => Self::$variant),+
                })
            }
        }
    };
}

closed_enum!(DigestAlgorithm { Sha256 });

closed_enum!(Sensitivity {
    Public,
    Internal,
    Restricted,
});

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactLocator {
    kind: ArtifactLocatorKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ArtifactLocatorKind {
    RepoRelative {
        repository_identity_fingerprint: Fingerprint,
        path: BoundedString<4096>,
    },
    ContentAddressed {
        store: ExactBinding,
        digest: Fingerprint,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactLocatorRef<'a> {
    RepoRelative {
        repository_identity_fingerprint: &'a Fingerprint,
        path: &'a BoundedString<4096>,
        no_follow: bool,
    },
    ContentAddressed {
        store: &'a ExactBinding,
        algorithm: DigestAlgorithm,
        digest: &'a Fingerprint,
    },
}

impl ArtifactLocator {
    pub fn repo_relative(
        repository_identity_fingerprint: Fingerprint,
        path: BoundedString<4096>,
    ) -> Result<Self, ContractError> {
        validate_portable_path(path.as_str())?;
        Ok(Self {
            kind: ArtifactLocatorKind::RepoRelative {
                repository_identity_fingerprint,
                path,
            },
        })
    }

    pub fn content_addressed(store: ExactBinding, digest: Fingerprint) -> Self {
        Self {
            kind: ArtifactLocatorKind::ContentAddressed { store, digest },
        }
    }

    pub fn as_ref(&self) -> ArtifactLocatorRef<'_> {
        match &self.kind {
            ArtifactLocatorKind::RepoRelative {
                repository_identity_fingerprint,
                path,
            } => ArtifactLocatorRef::RepoRelative {
                repository_identity_fingerprint,
                path,
                no_follow: true,
            },
            ArtifactLocatorKind::ContentAddressed { store, digest } => {
                ArtifactLocatorRef::ContentAddressed {
                    store,
                    algorithm: DigestAlgorithm::Sha256,
                    digest,
                }
            }
        }
    }
}

impl Serialize for ArtifactLocator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.kind {
            ArtifactLocatorKind::RepoRelative {
                repository_identity_fingerprint,
                path,
            } => {
                let mut object = serializer.serialize_struct("ArtifactLocator", 4)?;
                object.serialize_field("kind", "repo_relative")?;
                object.serialize_field(
                    "repository_identity_fingerprint",
                    repository_identity_fingerprint,
                )?;
                object.serialize_field("path", path)?;
                object.serialize_field("no_follow", &true)?;
                object.end()
            }
            ArtifactLocatorKind::ContentAddressed { store, digest } => {
                let mut object = serializer.serialize_struct("ArtifactLocator", 4)?;
                object.serialize_field("kind", "content_addressed")?;
                object.serialize_field("store", store)?;
                object.serialize_field("algorithm", &DigestAlgorithm::Sha256)?;
                object.serialize_field("digest", digest)?;
                object.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for ArtifactLocator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
        enum RawArtifactLocator {
            RepoRelative {
                repository_identity_fingerprint: Fingerprint,
                path: BoundedString<4096>,
                no_follow: bool,
            },
            ContentAddressed {
                store: ExactBinding,
                algorithm: BoundedString<255>,
                digest: Fingerprint,
            },
        }

        let raw =
            RawArtifactLocator::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        match raw {
            RawArtifactLocator::RepoRelative {
                repository_identity_fingerprint,
                path,
                no_follow,
            } => {
                if !no_follow {
                    return Err(D::Error::custom(ContractError::InvalidArtifactLocator));
                }
                Self::repo_relative(repository_identity_fingerprint, path).map_err(D::Error::custom)
            }
            RawArtifactLocator::ContentAddressed {
                store,
                algorithm,
                digest,
            } => {
                if algorithm.as_str() != "sha256" {
                    return Err(D::Error::custom(ContractError::InvalidArtifactLocator));
                }
                Ok(Self::content_addressed(store, digest))
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ArtifactRef {
    artifact: ExactBinding,
    media_type: BoundedString<255>,
    byte_length: SafeU64,
    sensitivity: Sensitivity,
    locator: ArtifactLocator,
}

impl ArtifactRef {
    pub fn new(
        artifact: ExactBinding,
        media_type: BoundedString<255>,
        byte_length: SafeU64,
        sensitivity: Sensitivity,
        locator: ArtifactLocator,
    ) -> Result<Self, ContractError> {
        if !valid_media_type(media_type.as_str()) {
            return Err(ContractError::InvalidString);
        }
        if let ArtifactLocatorRef::ContentAddressed { digest, .. } = locator.as_ref() {
            if digest != artifact.fingerprint() {
                return Err(ContractError::FingerprintMismatch);
            }
        }
        Ok(Self {
            artifact,
            media_type,
            byte_length,
            sensitivity,
            locator,
        })
    }

    pub fn artifact(&self) -> &ExactBinding {
        &self.artifact
    }

    pub fn media_type(&self) -> &BoundedString<255> {
        &self.media_type
    }

    pub fn byte_length(&self) -> SafeU64 {
        self.byte_length
    }

    pub fn sensitivity(&self) -> Sensitivity {
        self.sensitivity
    }

    pub fn locator(&self) -> &ArtifactLocator {
        &self.locator
    }
}

impl<'de> Deserialize<'de> for ArtifactRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RawArtifactRef {
            artifact: ExactBinding,
            media_type: BoundedString<255>,
            byte_length: SafeU64,
            sensitivity: Sensitivity,
            locator: ArtifactLocator,
        }

        let raw = RawArtifactRef::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::new(
            raw.artifact,
            raw.media_type,
            raw.byte_length,
            raw.sensitivity,
            raw.locator,
        )
        .map_err(D::Error::custom)
    }
}

fn validate_portable_path(path: &str) -> Result<(), ContractError> {
    if path.starts_with('/')
        || path.ends_with('/')
        || !path
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'/' | b'-'))
    {
        return Err(ContractError::InvalidArtifactLocator);
    }

    for segment in path.split('/') {
        if segment.is_empty()
            || matches!(segment, "." | "..")
            || segment.ends_with('.')
            || is_reserved_device_name(segment)
        {
            return Err(ContractError::InvalidArtifactLocator);
        }
    }
    Ok(())
}

fn is_reserved_device_name(segment: &str) -> bool {
    let basename = segment.split('.').next().unwrap_or_default();
    basename.eq_ignore_ascii_case("con")
        || basename.eq_ignore_ascii_case("prn")
        || basename.eq_ignore_ascii_case("aux")
        || basename.eq_ignore_ascii_case("nul")
        || (basename.len() == 4
            && (basename[..3].eq_ignore_ascii_case("com")
                || basename[..3].eq_ignore_ascii_case("lpt"))
            && matches!(basename.as_bytes()[3], b'1'..=b'9'))
}

fn valid_media_type(value: &str) -> bool {
    let Some((media_type, subtype)) = value.split_once('/') else {
        return false;
    };
    !subtype.contains('/') && valid_media_component(media_type) && valid_media_component(subtype)
}

fn valid_media_component(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes
        .first()
        .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        && bytes[1..].iter().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(
                    byte,
                    b'!' | b'#' | b'$' | b'&' | b'^' | b'_' | b'.' | b'+' | b'-'
                )
        })
}

fn sanitize_deserialize_error<E: serde::de::Error>(error: E) -> E {
    E::custom(super::serialization::classify_contract_error_message(
        &error.to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::DeserializeOwned;
    use std::fmt;

    const SENTINEL: &str = "raw-key-sentinel-do-not-echo";

    #[test]
    fn public_artifact_deserializers_redact_unknown_input() {
        assert_redacted::<DigestAlgorithm>(&format!(r#""{SENTINEL}""#));
        assert_redacted::<Sensitivity>(&format!(r#""{SENTINEL}""#));
        assert_redacted::<ArtifactLocator>(&format!(r#"{{"kind":"{SENTINEL}"}}"#));
        assert_redacted::<ArtifactRef>(&format!(r#"{{"{SENTINEL}":true}}"#));
        assert_redacted::<ArtifactLocator>(&format!(r#""{SENTINEL}""#));
        assert_redacted::<ArtifactRef>(&format!(r#""{SENTINEL}""#));
    }

    fn assert_redacted<T>(json: &str)
    where
        T: DeserializeOwned + fmt::Debug,
    {
        let error = serde_json::from_str::<T>(json).unwrap_err().to_string();
        assert!(
            !error.contains(SENTINEL),
            "public error echoed input: {error}"
        );
    }
}
