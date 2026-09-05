use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::time::Instant;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AudienceEdge {
    pub(crate) edge_id: String,
    pub(crate) source_id: String,
    pub(crate) artifact_kind: String,
    pub(crate) recipient_id: String,
    pub(crate) delay_minutes: i64,
    pub(crate) framing: String,
    pub(crate) access_scope: String,
    pub(crate) attention_probability: f64,
    pub(crate) revision_probability: f64,
    pub(crate) order_probability: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AudienceReception {
    pub(crate) delivery_id: String,
    pub(crate) edge_id: String,
    pub(crate) artifact_id: String,
    pub(crate) artifact_kind: String,
    pub(crate) recipient_id: String,
    pub(crate) delivery_time: String,
    pub(crate) access_scope: String,
    pub(crate) framing: String,
    pub(crate) exposed: bool,
    pub(crate) attended: bool,
    pub(crate) belief_revised: bool,
    pub(crate) order_intended: bool,
    pub(crate) policy_path_estimate: Option<f64>,
    pub(crate) claim_ids: Vec<String>,
}

#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
#[error("{0}")]
pub(crate) struct DeliveryError(pub(crate) String);

fn required_string(value: &Value, field: &str) -> Result<String, DeliveryError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| DeliveryError(format!("missing string {field}")))
}

fn required_bool(value: &Value, field: &str) -> Result<bool, DeliveryError> {
    value
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| DeliveryError(format!("missing boolean {field}")))
}

fn required_f64(value: &Value, field: &str) -> Result<f64, DeliveryError> {
    value
        .get(field)
        .and_then(Value::as_f64)
        .ok_or_else(|| DeliveryError(format!("missing number {field}")))
}

fn required_strings(value: &Value, field: &str) -> Result<Vec<String>, DeliveryError> {
    value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| DeliveryError(format!("missing string array {field}")))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| DeliveryError(format!("{field} must contain strings")))
        })
        .collect()
}

impl AudienceEdge {
    pub(crate) fn from_dict(value: &Value) -> Result<Self, DeliveryError> {
        let edge = Self {
            edge_id: required_string(value, "edge_id")?,
            source_id: required_string(value, "source_id")?,
            artifact_kind: required_string(value, "artifact_kind")?,
            recipient_id: required_string(value, "recipient_id")?,
            delay_minutes: value
                .get("delay_minutes")
                .and_then(Value::as_i64)
                .ok_or_else(|| DeliveryError("missing integer delay_minutes".into()))?,
            framing: required_string(value, "framing")?,
            access_scope: required_string(value, "access_scope")?,
            attention_probability: required_f64(value, "attention_probability")?,
            revision_probability: required_f64(value, "revision_probability")?,
            order_probability: required_f64(value, "order_probability")?,
        };
        edge.validate()?;
        Ok(edge)
    }

    fn validate(&self) -> Result<(), DeliveryError> {
        if self.delay_minutes < 0 {
            return Err(DeliveryError(
                "audience delivery delay cannot be negative".into(),
            ));
        }
        if [
            self.attention_probability,
            self.revision_probability,
            self.order_probability,
        ]
        .iter()
        .any(|probability| !(0.0..=1.0).contains(probability))
        {
            return Err(DeliveryError(
                "audience stage probabilities must be between zero and one".into(),
            ));
        }
        Ok(())
    }

    fn to_dict(&self) -> Value {
        json!({
            "access_scope": self.access_scope,
            "artifact_kind": self.artifact_kind,
            "attention_probability": self.attention_probability,
            "delay_minutes": self.delay_minutes,
            "edge_id": self.edge_id,
            "framing": self.framing,
            "order_probability": self.order_probability,
            "recipient_id": self.recipient_id,
            "revision_probability": self.revision_probability,
            "source_id": self.source_id,
        })
    }
}

impl AudienceReception {
    pub(crate) fn from_dict(value: &Value) -> Result<Self, DeliveryError> {
        Ok(Self {
            delivery_id: required_string(value, "delivery_id")?,
            edge_id: required_string(value, "edge_id")?,
            artifact_id: required_string(value, "artifact_id")?,
            artifact_kind: required_string(value, "artifact_kind")?,
            recipient_id: required_string(value, "recipient_id")?,
            delivery_time: required_string(value, "delivery_time")?,
            access_scope: required_string(value, "access_scope")?,
            framing: required_string(value, "framing")?,
            exposed: required_bool(value, "exposed")?,
            attended: required_bool(value, "attended")?,
            belief_revised: required_bool(value, "belief_revised")?,
            order_intended: required_bool(value, "order_intended")?,
            policy_path_estimate: match value.get("policy_path_estimate") {
                Some(Value::Null) | None => None,
                Some(estimate) => Some(estimate.as_f64().ok_or_else(|| {
                    DeliveryError("policy_path_estimate must be a number or null".into())
                })?),
            },
            claim_ids: required_strings(value, "claim_ids")?,
        })
    }

