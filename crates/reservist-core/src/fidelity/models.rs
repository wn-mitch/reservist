use crate::cognition::{BeliefLedger, BoundedEstimate};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FidelityTier {
    NamedCognition,
    LimitedRoleHolder,
    ParticipantDistribution,
    OrganizationCohortResponse,
    PopDistributedResponse,
    AttributedModel,
    MechanicalOrAdapter,
}

impl FidelityTier {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NamedCognition => "NAMED_COGNITION",
            Self::LimitedRoleHolder => "LIMITED_ROLE_HOLDER",
            Self::ParticipantDistribution => "PARTICIPANT_DISTRIBUTION",
            Self::OrganizationCohortResponse => "ORGANIZATION_COHORT_RESPONSE",
            Self::PopDistributedResponse => "POP_DISTRIBUTED_RESPONSE",
            Self::AttributedModel => "ATTRIBUTED_MODEL",
            Self::MechanicalOrAdapter => "MECHANICAL_OR_ADAPTER",
        }
    }
}

mod sealed {
    pub trait Sealed {}
}

trait FidelityModel: sealed::Sealed {
    const TIER: FidelityTier;
    const CLADES: &'static [&'static str];
    const REQUIRED_STATE: &'static str;
    const CAPABILITIES: &'static [&'static str];

    fn validate(owner: &str, states: OpeningStates<'_>) -> Result<(), String>;
}

#[derive(Clone, Copy)]
pub(crate) struct ModelDeclaration {
    pub tier: FidelityTier,
    pub clades: &'static [&'static str],
    pub required_state: &'static str,
    pub capabilities: &'static [&'static str],
    pub validate: fn(&str, OpeningStates<'_>) -> Result<(), String>,
}

const fn declaration<T: FidelityModel>() -> ModelDeclaration {
    ModelDeclaration {
        tier: T::TIER,
        clades: T::CLADES,
        required_state: T::REQUIRED_STATE,
        capabilities: T::CAPABILITIES,
        validate: T::validate,
    }
}

/// A borrowed, owner-scoped opening-state bundle. Each model parses this into its own typed view.
#[derive(Clone, Copy)]
pub(crate) struct OpeningStates<'a> {
    owner: &'a str,
    states: &'a [&'a Value],
}

impl<'a> OpeningStates<'a> {
    pub(crate) fn new(owner: &'a str, states: &'a [&'a Value]) -> Self {
        Self { owner, states }
    }

    fn value_for(&self, suffix: &str) -> Result<&'a Value, String> {
        self.states
            .iter()
            .find(|state| {
                state
                    .get("state_id")
                    .and_then(Value::as_str)
                    .is_some_and(|id| id.ends_with(suffix))
            })
            .and_then(|state| state.get("value"))
            .ok_or_else(|| {
                format!(
                    "fidelity_state: {} lacks required {suffix} opening state",
                    self.owner
                )
            })
    }

    fn object_for(&self, suffix: &str) -> Result<&'a Map<String, Value>, String> {
        self.value_for(suffix)?.as_object().ok_or_else(|| {
            format!(
                "fidelity_state: {} requires {suffix} to contain an object value",
                self.owner
            )
        })
    }
}

fn array<'a>(
    owner: &str,
    state: &str,
    object: &'a Map<String, Value>,
    field: &str,
) -> Result<&'a Vec<Value>, String> {
    object
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("fidelity_state: {owner} requires {state}.{field} as an array"))
}

struct NamedCognition {
    beliefs: Vec<BoundedEstimate>,
}

