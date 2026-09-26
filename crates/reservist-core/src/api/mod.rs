use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Owned compiler output supplied when creating a session, not a player view.
/// Runtime state is constructed privately; clients never receive this as a projection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrozenScenario {
    pub catalog_slice: Value,
    pub manifest: Value,
    pub initialization: Value,
    pub tape: Value,
    pub authority_content: Value,
    pub scenario_hash: String,
}

#[cfg(feature = "tooling")]
pub mod tooling;

mod command;
mod desk_views;
mod ops;
mod receipt;
mod session;
#[path = "../stewardship/mod.rs"]
mod stewardship;
pub mod views;
mod workflow;

pub use command::{Command, CommandAction};
pub use ops::{RequestTiming, View};
pub use receipt::Receipt;
pub use session::{Rejected, Session};
pub use workflow::validate_interaction_contract;

/// The package a session opens with when the caller names none.
pub fn default_package_id(scenario: &FrozenScenario) -> String {
    crate::packages::default_package_id(scenario)
}
