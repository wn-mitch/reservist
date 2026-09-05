use serde::Serialize;

use super::FidelityTier;
use super::models::DECLARATIONS;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FidelityPermission {
    pub tier: FidelityTier,
    pub identity_clade: String,
    pub required_state: String,
    pub capabilities: Vec<String>,
    pub runtime_promotion_permitted: bool,
}

/// Returns the complete compatibility closure emitted from the sealed model declarations.
pub fn permission_matrix() -> Vec<FidelityPermission> {
    let mut rows: Vec<_> = DECLARATIONS
        .iter()
        .flat_map(|model| {
            model.clades.iter().map(move |clade| FidelityPermission {
                tier: model.tier,
                identity_clade: (*clade).to_owned(),
                required_state: model.required_state.to_owned(),
                capabilities: model
                    .capabilities
                    .iter()
                    .map(|capability| (*capability).to_owned())
                    .collect(),
                runtime_promotion_permitted: false,
            })
        })
        .collect();
    rows.sort_by(|left, right| {
        left.identity_clade
            .cmp(&right.identity_clade)
            .then_with(|| left.tier.cmp(&right.tier))
    });
    rows
}

/// Stable save-engine identity for the executable fidelity contract.
pub fn permission_matrix_hash() -> String {
    crate::canon::sha256(
        &serde_json::to_value(permission_matrix()).expect("fidelity metadata is serializable"),
    )
}
