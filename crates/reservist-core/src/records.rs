use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum ReceiptStage {
    #[serde(rename = "PROPOSAL")]
    Proposal,
    #[serde(rename = "AUTHORIZATION")]
    Authorization,
    #[serde(rename = "EXECUTION")]
    Execution,
    #[serde(rename = "SETTLEMENT")]
    Settlement,
    #[serde(rename = "OBSERVED EFFECT")]
    ObservedEffect,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct StageReceipt {
    pub receipt_id: String,
    pub stage: ReceiptStage,
    pub owner_id: String,
    pub timestamp: String,
    pub status: String,
    pub source_record_id: String,
    pub epistemic_scope: String,
    pub details: Value,
}
impl StageReceipt {
    pub(crate) fn to_dict(&self) -> Value {
        serde_json::to_value(self).expect("receipt is JSON data")
    }
}