    pub(crate) fn to_dict(&self) -> Value {
        json!({
            "access_scope": self.access_scope,
            "artifact_id": self.artifact_id,
            "artifact_kind": self.artifact_kind,
            "attended": self.attended,
            "belief_revised": self.belief_revised,
            "claim_ids": self.claim_ids,
            "delivery_id": self.delivery_id,
            "delivery_time": self.delivery_time,
            "edge_id": self.edge_id,
            "exposed": self.exposed,
            "framing": self.framing,
            "order_intended": self.order_intended,
            "policy_path_estimate": self.policy_path_estimate,
            "recipient_id": self.recipient_id,
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct AudienceBeliefStore {
    policy_path: BTreeMap<String, f64>,
    history: Vec<AudienceReception>,
}

impl AudienceBeliefStore {
    pub(crate) fn record(&mut self, reception: AudienceReception) {
        if reception.belief_revised
            && let Some(estimate) = reception.policy_path_estimate
        {
            let updated = self
                .policy_path
                .get(&reception.recipient_id)
                .map_or(estimate, |prior| round_python((prior + estimate) / 2.0, 6));
            self.policy_path
                .insert(reception.recipient_id.clone(), updated);
        }
        self.history.push(reception);
    }

    #[cfg(test)]
    pub(crate) fn estimate(&self, recipient_id: &str) -> Option<f64> {
        self.policy_path.get(recipient_id).copied()
    }

    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({
            "history": self.history.iter().map(AudienceReception::to_dict).collect::<Vec<_>>(),
            "policy_path": self.policy_path,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct DirectAudienceRouter {
    edges: Vec<AudienceEdge>,
    seed: i64,
    pub(crate) beliefs: AudienceBeliefStore,
}

impl DirectAudienceRouter {
    pub(crate) fn new(
        edges: impl IntoIterator<Item = AudienceEdge>,
        seed: i64,
    ) -> Result<Self, DeliveryError> {
        let mut edges: Vec<_> = edges.into_iter().collect();
        let identifiers: BTreeSet<_> = edges.iter().map(|edge| edge.edge_id.as_str()).collect();
        if identifiers.len() != edges.len() {
            return Err(DeliveryError("duplicate audience edge identifier".into()));
        }
        for edge in &edges {
            edge.validate()?;
        }
        edges.sort_by(|left, right| left.edge_id.cmp(&right.edge_id));
        Ok(Self {
            edges,
            seed,
            beliefs: AudienceBeliefStore::default(),
        })
    }

    pub(crate) fn from_manifest(value: &Value, seed: i64) -> Result<Self, DeliveryError> {
        let edges = value
            .get("delivery_edges")
            .and_then(Value::as_array)
            .map_or_else(
                || Ok(Vec::new()),
                |rows| rows.iter().map(AudienceEdge::from_dict).collect(),
            )?;
        Self::new(edges, seed)
    }

    pub(crate) fn edges(&self) -> &[AudienceEdge] {
        &self.edges
    }

    #[cfg(test)]
    pub(crate) fn deliver(
        &mut self,
        source_id: &str,
        artifact_kind: &str,
        artifact_id: &str,
        published_at: &str,
        claims: &[Value],
    ) -> Result<Vec<AudienceReception>, DeliveryError> {
        let receptions =
            self.plan_deliveries(source_id, artifact_kind, artifact_id, published_at, claims)?;
        for reception in &receptions {
            self.beliefs.record(reception.clone());
        }
        Ok(receptions)
    }

    pub(crate) fn plan_deliveries(
        &self,
        source_id: &str,
        artifact_kind: &str,
        artifact_id: &str,
        published_at: &str,
        claims: &[Value],
    ) -> Result<Vec<AudienceReception>, DeliveryError> {
        let published_at = Instant::parse(published_at).map_err(|error| DeliveryError(error.0))?;
        let claim_ids = claims
            .iter()
            .map(|claim| required_string(claim, "claim_id"))
            .collect::<Result<Vec<_>, _>>()?;
        self.edges
            .iter()
            .filter(|edge| edge.source_id == source_id && edge.artifact_kind == artifact_kind)
            .map(|edge| {
                let attended =
                    self.draw(&edge.edge_id, artifact_id, "attention") < edge.attention_probability;
                let belief_revised = attended
                    && self.draw(&edge.edge_id, artifact_id, "revision")
                        < edge.revision_probability;
                let order_intended = belief_revised
                    && self.draw(&edge.edge_id, artifact_id, "order") < edge.order_probability;
                let policy_path_estimate = belief_revised
                    .then(|| self.interpret(claims, &edge.framing, &edge.edge_id, artifact_id))
                    .transpose()?;
                let delivery_time = published_at
                    .add_minutes(edge.delay_minutes)
                    .map_err(|error| DeliveryError(error.0))?
                    .to_string();
                Ok(AudienceReception {
                    delivery_id: format!(
                        "delivery.{}.{}",
                        edge.edge_id,
                        artifact_id.rsplit('.').next().unwrap_or(artifact_id)
                    ),
                    edge_id: edge.edge_id.clone(),
                    artifact_id: artifact_id.into(),
                    artifact_kind: artifact_kind.into(),
                    recipient_id: edge.recipient_id.clone(),
                    delivery_time,
                    access_scope: edge.access_scope.clone(),
                    framing: edge.framing.clone(),
                    exposed: true,
                    attended,
                    belief_revised,
                    order_intended,
                    policy_path_estimate,
                    claim_ids: claim_ids.clone(),
                })
            })
            .collect()
    }

    fn draw(&self, edge_id: &str, artifact_id: &str, stage: &str) -> f64 {
        let material = format!("{}|{edge_id}|{artifact_id}|{stage}", self.seed);
        let digest = Sha256::digest(material.as_bytes());
        let integer = u64::from_be_bytes(
            digest[..8]
                .try_into()
                .expect("SHA-256 digest has eight bytes"),
        );
        integer as f64 / 18_446_744_073_709_551_616.0
    }

    fn interpret(
        &self,
        claims: &[Value],
        framing: &str,
        edge_id: &str,
        artifact_id: &str,
    ) -> Result<f64, DeliveryError> {
        let scores = claims
            .iter()
            .map(|claim| {
                let category = required_string(claim, "magnitude_or_category")?;
                let predicate = required_string(claim, "predicate")?;
                Ok(match category.as_str() {
                    "standard_firming_step" => 0.76,
                    "further_firming_likely" => 0.9,
                    "current_target_maintained" => 0.32,
                    "no_action_authorized" => 0.4,
                    _ if predicate == "rising" => 0.67,
                    _ if predicate == "conditional" => 0.54,
                    _ => 0.5,
                })
            })
            .collect::<Result<Vec<f64>, DeliveryError>>()?;
        let baseline = if scores.is_empty() {
            0.5
        } else {
            scores.iter().sum::<f64>() / scores.len() as f64
        };
        let framing_bias = match framing {
            "hawkish" => 0.08,
            "cautious" => -0.04,
            "housing_sensitive" => 0.03,
            _ => 0.0,
        };
        let noise = (self.draw(edge_id, artifact_id, "interpretation") - 0.5) * 0.08;
        Ok(round_python(
            (baseline + framing_bias + noise).clamp(0.0, 1.0),
            6,
        ))
    }

    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({
            "beliefs": self.beliefs.snapshot_for_hash(),
            "edges": self.edges.iter().map(AudienceEdge::to_dict).collect::<Vec<_>>(),
            "seed": self.seed,
        })
    }
}

fn round_python(value: f64, decimal_places: usize) -> f64 {
    format!("{value:.decimal_places$}")
        .parse()
        .expect("formatted finite float parses")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edge(
        edge_id: &str,
        recipient_id: &str,
        attention: f64,
        revision: f64,
        order: f64,
    ) -> AudienceEdge {
        AudienceEdge::from_dict(&json!({
            "edge_id": edge_id, "source_id": "source.test", "artifact_kind": "REPORT",
            "recipient_id": recipient_id, "delay_minutes": 0, "framing": "balanced", "access_scope": "TEST",
            "attention_probability": attention, "revision_probability": revision, "order_probability": order,
        })).unwrap()
    }

    fn claims() -> Vec<Value> {
        vec![
            json!({"claim_id":"claim.test","magnitude_or_category":"standard_firming_step","predicate":"intended"}),
        ]
    }

    #[test]
    fn delivery_stages_require_the_previous_stage() {
        let router = DirectAudienceRouter::new(
            [
                edge("edge.exposure_only", "audience.exposure", 0.0, 1.0, 1.0),
                edge("edge.attention_only", "audience.attention", 1.0, 0.0, 1.0),
                edge("edge.revision_only", "audience.revision", 1.0, 1.0, 0.0),
            ],
            7,
        )
        .unwrap();
        let rows = router
            .plan_deliveries(
                "source.test",
                "REPORT",
                "report.test",
                "2006-03-28T14:15:00-05:00",
                &claims(),
            )
            .unwrap();
        let rows: BTreeMap<_, _> = rows
            .into_iter()
            .map(|row| (row.recipient_id.clone(), row))
            .collect();
        assert!(!rows["audience.exposure"].attended);
        assert!(!rows["audience.exposure"].belief_revised);
        assert!(!rows["audience.attention"].belief_revised);
        assert!(!rows["audience.attention"].order_intended);
        assert!(rows["audience.revision"].belief_revised);
        assert!(!rows["audience.revision"].order_intended);
    }

    #[test]
    fn keyed_delivery_outcomes_are_deterministic() {
        let router =
            DirectAudienceRouter::new([edge("edge.test", "audience.test", 1.0, 1.0, 1.0)], 7)
                .unwrap();
        let first = router
            .plan_deliveries(
                "source.test",
                "REPORT",
                "report.test",
                "2006-03-28T14:15:00-05:00",
                &claims(),
            )
            .unwrap();
        let second = router
            .plan_deliveries(
                "source.test",
                "REPORT",
                "report.test",
                "2006-03-28T14:15:00-05:00",
                &claims(),
            )
            .unwrap();
        assert_eq!(first, second);
        assert_eq!(
            router.draw("edge.test", "report.test", "attention"),
            router.draw("edge.test", "report.test", "attention")
        );
    }
}
