from __future__ import annotations

from engine.authority import ActionResult, ActionStatus, Command, Directive
from engine.clock import parse_time
from engine.legal import LegalRegistry, LegalResolutionError


class DeskExecutor:
    OWNER_ID = "inst.us.federal_reserve.new_york"
    MARKET_ID = "market.us.treasury.secondary"

    def __init__(self, legal: LegalRegistry) -> None:
        self._legal = legal

    def execute(
        self,
        directive: Directive,
        requested_effect: str,
        at_time: str,
    ) -> ActionResult:
        command_id = f"command.desk.{requested_effect.removeprefix('desk.')}"
        if directive.target_owner != self.OWNER_ID or requested_effect not in directive.authorized_effects:
            return ActionResult(
                result_id=f"result.{command_id}",
                command_id=command_id,
                responsible_owner=self.OWNER_ID,
                status=ActionStatus.REJECTED_OUTSIDE_DIRECTIVE,
                realized_effect=None,
                failure_stage="execution_preflight",
                reason="The requested leg is outside the certified directive.",
            )
        if not (
            parse_time(directive.effective_time)
            <= parse_time(at_time)
            < parse_time(directive.expiry_time)
        ):
            return ActionResult(
                result_id=f"result.{command_id}",
                command_id=command_id,
                responsible_owner=self.OWNER_ID,
                status=ActionStatus.REJECTED_OUTSIDE_DIRECTIVE,
                realized_effect=None,
                failure_stage="execution_preflight",
                reason="The certified directive is not effective at the requested execution time.",
            )
        required_refs = {
            "clause.fra.12a.fomc_direction",
            "clause.fomc.rules.section3.vote",
            "clause.fra.14.reserve_bank_open_market_power",
            "clause.domestic_authorization.2006.paragraph4",
        }
        if (
            directive.issuing_body != "body.us.federal_reserve.fomc"
            or not required_refs.issubset(directive.authority_refs)
        ):
            return ActionResult(
                result_id=f"result.{command_id}",
                command_id=command_id,
                responsible_owner=self.OWNER_ID,
                status=ActionStatus.REJECTED_NO_APPLICABLE_DELEGATION,
                realized_effect=None,
                failure_stage="execution_preflight",
                reason="The directive lacks the required FOMC authority and Desk delegation chain.",
            )
        try:
            reserve_bank_power = self._legal.clause(
                "clause.fra.14.reserve_bank_open_market_power", at_time
            )
            delegation = self._legal.clause(
                "clause.domestic_authorization.2006.paragraph4", at_time
            )
        except LegalResolutionError as exc:
            return ActionResult(
                result_id=f"result.{command_id}",
                command_id=command_id,
                responsible_owner=self.OWNER_ID,
                status=ActionStatus.REJECTED_NO_APPLICABLE_DELEGATION,
                realized_effect=None,
                failure_stage="execution_preflight",
                reason=str(exc),
            )
        legal_legs = (reserve_bank_power, delegation)
        if not all(
            clause.permits(
                requesting_subject=self.OWNER_ID,
                target_owner=self.MARKET_ID,
                proposed_effect=requested_effect,
                at_time=at_time,
                has_certified_fomc_decision=True,
            )
            for clause in legal_legs
        ):
            return ActionResult(
                result_id=f"result.{command_id}",
                command_id=command_id,
                responsible_owner=self.OWNER_ID,
                status=ActionStatus.REJECTED_NO_APPLICABLE_DELEGATION,
                realized_effect=None,
                failure_stage="execution_preflight",
                reason="The 2006 domestic authorization does not cover the requested leg.",
            )
        return ActionResult(
            result_id=f"result.{command_id}",
            command_id=command_id,
            responsible_owner=self.OWNER_ID,
            status=ActionStatus.EXECUTED,
            realized_effect=requested_effect,
            failure_stage=None,
            reason="The Desk submitted the authorized operation to the Treasury market.",
            witness_refs=(directive.directive_id,),
        )

    @staticmethod
    def chair_only_command(at_time: str) -> Command:
        return Command(
            command_id="chair_only_market_command",
            requesting_office="office.us.federal_reserve.fomc_chair",
            target_owner=DeskExecutor.OWNER_ID,
            proposed_effect="desk.raise_target_range_25bp",
            claimed_authority="clause.fra.12a.fomc_direction",
            requested_effective_time=at_time,
            source_record_id="record.unsanctioned.chair_command",
        )