impl NamedCognition {
    fn parse(owner: &str, states: OpeningStates<'_>) -> Result<Self, String> {
        let cognition = states.object_for(".cognition")?;
        let beliefs = array(owner, "cognition", cognition, "beliefs")?
            .iter()
            .map(BoundedEstimate::from_value)
            .collect::<Result<_, _>>()
            .map_err(|error| {
                format!("fidelity_state: {owner} has malformed cognition belief: {error}")
            })?;
        let _ = array(owner, "cognition", cognition, "goals")?;
        let _ = array(owner, "cognition", cognition, "memory")?;
        let _ = array(owner, "cognition", cognition, "plans")?;
        Ok(Self { beliefs })
    }
}
impl sealed::Sealed for NamedCognition {}
impl FidelityModel for NamedCognition {
    const TIER: FidelityTier = FidelityTier::NamedCognition;
    const CLADES: &'static [&'static str] = &["Person"];
    const REQUIRED_STATE: &'static str = "cognition(beliefs,goals,memory,plans)";
    const CAPABILITIES: &'static [&'static str] =
        &["attributable_choice", "private_cognition", "reconsiders"];

    fn validate(owner: &str, states: OpeningStates<'_>) -> Result<(), String> {
        let model = Self::parse(owner, states)?;
        BeliefLedger::new(model.beliefs).map_err(|error| {
            format!("fidelity_state: {owner} has invalid cognition beliefs: {error}")
        })?;
        Ok(())
    }
}

struct LimitedRoleHolder<'a> {
    role_state: &'a Map<String, Value>,
}
impl<'a> LimitedRoleHolder<'a> {
    fn parse(states: OpeningStates<'a>) -> Result<Self, String> {
        let role_state = states
            .states
            .first()
            .and_then(|state| state.get("value"))
            .and_then(Value::as_object)
            .ok_or_else(|| format!("fidelity_state: {} requires typed role state", states.owner))?;
        Ok(Self { role_state })
    }
    fn validate(&self, owner: &str) -> Result<(), String> {
        let tenure = ["effective_period", "status"];
        let individual_tenure = tenure.iter().all(|key| self.role_state.contains_key(*key))
            && (self
                .role_state
                .get("holder_id")
                .is_some_and(Value::is_string)
                || self
                    .role_state
                    .get("holder_ids")
                    .is_some_and(Value::is_array));
        if individual_tenure
            || (self.role_state.get("beliefs").is_some_and(Value::is_object)
                && self
                    .role_state
                    .get("private")
                    .is_some_and(Value::is_boolean)
                && self.role_state.get("source").is_some_and(Value::is_string))
            || (self.role_state.get("history").is_some_and(Value::is_array)
                && self.role_state.get("storage").is_some_and(Value::is_string))
        {
            Ok(())
        } else {
            Err(format!(
                "fidelity_state: {owner} requires tenure ({}), private role beliefs with a source, or publication history with storage",
                tenure.join(",")
            ))
        }
    }
}
impl sealed::Sealed for LimitedRoleHolder<'_> {}
impl FidelityModel for LimitedRoleHolder<'_> {
    const TIER: FidelityTier = FidelityTier::LimitedRoleHolder;
    const CLADES: &'static [&'static str] = &["Office", "Outlet", "Person"];
    const REQUIRED_STATE: &'static str = "office tenure, role cognition, or publication role state";
    const CAPABILITIES: &'static [&'static str] = &[
        "attributable_choice",
        "bounded_reconsideration",
        "role_action",
    ];
    fn validate(owner: &str, states: OpeningStates<'_>) -> Result<(), String> {
        LimitedRoleHolder::parse(states)?.validate(owner)
    }
}

