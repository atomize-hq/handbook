use handbook_engine::artifact_manifest::ManifestError;

pub enum CompilerError {
    Manifest(ManifestError),
}

impl std::fmt::Debug for CompilerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Manifest(error) => formatter.debug_tuple("Manifest").field(error).finish(),
        }
    }
}
