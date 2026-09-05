//! Evidence-bounded routing.
//!
//! This module is deliberately supplied with delivered artifacts and dated
//! operational metadata. It has no path to canonical state or scenario input.

mod access;
mod options;

pub(crate) use access::{
    AccessConflictChoice, AccessConflictChoiceKind, AccessDecision, resolve_access,
};
pub(crate) use options::{
    AuthoredRoutingPolicy, DeliveredArtifact, DisplacedWork, EvidenceUncertainty, RoutingPlanner,
    RoutingRequest, TaskForecast, UnitAvailability,
};
