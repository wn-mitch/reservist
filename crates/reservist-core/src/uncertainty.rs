use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) enum UncertaintyKind {
    #[serde(rename = "measurement")]
    Measurement,
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "strategic")]
    Strategic,
    #[serde(rename = "institutional")]
    Institutional,
    #[serde(rename = "aleatory")]
    Aleatory,
    #[serde(rename = "reflexive")]
    Reflexive,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct UncertaintyNote {
    pub(crate) kind: UncertaintyKind,
    pub(crate) description: String,
    #[serde(default)]
    pub(crate) evidence_refs: Vec<String>,
}
