use handbook_engine::{ArtifactPresence, CanonicalArtifactIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetDisposition {
    Keep,
    Summarize,
    Exclude,
    Refuse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetReason {
    WithinBudget,
    OptionalArtifactTooLarge,
    TotalBytesExceeded,
    RequiredArtifactTooLarge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextSafeAction {
    ReduceCanonicalArtifactSize {
        canonical_repo_relative_path: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetByteDomain {
    Source,
    RenderedOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetTarget {
    pub canonical_repo_relative_path: String,
    pub byte_len: u64,
    pub byte_domain: BudgetByteDomain,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetEffectiveBytes {
    pub canonical_repo_relative_path: String,
    pub byte_len: u64,
    pub byte_domain: BudgetByteDomain,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetOutcome {
    pub disposition: BudgetDisposition,
    pub reason: BudgetReason,
    pub targets: Vec<BudgetTarget>,
    pub next_safe_action: Option<NextSafeAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BudgetPolicy {
    pub max_total_bytes: Option<u64>,
    pub max_per_artifact_bytes: Option<u64>,
}

pub fn evaluate_budget(
    artifacts: &[CanonicalArtifactIdentity],
    policy: BudgetPolicy,
) -> BudgetOutcome {
    evaluate_budget_with_effective_bytes(artifacts, &[], policy)
}

pub fn evaluate_budget_with_effective_bytes(
    artifacts: &[CanonicalArtifactIdentity],
    effective_bytes: &[BudgetEffectiveBytes],
    policy: BudgetPolicy,
) -> BudgetOutcome {
    let present: Vec<(&CanonicalArtifactIdentity, u64, BudgetByteDomain)> = artifacts
        .iter()
        .filter_map(|artifact| match artifact.presence {
            ArtifactPresence::Missing => None,
            ArtifactPresence::PresentEmpty | ArtifactPresence::PresentNonEmpty => {
                let effective = effective_bytes.iter().find(|effective| {
                    effective.canonical_repo_relative_path == artifact.relative_path
                });
                effective
                    .map(|effective| (artifact, effective.byte_len, effective.byte_domain))
                    .or_else(|| {
                        artifact
                            .byte_len
                            .map(|len| (artifact, len, BudgetByteDomain::Source))
                    })
            }
        })
        .collect();

    if let Some(max_per) = policy.max_per_artifact_bytes {
        for (artifact, len, byte_domain) in &present {
            if *len <= max_per {
                continue;
            }

            if artifact.packet_required {
                return BudgetOutcome {
                    disposition: BudgetDisposition::Refuse,
                    reason: BudgetReason::RequiredArtifactTooLarge,
                    targets: vec![BudgetTarget {
                        canonical_repo_relative_path: artifact.relative_path.clone(),
                        byte_len: *len,
                        byte_domain: *byte_domain,
                    }],
                    next_safe_action: Some(NextSafeAction::ReduceCanonicalArtifactSize {
                        canonical_repo_relative_path: artifact.relative_path.clone(),
                    }),
                };
            }

            return BudgetOutcome {
                disposition: BudgetDisposition::Summarize,
                reason: BudgetReason::OptionalArtifactTooLarge,
                targets: vec![BudgetTarget {
                    canonical_repo_relative_path: artifact.relative_path.clone(),
                    byte_len: *len,
                    byte_domain: *byte_domain,
                }],
                next_safe_action: None,
            };
        }
    }

    if let Some(max_total) = policy.max_total_bytes {
        let total: u64 = present.iter().map(|(_, len, _)| *len).sum();
        if total > max_total {
            let mut remaining_total = total;
            let mut exclusions = Vec::new();
            for (artifact, len, byte_domain) in &present {
                if artifact.packet_required {
                    continue;
                }
                exclusions.push(BudgetTarget {
                    canonical_repo_relative_path: artifact.relative_path.clone(),
                    byte_len: *len,
                    byte_domain: *byte_domain,
                });
                remaining_total = remaining_total.saturating_sub(*len);
                if remaining_total <= max_total {
                    return BudgetOutcome {
                        disposition: BudgetDisposition::Exclude,
                        reason: BudgetReason::TotalBytesExceeded,
                        targets: exclusions,
                        next_safe_action: None,
                    };
                }
            }

            let mut largest_required: Option<(&CanonicalArtifactIdentity, u64, BudgetByteDomain)> =
                None;
            for (artifact, len, byte_domain) in &present {
                if !artifact.packet_required {
                    continue;
                }
                match largest_required {
                    None => largest_required = Some((artifact, *len, *byte_domain)),
                    Some((_, best_len, _)) if *len > best_len => {
                        largest_required = Some((artifact, *len, *byte_domain))
                    }
                    _ => {}
                }
            }

            if let Some((artifact, len, byte_domain)) = largest_required {
                return BudgetOutcome {
                    disposition: BudgetDisposition::Refuse,
                    reason: BudgetReason::TotalBytesExceeded,
                    targets: vec![BudgetTarget {
                        canonical_repo_relative_path: artifact.relative_path.clone(),
                        byte_len: len,
                        byte_domain,
                    }],
                    next_safe_action: Some(NextSafeAction::ReduceCanonicalArtifactSize {
                        canonical_repo_relative_path: artifact.relative_path.clone(),
                    }),
                };
            }
        }
    }

    BudgetOutcome {
        disposition: BudgetDisposition::Keep,
        reason: BudgetReason::WithinBudget,
        targets: Vec::new(),
        next_safe_action: None,
    }
}