struct ParticipantDistribution<'a> {
    states: &'a [&'a Value],
}
impl<'a> ParticipantDistribution<'a> {
    fn parse(states: OpeningStates<'a>) -> Result<Self, String> {
        Ok(Self {
            states: states.states,
        })
    }
    fn validate(&self, owner: &str) -> Result<(), String> {
        let has = |suffix: &str| {
            self.states.iter().any(|state| {
                state
                    .get("state_id")
                    .and_then(Value::as_str)
                    .is_some_and(|id| id.ends_with(suffix))
                    && state.get("value").is_some_and(Value::is_object)
            })
        };
        if has(".procedure")
            || has(".governance")
            || has(".desk_authority")
            || has(".work")
            || has(".evidence")
            || has(".mass")
            || has(".allocations")
        {
            Ok(())
        } else {
            Err(format!(
                "fidelity_state: {owner} requires a typed procedure, governance, staff, or population participant state"
            ))
        }
    }
}
impl sealed::Sealed for ParticipantDistribution<'_> {}
impl FidelityModel for ParticipantDistribution<'_> {
    const TIER: FidelityTier = FidelityTier::ParticipantDistribution;
    const CLADES: &'static [&'static str] = &[
        "DecisionBody",
        "HouseholdCohort",
        "Institution",
        "PersonPopulationCell",
        "StaffUnit",
    ];
    const REQUIRED_STATE: &'static str =
        "procedure, governance, staff, or population participant state";
    const CAPABILITIES: &'static [&'static str] = &[
        "distributed_deliberation",
        "institutional_execution",
        "procedural_decision",
    ];
    fn validate(owner: &str, states: OpeningStates<'_>) -> Result<(), String> {
        ParticipantDistribution::parse(states)?.validate(owner)
    }
}

struct OrganizationCohortResponse<'a> {
    states: &'a [&'a Value],
}
impl<'a> OrganizationCohortResponse<'a> {
    fn parse(states: OpeningStates<'a>) -> Result<Self, String> {
        Ok(Self {
            states: states.states,
        })
    }
    fn validate(&self, owner: &str) -> Result<(), String> {
        let has = |suffix: &str| {
            self.states.iter().any(|state| {
                state
                    .get("state_id")
                    .and_then(Value::as_str)
                    .is_some_and(|id| id.ends_with(suffix))
                    && state.get("value").is_some_and(Value::is_object)
            })
        };
        if has(".capacity") || has(".behavior") {
            Ok(())
        } else {
            Err(format!(
                "fidelity_state: {owner} requires cohort capacity or behavior state"
            ))
        }
    }
}
impl sealed::Sealed for OrganizationCohortResponse<'_> {}
impl FidelityModel for OrganizationCohortResponse<'_> {
    const TIER: FidelityTier = FidelityTier::OrganizationCohortResponse;
    const CLADES: &'static [&'static str] = &[
        "IndustryCohort",
        "Institution",
        "NamedFirm",
        "OrganizationCohort",
    ];
    const REQUIRED_STATE: &'static str = "cohort capacity or behavior state";
    const CAPABILITIES: &'static [&'static str] =
        &["bounded_response", "settlement", "strategy_distribution"];
    fn validate(owner: &str, states: OpeningStates<'_>) -> Result<(), String> {
        OrganizationCohortResponse::parse(states)?.validate(owner)
    }
}

struct PopDistributedResponse<'a> {
    population: &'a Map<String, Value>,
    exposures: &'a Map<String, Value>,
    flows: &'a Map<String, Value>,
}
impl<'a> PopDistributedResponse<'a> {
    fn parse(states: OpeningStates<'a>) -> Result<Self, String> {
        Ok(Self {
            population: states.object_for(".conserved_population")?,
            exposures: states.object_for(".exposure_distribution")?,
            flows: states.object_for(".realized_flows")?,
        })
    }
    fn validate(&self, owner: &str) -> Result<(), String> {
        if self.population.is_empty() || self.exposures.is_empty() || self.flows.is_empty() {
            return Err(format!(
                "fidelity_state: {owner} requires conserved population, exposure distribution, and realized flows"
            ));
        }
        Ok(())
    }
}
impl sealed::Sealed for PopDistributedResponse<'_> {}
impl FidelityModel for PopDistributedResponse<'_> {
    const TIER: FidelityTier = FidelityTier::PopDistributedResponse;
    const CLADES: &'static [&'static str] = &["HouseholdCohort", "PersonPopulationCell", "PopLens"];
    const REQUIRED_STATE: &'static str =
        "conserved_population, exposure_distribution, realized_flows";
    const CAPABILITIES: &'static [&'static str] = &[
        "bounded_attempt",
        "distributional_response",
        "flow_realization",
    ];
    fn validate(owner: &str, states: OpeningStates<'_>) -> Result<(), String> {
        PopDistributedResponse::parse(states)?.validate(owner)
    }
}

