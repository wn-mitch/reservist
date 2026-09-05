use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::communication::CommunicationAct;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Report {
    pub(crate) report_id: String,
    pub(crate) outlet_id: String,
    pub(crate) source_communication_ref: String,
    pub(crate) selected_claims: Vec<Value>,
    pub(crate) headline: String,
    pub(crate) framing: String,
    pub(crate) omissions: Vec<String>,
    pub(crate) audience_targets: Vec<String>,
    pub(crate) publication_time: String,
}

impl Report {
    pub(crate) fn to_dict(&self) -> Value {
        json!({
            "audience_targets": self.audience_targets,
            "framing": self.framing,
            "headline": self.headline,
            "omissions": self.omissions,
            "outlet_id": self.outlet_id,
            "publication_time": self.publication_time,
            "record_id": self.report_id,
            "record_kind": "Report",
            "selected_claims": self.selected_claims,
            "source_communication_ref": self.source_communication_ref,
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct LoonbergOutlet;

impl LoonbergOutlet {
    pub(crate) const OUTLET_ID: &'static str = "outlet.media.loonberg";

    pub(crate) fn publish(
        &self,
        communication: &CommunicationAct,
        publication_time: impl Into<String>,
        audience_targets: impl IntoIterator<Item = String>,
    ) -> Report {
        let selected_claims: Vec<_> = communication
            .claims
            .iter()
            .map(|claim| claim.to_dict())
            .collect();
        let categories: Vec<_> = selected_claims
            .iter()
            .filter_map(|claim| claim.get("magnitude_or_category").and_then(Value::as_str))
            .collect();
        let (headline, framing) = if categories.contains(&"standard_firming_step") {
            (
                "HONK - FOMC FIRMS TARGET; NEXT STEP REMAINS CONTESTED",
                "hawkish",
            )
        } else if categories.contains(&"current_target_maintained") {
            (
                "HONK - FOMC HOLDS TARGET, RETAINS INFLATION WARNING",
                "balanced",
            )
        } else {
            (
                "HONK - FOMC RECORDS NO AUTHORIZED TARGET ACTION",
                "cautious",
            )
        };
        Report {
            report_id: format!(
                "report.loonberg.{}",
                communication
                    .communication_id
                    .rsplit('.')
                    .next()
                    .unwrap_or(&communication.communication_id)
            ),
            outlet_id: Self::OUTLET_ID.into(),
            source_communication_ref: communication.communication_id.clone(),
            selected_claims,
            headline: headline.into(),
            framing: framing.into(),
            omissions: vec![
                "No claim resolves the intermeeting inflation or employment path.".into(),
            ],
            audience_targets: audience_targets.into_iter().collect(),
            publication_time: publication_time.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{claims::ClaimRegistry, communication::CommunicationAct};

    #[test]
    fn report_is_claim_only_and_inert() {
        let registry = ClaimRegistry::new();
        let claim = registry
            .bind(
                "claim.target_range_firmed",
                "authorization.test",
                vec!["record.test".into()],
            )
            .unwrap();
        let communication = CommunicationAct {
            communication_id: "communication.fomc.20060328".into(),
            speaker_id: "speaker".into(),
            authorizing_body_id: "body".into(),
            venue: "venue".into(),
            intended_audiences: vec![],
            claims: vec![claim],
            published_at: "2006-03-28T14:15:00-05:00".into(),
            authorization_ref: "authorization.test".into(),
        };
        let report = LoonbergOutlet.publish(
            &communication,
            "2006-03-28T14:16:00-05:00",
            vec!["audience.test".into()],
        );
        assert_eq!(report.framing, "hawkish");
        assert!(report.to_dict().get("economic_state").is_none());
        assert_eq!(
            report.selected_claims,
            communication
                .claims
                .iter()
                .map(|claim| claim.to_dict())
                .collect::<Vec<_>>()
        );
    }
}
