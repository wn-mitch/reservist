use crate::{
    authority::{ActionResult, ActionStatus, Command, Directive},
    legal::LegalRegistry,
    time::Instant,
};

pub(crate) struct DeskExecutor;
impl DeskExecutor {
    pub(crate) const OWNER_ID: &'static str = "inst.us.federal_reserve.new_york";
    pub(crate) const MARKET_ID: &'static str = "market.us.treasury.secondary";

    /// Executes one directive leg against `market_id`. The directive's authority
    /// references beyond the FOMC decision clauses form its delegation chain,
    /// which must include section 14 and at least one domestic authorization.
    pub(crate) fn execute(
        legal: &LegalRegistry,
        directive: &Directive,
        requested_effect: &str,
        at_time: &str,
        market_id: &str,
    ) -> ActionResult {
        let command_id = format!(
            "command.desk.{}",
            requested_effect
                .strip_prefix("desk.")
                .unwrap_or(requested_effect)
        );
        let reject = |status, reason: String| ActionResult {
            result_id: format!("result.{command_id}"),
            command_id: command_id.clone(),
            responsible_owner: Self::OWNER_ID.into(),
            status,
            realized_effect: None,
            failure_stage: Some("execution_preflight".into()),
            reason,
            witness_refs: Vec::new(),
        };
        if directive.target_owner != Self::OWNER_ID
            || !directive
                .authorized_effects
                .iter()
                .any(|effect| effect == requested_effect)
        {
            return reject(
                ActionStatus::RejectedOutsideDirective,
                "The requested leg is outside the certified directive.".into(),
            );
        }
        let effective = match (
            Instant::parse(&directive.effective_time),
            Instant::parse(at_time),
            Instant::parse(&directive.expiry_time),
        ) {
            (Ok(start), Ok(at), Ok(end)) => start <= at && at < end,
            _ => false,
        };
        if !effective {
            return reject(
                ActionStatus::RejectedOutsideDirective,
                "The certified directive is not effective at the requested execution time.".into(),
            );
        }
        const DECISION: [&str; 2] = [
            "clause.fra.12a.fomc_direction",
            "clause.fomc.rules.section3.vote",
        ];
        const SECTION_14: &str = "clause.fra.14.reserve_bank_open_market_power";
        let delegation = directive
            .authority_refs
            .iter()
            .filter(|reference| !DECISION.contains(&reference.as_str()))
            .collect::<Vec<_>>();
        if directive.issuing_body != "body.us.federal_reserve.fomc"
            || !DECISION.iter().all(|id| {
                directive
                    .authority_refs
                    .iter()
                    .any(|reference| reference == id)
            })
            || !delegation.iter().any(|reference| *reference == SECTION_14)
            || delegation.len() < 2
        {
            return reject(
                ActionStatus::RejectedNoApplicableDelegation,
                "The directive lacks the required FOMC authority and Desk delegation chain.".into(),
            );
        }
        let mut clauses = Vec::with_capacity(delegation.len());
        for id in &delegation {
            match legal.clause(id, at_time) {
                Ok(clause) => clauses.push(clause),
                Err(error) => {
                    return reject(
                        ActionStatus::RejectedNoApplicableDelegation,
                        error.to_string(),
                    );
                }
            }
        }
        let treasury = market_id == Self::MARKET_ID;
        if !clauses.iter().all(|clause| {
            clause.permits(Self::OWNER_ID, market_id, requested_effect, at_time, true)
        }) {
            return reject(
                ActionStatus::RejectedNoApplicableDelegation,
                if treasury {
                    "The 2006 domestic authorization does not cover the requested leg.".into()
                } else {
                    "The domestic authorization does not cover the requested leg.".into()
                },
            );
        }
        ActionResult {
            result_id: format!("result.{command_id}"),
            command_id,
            responsible_owner: Self::OWNER_ID.into(),
            status: ActionStatus::Executed,
            realized_effect: Some(requested_effect.into()),
            failure_stage: None,
            reason: if treasury {
                "The Desk submitted the authorized operation to the Treasury market.".into()
            } else {
                "The Desk adopted the authorized operating regime in the reserves market.".into()
            },
            witness_refs: vec![directive.directive_id.clone()],
        }
    }

    pub(crate) fn chair_only_command(at_time: &str) -> Command {
        Command {
            command_id: "chair_only_market_command".into(),
            requesting_office: "office.us.federal_reserve.fomc_chair".into(),
            target_owner: Self::OWNER_ID.into(),
            proposed_effect: "desk.raise_target_range_25bp".into(),
            claimed_authority: "clause.fra.12a.fomc_direction".into(),
            requested_effective_time: at_time.into(),
            source_record_id: "record.unsanctioned.chair_command".into(),
        }
    }
}