struct AttributedModel<'a> {
    estimate: &'a Map<String, Value>,
    provenance: &'a Map<String, Value>,
}
impl<'a> AttributedModel<'a> {
    fn parse(states: OpeningStates<'a>) -> Result<Self, String> {
        Ok(Self {
            estimate: states.object_for(".attributed_estimate")?,
            provenance: states.object_for(".provenance")?,
        })
    }
    fn validate(&self, owner: &str) -> Result<(), String> {
        if self.estimate.is_empty() || self.provenance.is_empty() {
            return Err(format!(
                "fidelity_state: {owner} requires a populated first-order estimate and provenance"
            ));
        }
        Ok(())
    }
}
impl sealed::Sealed for AttributedModel<'_> {}
impl FidelityModel for AttributedModel<'_> {
    const TIER: FidelityTier = FidelityTier::AttributedModel;
    const CLADES: &'static [&'static str] = &[
        "DecisionBody",
        "HouseholdCohort",
        "Institution",
        "NamedFirm",
        "OrganizationCohort",
        "Person",
        "PersonPopulationCell",
    ];
    const REQUIRED_STATE: &'static str = "observer_owned_estimate, provenance";
    const CAPABILITIES: &'static [&'static str] =
        &["first_order_estimate", "provenanced_observation"];
    fn validate(owner: &str, states: OpeningStates<'_>) -> Result<(), String> {
        AttributedModel::parse(states)?.validate(owner)
    }
}

struct MechanicalOrAdapter<'a> {
    subject_state: &'a Value,
}
impl<'a> MechanicalOrAdapter<'a> {
    fn parse(states: OpeningStates<'a>) -> Result<Self, String> {
        let subject_state = states
            .states
            .first()
            .and_then(|state| state.get("value"))
            .ok_or_else(|| {
                format!(
                    "fidelity_state: {} requires subject-owned canonical or boundary state",
                    states.owner
                )
            })?;
        Ok(Self { subject_state })
    }
    fn validate(&self, owner: &str) -> Result<(), String> {
        if self.subject_state.is_null() {
            return Err(format!(
                "fidelity_state: {owner} mechanical state may not be null"
            ));
        }
        Ok(())
    }
}
impl sealed::Sealed for MechanicalOrAdapter<'_> {}
impl FidelityModel for MechanicalOrAdapter<'_> {
    const TIER: FidelityTier = FidelityTier::MechanicalOrAdapter;
    const CLADES: &'static [&'static str] = &[
        "Agreement",
        "BoundaryAdapter",
        "Coalition",
        "DecisionBody",
        "Facility",
        "FederatedSystem",
        "Generator",
        "HouseholdCohort",
        "IndustryCohort",
        "Institution",
        "LegalInstrument",
        "Market",
        "MechanicalSystem",
        "NamedFirm",
        "Network",
        "Office",
        "OrganizationCohort",
        "Outlet",
        "PersonPopulationCell",
        "PopLens",
        "PublishedReference",
        "Record",
        "Region",
        "ScheduledProcess",
        "SovereignSystem",
        "StaffUnit",
        "StatefulExternalProcess",
    ];
    const REQUIRED_STATE: &'static str = "subject-owned canonical or boundary state";
    const CAPABILITIES: &'static [&'static str] =
        &["deterministic_transition", "observation", "persistence"];
    fn validate(owner: &str, states: OpeningStates<'_>) -> Result<(), String> {
        MechanicalOrAdapter::parse(states)?.validate(owner)
    }
}

pub(crate) const DECLARATIONS: &[ModelDeclaration] = &[
    declaration::<NamedCognition>(),
    declaration::<LimitedRoleHolder<'static>>(),
    declaration::<ParticipantDistribution<'static>>(),
    declaration::<OrganizationCohortResponse<'static>>(),
    declaration::<PopDistributedResponse<'static>>(),
    declaration::<AttributedModel<'static>>(),
    declaration::<MechanicalOrAdapter<'static>>(),
];
