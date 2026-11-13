use crate::pipeline::{
    ActivationClause, ActivationConditionSet, ActivationOperator, PipelineDefinition,
    PipelineStage, StageActivation,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RouteVariables {
    values: BTreeMap<String, bool>,
}

impl RouteVariables {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new(values: BTreeMap<String, bool>) -> Result<Self, RouteEvaluationError> {
        validate_route_variable_map(&values)?;
        Ok(Self { values })
    }

    pub fn from_pairs<I, S>(pairs: I) -> Result<Self, RouteEvaluationError>
    where
        I: IntoIterator<Item = (S, bool)>,
        S: Into<String>,
    {
        let mut values = BTreeMap::new();
        for (name, value) in pairs {
            values.insert(name.into(), value);
        }

        Self::new(values)
    }

    pub fn get(&self, variable: &str) -> Option<bool> {
        self.values.get(variable).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, bool)> + '_ {
        self.values
            .iter()
            .map(|(name, value)| (name.as_str(), *value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPipelineRoute {
    pub pipeline_id: String,
    pub stages: Vec<ResolvedPipelineStage>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPipelineStage {
    pub stage_id: String,
    pub status: RouteStageStatus,
    pub reason: Option<RouteStageReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteStageStatus {
    Active,
    Skipped,
    Blocked,
    Next,
}

impl RouteStageStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            RouteStageStatus::Active => "active",
            RouteStageStatus::Skipped => "skipped",
            RouteStageStatus::Blocked => "blocked",
            RouteStageStatus::Next => "next",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteStageReason {
    SkippedActivationFalse {
        operator: ActivationOperator,
        unsatisfied_variables: Vec<String>,
    },
    NextMissingRouteVariables {
        operator: ActivationOperator,
        missing_variables: Vec<String>,
    },
    BlockedByUnresolvedStage {
        upstream_stage_id: String,
        upstream_status: RouteStageStatus,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteEvaluationError {
    InvalidRouteVariableName {
        variable: String,
    },
    EmptyActivationClauses {
        stage_id: String,
        operator: ActivationOperator,
    },
    InvalidActivationClause {
        stage_id: String,
        variable: String,
        reason: &'static str,
    },
}

pub fn resolve_pipeline_route(
    definition: &PipelineDefinition,
    route_variables: &RouteVariables,
) -> Result<ResolvedPipelineRoute, RouteEvaluationError> {
    validate_route_variable_map(&route_variables.values)?;

    let mut stages = Vec::with_capacity(definition.declared_stages().len());
    let mut unresolved_upstream: Option<(String, RouteStageStatus)> = None;

    for stage in definition.declared_stages() {
        validate_stage_activation(stage)?;

        let decision =
            if let Some((upstream_stage_id, upstream_status)) = unresolved_upstream.as_ref() {
                ResolvedPipelineStage {
                    stage_id: stage.id.clone(),
                    status: RouteStageStatus::Blocked,
                    reason: Some(RouteStageReason::BlockedByUnresolvedStage {
                        upstream_stage_id: upstream_stage_id.clone(),
                        upstream_status: *upstream_status,
                    }),
                }
            } else {
                evaluate_stage(stage, route_variables)?
            };

        if decision.status == RouteStageStatus::Next {
            unresolved_upstream = Some((decision.stage_id.clone(), decision.status));
        }

        stages.push(decision);
    }

    Ok(ResolvedPipelineRoute {
        pipeline_id: definition.header.id.clone(),
        stages,
    })
}

fn evaluate_stage(
    stage: &PipelineStage,
    route_variables: &RouteVariables,
) -> Result<ResolvedPipelineStage, RouteEvaluationError> {
    let Some(activation) = stage.activation.as_ref() else {
        return Ok(ResolvedPipelineStage {
            stage_id: stage.id.clone(),
            status: RouteStageStatus::Active,
            reason: None,
        });
    };

    evaluate_activation(stage, activation, route_variables)
}

fn evaluate_activation(
    stage: &PipelineStage,
    activation: &StageActivation,
    route_variables: &RouteVariables,
) -> Result<ResolvedPipelineStage, RouteEvaluationError> {
    let evaluation = evaluate_condition_set(stage, &activation.when, route_variables)?;

    match evaluation {
        ConditionEvaluation::Satisfied => Ok(ResolvedPipelineStage {
            stage_id: stage.id.clone(),
            status: RouteStageStatus::Active,
            reason: None,
        }),
        ConditionEvaluation::False {
            unsatisfied_variables,
        } => Ok(ResolvedPipelineStage {
            stage_id: stage.id.clone(),
            status: RouteStageStatus::Skipped,
            reason: Some(RouteStageReason::SkippedActivationFalse {
                operator: activation.when.operator,
                unsatisfied_variables,
            }),
        }),
        ConditionEvaluation::Missing { missing_variables } => Ok(ResolvedPipelineStage {
            stage_id: stage.id.clone(),
            status: RouteStageStatus::Next,
            reason: Some(RouteStageReason::NextMissingRouteVariables {
                operator: activation.when.operator,
                missing_variables,
            }),
        }),
    }
}

enum ConditionEvaluation {
    Satisfied,
    False { unsatisfied_variables: Vec<String> },
    Missing { missing_variables: Vec<String> },
}

fn evaluate_condition_set(
    stage: &PipelineStage,
    condition_set: &ActivationConditionSet,
    route_variables: &RouteVariables,
) -> Result<ConditionEvaluation, RouteEvaluationError> {
    if condition_set.clauses.is_empty() {
        return Err(RouteEvaluationError::EmptyActivationClauses {
            stage_id: stage.id.clone(),
            operator: condition_set.operator,
        });
    }

    let mut unsatisfied_variables = BTreeSet::new();
    let mut missing_variables = BTreeSet::new();
    let mut matched = false;

    for clause in &condition_set.clauses {
        validate_clause(stage, clause)?;

        match route_variables.get(&clause.variable) {
            Some(actual) if actual == clause.value => {
                matched = true;
            }
            Some(_) => {
                unsatisfied_variables.insert(clause.variable.clone());
            }
            None => {
                missing_variables.insert(clause.variable.clone());
            }
        }
    }

    match condition_set.operator {
        ActivationOperator::Any if matched => Ok(ConditionEvaluation::Satisfied),
        ActivationOperator::Any if !missing_variables.is_empty() => {
            Ok(ConditionEvaluation::Missing {
                missing_variables: missing_variables.into_iter().collect(),
            })
        }
        ActivationOperator::Any => Ok(ConditionEvaluation::False {
            unsatisfied_variables: unsatisfied_variables.into_iter().collect(),
        }),
        ActivationOperator::All if !unsatisfied_variables.is_empty() => {
            Ok(ConditionEvaluation::False {
                unsatisfied_variables: unsatisfied_variables.into_iter().collect(),
            })
        }
        ActivationOperator::All if !missing_variables.is_empty() => {
            Ok(ConditionEvaluation::Missing {
                missing_variables: missing_variables.into_iter().collect(),
            })
        }
        ActivationOperator::All => Ok(ConditionEvaluation::Satisfied),
    }
}

fn validate_stage_activation(stage: &PipelineStage) -> Result<(), RouteEvaluationError> {
    let Some(activation) = stage.activation.as_ref() else {
        return Ok(());
    };

    if activation.when.clauses.is_empty() {
        return Err(RouteEvaluationError::EmptyActivationClauses {
            stage_id: stage.id.clone(),
            operator: activation.when.operator,
        });
    }

    for clause in &activation.when.clauses {
        validate_clause(stage, clause)?;
    }

    Ok(())
}

fn validate_clause(
    stage: &PipelineStage,
    clause: &ActivationClause,
) -> Result<(), RouteEvaluationError> {
    if !is_supported_variable_name(&clause.variable) {
        return Err(RouteEvaluationError::InvalidActivationClause {
            stage_id: stage.id.clone(),
            variable: clause.variable.clone(),
            reason: "variable name must start with an ASCII letter or `_` and continue with ASCII alphanumeric or `_` characters",
        });
    }

    Ok(())
}

fn validate_route_variable_map(
    values: &BTreeMap<String, bool>,
) -> Result<(), RouteEvaluationError> {
    for variable in values.keys() {
        if !is_supported_variable_name(variable) {
            return Err(RouteEvaluationError::InvalidRouteVariableName {
                variable: variable.clone(),
            });
        }
    }

    Ok(())
}

fn is_supported_variable_name(variable: &str) -> bool {
    let mut chars = variable.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }

    chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

impl fmt::Display for RouteEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteEvaluationError::InvalidRouteVariableName { variable } => {
                write!(f, "route variable `{variable}` is out of contract")
            }
            RouteEvaluationError::EmptyActivationClauses { stage_id, operator } => {
                write!(
                    f,
                    "stage `{stage_id}` activation `when.{}` must contain at least one clause",
                    activation_operator_label(*operator)
                )
            }
            RouteEvaluationError::InvalidActivationClause {
                stage_id,
                variable,
                reason,
            } => {
                write!(
                    f,
                    "stage `{stage_id}` has an out-of-contract activation clause for `{variable}`: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for RouteEvaluationError {}

fn activation_operator_label(operator: ActivationOperator) -> &'static str {
    match operator {
        ActivationOperator::Any => "any",
        ActivationOperator::All => "all",
    }
}
