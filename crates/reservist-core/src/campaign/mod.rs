//! Campaign continuity is a non-causal aggregate: it records lawful chairmanship
//! transitions and Stewardship attribution without owning institutional state.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuccessionCause {
    Resignation,
    Incapacity,
    Death,
    Removal,
    FailedRenomination,
    TermExpiry,
    StatutoryReorganization,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SuccessionRule {
    pub rule_id: String,
    pub office_id: String,
    pub cause: SuccessionCause,
    pub eligible_successor_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChairmanshipProgram {
    pub program_id: String,
    pub revision_id: String,
    pub aspirations: Vec<String>,
    pub inherited_project_ids: Vec<String>,
    pub prior_claim_ids: Vec<String>,
    pub relationship_ids: Vec<String>,
    pub commitment_refs: Vec<String>,
}
impl ChairmanshipProgram {
    pub fn validate(&self) -> Result<(), String> {
        if self.program_id.is_empty() || self.revision_id.is_empty() || self.aspirations.is_empty()
        {
            Err("chairmanship program must identify itself and contain an aspiration".into())
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChairmanshipDossier {
    pub chairmanship_id: String,
    pub chair_person_id: String,
    pub office_id: String,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub inherited_official_refs: Vec<String>,
    pub incoming_information_refs: Vec<String>,
    pub program: ChairmanshipProgram,
    pub program_history: Vec<ChairmanshipProgram>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDisposition {
    Accept,
    AcceptWithChairResponse,
    RequestRevision,
    AcceptWithSupplementalReview,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewRecord {
    pub review_id: String,
    pub review_version: u32,
    pub owner_chairmanship_id: String,
    pub disclosed: bool,
    pub final_required: bool,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    #[serde(default)]
    pub dissent_refs: Vec<String>,
    #[serde(default)]
    pub confidence_refs: Vec<String>,
    #[serde(default)]
    pub timing_boundary: String,
    #[serde(default)]
    pub access_boundary: String,
    #[serde(default)]
    pub capacity_reservation_id: Option<String>,
    #[serde(default)]
    pub capacity_owner_id: String,
    #[serde(default)]
    pub capacity_units: i64,
    #[serde(default)]
    pub capacity_duration_minutes: i64,
    #[serde(default)]
    pub capacity_releases_at: Option<String>,
    pub disposed: Option<ReviewDisposition>,
    pub disposition_history: Vec<ReviewDisposition>,
    #[serde(default)]
    pub response_record_id: Option<String>,
    pub supplemental_review_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LedgerEntryKind {
    Award,
    CompensatingAdjustment,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StewardshipEntry {
    pub entry_id: String,
    pub kind: LedgerEntryKind,
    pub review_id: String,
    pub review_version: u32,
    pub finding_id: String,
    pub chairmanship_id: String,
    pub delta: i64,
    pub witness_ids: Vec<String>,
    pub compensates_entry_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CampaignState {
    pub campaign_id: String,
    pub endpoint: String,
    pub current_chairmanship_id: String,
    pub dossiers: Vec<ChairmanshipDossier>,
    pub succession_rules: Vec<SuccessionRule>,
    pub successor_programs: BTreeMap<String, ChairmanshipProgram>,
    pub reviews: BTreeMap<String, ReviewRecord>,
    pub ledger: Vec<StewardshipEntry>,
    #[serde(default)]
    pub processed_event_ids: Vec<String>,
    pub endpoint_reached: bool,
    pub terminal: bool,
    pub final_dossier_id: Option<String>,
}
impl CampaignState {
    pub fn new(
        campaign_id: String,
        endpoint: String,
        initial: ChairmanshipDossier,
        succession_rules: Vec<SuccessionRule>,
        reviews: Vec<ReviewRecord>,
    ) -> Result<Self, String> {
        initial.program.validate()?;
        if campaign_id.is_empty() || endpoint.is_empty() {
            return Err("campaign requires identity and endpoint".into());
        }
        let current_chairmanship_id = initial.chairmanship_id.clone();
        let mut keyed = BTreeMap::new();
        for review in reviews {
            if review.review_id.is_empty()
                || review.review_version == 0
                || keyed.insert(review.review_id.clone(), review).is_some()
            {
                return Err("campaign reviews require unique nonempty IDs and versions".into());
            }
        }
        Ok(Self {
            campaign_id,
            endpoint,
            current_chairmanship_id,
            dossiers: vec![initial],
            succession_rules,
            successor_programs: BTreeMap::new(),
            reviews: keyed,
            ledger: Vec::new(),
            processed_event_ids: Vec::new(),
            endpoint_reached: false,
            terminal: false,
            final_dossier_id: None,
        })
    }
    pub fn total(&self) -> Result<i64, String> {
        self.ledger.iter().try_fold(0i64, |sum, entry| {
            sum.checked_add(entry.delta)
                .ok_or_else(|| "campaign stewardship total overflow".into())
        })
    }
    pub fn program_for_successor(&self, successor_id: &str) -> Result<ChairmanshipProgram, String> {
        self.successor_programs
            .get(successor_id)
            .cloned()
            .ok_or_else(|| {
                format!("lawful successor has no authored chairmanship program: {successor_id}")
            })
    }
    pub fn revise_program(
        &mut self,
        program_id: &str,
        revision: ChairmanshipProgram,
    ) -> Result<(), String> {
        if self.terminal {
            return Err("campaign_terminal".into());
        }
        revision.validate()?;
        let dossier = self
            .dossiers
            .last_mut()
            .ok_or("campaign has no current dossier")?;
        if dossier.program.program_id != program_id
            || revision.program_id != program_id
            || revision.revision_id == dossier.program.revision_id
        {
            return Err(
                "program revision does not prospectively revise the current program".into(),
            );
        }
        dossier.program_history.push(dossier.program.clone());
        dossier.program = revision;
        Ok(())
    }
    pub fn dispose_review(
        &mut self,
        review_id: &str,
        version: u32,
        disposition: ReviewDisposition,
        supplemental: Option<String>,
    ) -> Result<bool, String> {
        if self.terminal {
            return Err("campaign_terminal".into());
        }
        let review = self
            .reviews
            .get_mut(review_id)
            .ok_or("unknown campaign review")?;
        if review.review_version != version {
            return Err("review version mismatch".into());
        }
        if let Some(existing) = &review.disposed {
            if existing == &disposition && review.supplemental_review_id == supplemental {
                return Ok(false);
            }
            return Err("conflicting review disposition".into());
        }
        if matches!(disposition, ReviewDisposition::AcceptWithSupplementalReview)
            && supplemental.as_deref().unwrap_or("").is_empty()
        {
            return Err("supplemental disposition requires a supplemental review".into());
        }
        if matches!(disposition, ReviewDisposition::RequestRevision) {
            review.disposition_history.push(disposition);
            review.review_version = review
                .review_version
                .checked_add(1)
                .ok_or("review version overflow")?;
            return Ok(true);
        }
        review.supplemental_review_id = supplemental;
        review.disposed = Some(disposition.clone());
        review.disposition_history.push(disposition);
        Ok(true)
    }
    pub fn record_award(
        &mut self,
        review_id: &str,
        version: u32,
        finding_id: &str,
        delta: i64,
        witness_ids: Vec<String>,
    ) -> Result<bool, String> {
        if self.terminal {
            return Err("campaign_terminal".into());
        }
        let review = self
            .reviews
            .get(review_id)
            .ok_or("unknown campaign review")?;
        if review.review_version != version || review.disposed.is_none() {
            return Err("award requires a disposed review version".into());
        }
        let witnesses: BTreeSet<_> = witness_ids.iter().collect();
        if witnesses.len() != witness_ids.len() || witness_ids.is_empty() {
            return Err("award requires unique witnessed attribution".into());
        }
        if self.ledger.iter().any(|entry| {
            entry.review_id == review_id
                && entry.review_version == version
                && entry.finding_id == finding_id
                && matches!(entry.kind, LedgerEntryKind::Award)
        }) {
            return Ok(false);
        }
        let id = format!("stewardship.{review_id}.{version}.{finding_id}");
        self.ledger.push(StewardshipEntry {
            entry_id: id,
            kind: LedgerEntryKind::Award,
            review_id: review_id.into(),
            review_version: version,
            finding_id: finding_id.into(),
            chairmanship_id: self.current_chairmanship_id.clone(),
            delta,
            witness_ids,
            compensates_entry_id: None,
        });
        Ok(true)
    }
    pub fn compensate(
        &mut self,
        prior_entry_id: &str,
        new_review_id: &str,
        new_version: u32,
        witness_ids: Vec<String>,
        delta: i64,
    ) -> Result<(), String> {
        let (finding_id, chairmanship_id) = self
            .ledger
            .iter()
            .find(|entry| entry.entry_id == prior_entry_id)
            .map(|entry| (entry.finding_id.clone(), entry.chairmanship_id.clone()))
            .ok_or("missing prior stewardship entry")?;
        if self
            .ledger
            .iter()
            .any(|entry| entry.compensates_entry_id.as_deref() == Some(prior_entry_id))
        {
            return Err("prior stewardship entry already compensated".into());
        }
        self.ledger.push(StewardshipEntry {
            entry_id: format!("stewardship.compensation.{prior_entry_id}.{new_version}"),
            kind: LedgerEntryKind::CompensatingAdjustment,
            review_id: new_review_id.into(),
            review_version: new_version,
            finding_id,
            chairmanship_id,
            delta,
            witness_ids,
            compensates_entry_id: Some(prior_entry_id.into()),
        });
        Ok(())
    }
    pub fn commission_supplemental(
        &mut self,
        review_id: &str,
        version: u32,
        supplemental_review_id: String,
    ) -> Result<(), String> {
        let source = self
            .reviews
            .get(review_id)
            .cloned()
            .ok_or("unknown campaign review")?;
        if source.review_version != version {
            return Err("review version mismatch".into());
        }
        if self.reviews.contains_key(&supplemental_review_id) {
            return Err("duplicate supplemental review id".into());
        }
        self.reviews.insert(
            supplemental_review_id.clone(),
            ReviewRecord {
                review_id: supplemental_review_id,
                review_version: 1,
                owner_chairmanship_id: self.current_chairmanship_id.clone(),
                disclosed: source.disclosed,
                final_required: source.final_required,
                evidence_refs: source.evidence_refs,
                dissent_refs: source.dissent_refs,
                confidence_refs: source.confidence_refs,
                timing_boundary: source.timing_boundary,
                access_boundary: source.access_boundary,
                capacity_reservation_id: source.capacity_reservation_id,
                capacity_owner_id: source.capacity_owner_id,
                capacity_units: source.capacity_units,
                capacity_duration_minutes: source.capacity_duration_minutes,
                capacity_releases_at: None,
                disposed: None,
                disposition_history: Vec::new(),
                response_record_id: None,
                supplemental_review_id: None,
            },
        );
        Ok(())
    }
    pub fn succeed(
        &mut self,
        cause: SuccessionCause,
        rule_id: &str,
        successor_id: String,
        successor_program: ChairmanshipProgram,
        at: String,
        incoming_information_refs: Vec<String>,
    ) -> Result<(), String> {
        if self.terminal {
            return Err("campaign_terminal".into());
        }
        let rule = self
            .succession_rules
            .iter()
            .find(|rule| rule.rule_id == rule_id && rule.cause == cause)
            .ok_or("unregistered campaign succession rule")?;
        if rule
            .eligible_successor_ids
            .iter()
            .filter(|id| *id == &successor_id)
            .count()
            != 1
        {
            return Err("lawful successor is missing, ineligible, or ambiguous".into());
        }
        successor_program.validate()?;
        let office_id = rule.office_id.clone();
        let inherited_official_refs = {
            let prior = self
                .dossiers
                .last_mut()
                .ok_or("campaign has no outgoing chairmanship")?;
            prior.ended_at = Some(at.clone());
            prior.inherited_official_refs.clone()
        };
        let next = ChairmanshipDossier {
            chairmanship_id: format!(
                "chairmanship.{}.{}",
                self.campaign_id,
                self.dossiers.len() + 1
            ),
            chair_person_id: successor_id,
            office_id,
            started_at: at,
            ended_at: None,
            inherited_official_refs,
            incoming_information_refs,
            program: successor_program,
            program_history: Vec::new(),
        };
        self.current_chairmanship_id = next.chairmanship_id.clone();
        self.dossiers.push(next);
        Ok(())
    }
    pub fn reach_endpoint(&mut self) {
        self.endpoint_reached = true;
        self.maybe_finalize();
    }
    pub fn finalize_if_ready(&mut self) {
        self.maybe_finalize();
    }
    pub fn from_content(campaign: &Value, reviews: &Value) -> Result<Self, String> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct CampaignContent {
            campaign_id: String,
            endpoint: String,
            initial_dossier: ChairmanshipDossier,
            succession_rules: Vec<SuccessionRule>,
            successor_programs: BTreeMap<String, ChairmanshipProgram>,
        }
        let content: CampaignContent = serde_json::from_value(campaign.clone())
            .map_err(|error| format!("invalid frozen campaign content: {error}"))?;
        let reviews: Vec<ReviewRecord> = serde_json::from_value(reviews.clone())
            .map_err(|error| format!("invalid frozen campaign reviews: {error}"))?;
        let mut state = Self::new(
            content.campaign_id,
            content.endpoint,
            content.initial_dossier,
            content.succession_rules,
            reviews,
        )?;
        for program in content.successor_programs.values() {
            program.validate()?;
        }
        for successor_id in state
            .succession_rules
            .iter()
            .flat_map(|rule| &rule.eligible_successor_ids)
        {
            if !content.successor_programs.contains_key(successor_id) {
                return Err(format!(
                    "lawful successor has no authored chairmanship program: {successor_id}"
                ));
            }
        }
        state.successor_programs = content.successor_programs;
        Ok(state)
    }
    fn maybe_finalize(&mut self) {
        if self.endpoint_reached
            && self
                .reviews
                .values()
                .filter(|review| review.final_required)
                .all(|review| review.disposed.is_some())
            && !self.terminal
        {
            self.terminal = true;
            self.final_dossier_id = Some(format!("campaign_dossier.{}", self.campaign_id));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn program(revision: &str) -> ChairmanshipProgram {
        ChairmanshipProgram {
            program_id: "program.chair".into(),
            revision_id: revision.into(),
            aspirations: vec!["aspiration.stability".into()],
            inherited_project_ids: vec![],
            prior_claim_ids: vec![],
            relationship_ids: vec![],
            commitment_refs: vec![],
        }
    }
    fn campaign() -> CampaignState {
        CampaignState::new(
            "campaign.test".into(),
            "2006-12-31T17:00:00-05:00".into(),
            ChairmanshipDossier {
                chairmanship_id: "chairmanship.test.1".into(),
                chair_person_id: "person.outgoing".into(),
                office_id: "office.chair".into(),
                started_at: "2006-01-01T00:00:00-05:00".into(),
                ended_at: None,
                inherited_official_refs: vec!["official.condition".into()],
                incoming_information_refs: vec!["record.public".into()],
                program: program("initial"),
                program_history: vec![],
            },
            vec![SuccessionRule {
                rule_id: "rule.resignation".into(),
                office_id: "office.chair".into(),
                cause: SuccessionCause::Resignation,
                eligible_successor_ids: vec!["person.successor".into()],
            }],
            vec![ReviewRecord {
                review_id: "review.final".into(),
                review_version: 1,
                owner_chairmanship_id: "chairmanship.test.1".into(),
                disclosed: true,
                final_required: true,
                evidence_refs: vec![],
                dissent_refs: vec![],
                confidence_refs: vec![],
                timing_boundary: String::new(),
                access_boundary: String::new(),
                capacity_reservation_id: None,
                capacity_owner_id: String::new(),
                capacity_units: 0,
                capacity_duration_minutes: 0,
                capacity_releases_at: None,
                disposed: None,
                disposition_history: vec![],
                response_record_id: None,
                supplemental_review_id: None,
            }],
        )
        .unwrap()
    }
    #[test]
    fn lawful_succession_preserves_official_burden_without_private_transfer() {
        let mut state = campaign();
        state
            .succeed(
                SuccessionCause::Resignation,
                "rule.resignation",
                "person.successor".into(),
                program("successor"),
                "2006-02-01T00:00:00-05:00".into(),
                vec!["record.authorized".into()],
            )
            .unwrap();
        assert_eq!(state.dossiers.len(), 2);
        assert_eq!(
            state.dossiers[1].inherited_official_refs,
            vec!["official.condition"]
        );
        assert_eq!(
            state.dossiers[1].incoming_information_refs,
            vec!["record.authorized"]
        );
        assert_eq!(state.dossiers[0].chair_person_id, "person.outgoing");
    }
    #[test]
    fn disposed_review_award_is_idempotent_and_terminalizes_only_after_endpoint() {
        let mut state = campaign();
        assert!(
            state
                .dispose_review("review.final", 1, ReviewDisposition::Accept, None)
                .unwrap()
        );
        assert!(
            state
                .record_award(
                    "review.final",
                    1,
                    "finding.test",
                    4,
                    vec!["witness.1".into()]
                )
                .unwrap()
        );
        assert!(
            !state
                .record_award(
                    "review.final",
                    1,
                    "finding.test",
                    4,
                    vec!["witness.1".into()]
                )
                .unwrap()
        );
        assert_eq!(state.total().unwrap(), 4);
        state.reach_endpoint();
        assert!(state.terminal);
        assert_eq!(
            state
                .revise_program("program.chair", program("later"))
                .unwrap_err(),
            "campaign_terminal"
        );
    }
    #[test]
    fn every_frozen_succession_cause_rebinds_the_lawful_successor() {
        for cause in [
            SuccessionCause::Resignation,
            SuccessionCause::Incapacity,
            SuccessionCause::Death,
            SuccessionCause::Removal,
            SuccessionCause::FailedRenomination,
            SuccessionCause::TermExpiry,
            SuccessionCause::StatutoryReorganization,
        ] {
            let mut state = campaign();
            state.succession_rules[0].cause = cause.clone();
            state
                .succeed(
                    cause,
                    "rule.resignation",
                    "person.successor".into(),
                    program("successor"),
                    "2006-02-01T00:00:00-05:00".into(),
                    vec![],
                )
                .unwrap();
            assert_eq!(
                state.current_chairmanship_id,
                "chairmanship.campaign.test.2"
            );
            assert_eq!(state.dossiers[1].chair_person_id, "person.successor");
        }
    }
    #[test]
    fn unlawful_succession_has_no_partial_effect() {
        let mut state = campaign();
        let before = state.clone();
        let error = state
            .succeed(
                SuccessionCause::Resignation,
                "rule.resignation",
                "person.ineligible".into(),
                program("successor"),
                "2006-02-01T00:00:00-05:00".into(),
                Vec::new(),
            )
            .unwrap_err();
        assert_eq!(
            error,
            "lawful successor is missing, ineligible, or ambiguous"
        );
        assert_eq!(state, before);
    }

    #[test]
    fn program_revision_is_prospective_and_does_not_score() {
        let mut state = campaign();
        state
            .revise_program("program.chair", program("revised"))
            .unwrap();
        assert_eq!(state.dossiers[0].program.revision_id, "revised");
        assert_eq!(state.dossiers[0].program_history, vec![program("initial")]);
        assert!(state.ledger.is_empty());
        assert!(
            state
                .revise_program("program.chair", program("revised"))
                .is_err()
        );
    }

    #[test]
    fn revision_response_and_supplemental_dispositions_preserve_history() {
        let mut revised = campaign();
        revised
            .dispose_review("review.final", 1, ReviewDisposition::RequestRevision, None)
            .unwrap();
        let review = &revised.reviews["review.final"];
        assert_eq!(review.review_version, 2);
        assert_eq!(
            review.disposition_history,
            vec![ReviewDisposition::RequestRevision]
        );
        assert!(review.disposed.is_none());

        let mut responded = campaign();
        responded
            .dispose_review(
                "review.final",
                1,
                ReviewDisposition::AcceptWithChairResponse,
                None,
            )
            .unwrap();
        assert_eq!(
            responded.reviews["review.final"].disposed,
            Some(ReviewDisposition::AcceptWithChairResponse)
        );

        let mut supplemented = campaign();
        supplemented
            .dispose_review(
                "review.final",
                1,
                ReviewDisposition::AcceptWithSupplementalReview,
                Some("review.supplemental".into()),
            )
            .unwrap();
        supplemented
            .commission_supplemental("review.final", 1, "review.supplemental".into())
            .unwrap();
        supplemented.reach_endpoint();
        assert!(!supplemented.terminal);
        supplemented
            .dispose_review("review.supplemental", 1, ReviewDisposition::Accept, None)
            .unwrap();
        supplemented.finalize_if_ready();
        assert!(supplemented.terminal);
    }

    #[test]
    fn compensation_appends_without_rewriting_the_award() {
        let mut state = campaign();
        state
            .dispose_review("review.final", 1, ReviewDisposition::Accept, None)
            .unwrap();
        state
            .record_award(
                "review.final",
                1,
                "finding.test",
                4,
                vec!["witness.1".into()],
            )
            .unwrap();
        let award = state.ledger[0].clone();
        state
            .compensate(
                &award.entry_id,
                "review.corrected",
                2,
                vec!["witness.2".into()],
                -3,
            )
            .unwrap();
        assert_eq!(state.ledger[0], award);
        assert_eq!(state.ledger[1].compensates_entry_id, Some(award.entry_id));
        assert_eq!(state.total().unwrap(), 1);
    }
}
