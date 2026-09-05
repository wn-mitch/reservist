from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal
from enum import StrEnum
from typing import Any

from engine.accounting.ledger import AccountingLedger, LedgerEntry, amount
from engine.clock import parse_time
from engine.settlement.envelope import SettlementEnvelope
from engine.witness import WitnessLedger


class RepoStatus(StrEnum):
    ACTIVE = "ACTIVE"
    NON_ROLL_PENDING = "NON_ROLL_PENDING"
    SETTLED = "SETTLED"


@dataclass(frozen=True)
class RepoMaturityResult:
    agreement_id: str
    status: str
    maturity_time: str
    liquidity_deficit: Decimal
    trigger_witness: str

    def to_dict(self) -> dict[str, str]:
        return {
            "agreement_id": self.agreement_id,
            "liquidity_deficit": str(self.liquidity_deficit),
            "maturity_time": self.maturity_time,
            "status": self.status,
            "trigger_witness": self.trigger_witness,
        }


class BilateralRepoAgreement:
    def __init__(self, agreement_id: str, value: dict[str, Any]) -> None:
        self.agreement_id = agreement_id
        self.lender_id = value["lender_id"]
        self.borrower_id = value["borrower_id"]
        self.principal = amount(value["principal"])
        self.collateral_quantity = amount(value["collateral_quantity"])
        self.haircut = amount(value["haircut"])
        self.maturity_time = value["maturity_time"]
        self.roll_policy = value["roll_policy"]
        self.accounts = value["accounts"]
        self.status = RepoStatus(value.get("status", RepoStatus.ACTIVE.value))
        self.non_roll_witness: str | None = None

    def process_non_roll(
        self,
        at_time: str,
        ledger: AccountingLedger,
        trigger_witness: str,
        witness_ledger: WitnessLedger | None = None,
    ) -> RepoMaturityResult:
        if self.status != RepoStatus.ACTIVE:
            raise ValueError("repo maturity may be processed only once")
        if parse_time(at_time) < parse_time(self.maturity_time):
            raise ValueError("repo non-roll cannot be processed before maturity")
        if self.roll_policy != "COUNTERPARTY_DECIDES_NO_AUTOMATIC_RENEWAL":
            raise ValueError("repo agreement lacks the no-automatic-renewal contract")
        available_cash = ledger.account(self.accounts["borrower_cash"]).available
        deficit = max(Decimal("0"), self.principal - available_cash)
        self.status = RepoStatus.NON_ROLL_PENDING
        self.non_roll_witness = trigger_witness
        result = RepoMaturityResult(
            agreement_id=self.agreement_id,
            status=("NON_ROLL_LIQUIDITY_DEFICIT" if deficit else "NON_ROLL_READY_TO_SETTLE"),
            maturity_time=at_time,
            liquidity_deficit=deficit,
            trigger_witness=trigger_witness,
        )
        if witness_ledger is not None:
            witness_ledger.append(
                completion_time=at_time,
                transition_kind="repo_non_roll_recorded",
                responsible_owner=self.agreement_id,
                causal_parent=trigger_witness,
                payload={"repo_maturity": result.to_dict()},
            )
        return result

    def settlement_envelope(
        self, at_time: str, causal_parent: str | None = None
    ) -> SettlementEnvelope:
        if self.status != RepoStatus.NON_ROLL_PENDING:
            raise ValueError("repo settlement requires a witnessed non-roll decision")
        entries = (
            LedgerEntry(self.accounts["borrower_cash"], -self.principal, "USD_CASH", "USD"),
            LedgerEntry(self.accounts["lender_cash"], self.principal, "USD_CASH", "USD"),
            LedgerEntry(self.accounts["lender_repo_claim"], -self.principal, "REPO_CLAIM", "USD"),
            LedgerEntry(self.accounts["borrower_repo_obligation"], self.principal, "REPO_CLAIM", "USD"),
            LedgerEntry(
                self.accounts["lender_collateral_control"],
                -self.collateral_quantity,
                "TREASURY_COLLATERAL_CONTROL",
                "treasury_face",
            ),
            LedgerEntry(
                self.accounts["borrower_collateral_encumbrance"],
                self.collateral_quantity,
                "TREASURY_COLLATERAL_CONTROL",
                "treasury_face",
            ),
        )
        return SettlementEnvelope(
            f"repo.{self.agreement_id}.maturity",
            entries,
            at_time,
            causal_parent,
            self.agreement_id,
        )

    def mark_settled(self) -> None:
        if self.status != RepoStatus.NON_ROLL_PENDING:
            raise ValueError("repo is not pending settlement")
        self.status = RepoStatus.SETTLED

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "accounts": dict(sorted(self.accounts.items())),
            "agreement_id": self.agreement_id,
            "borrower_id": self.borrower_id,
            "collateral_quantity": str(self.collateral_quantity),
            "haircut": str(self.haircut),
            "lender_id": self.lender_id,
            "maturity_time": self.maturity_time,
            "non_roll_witness": self.non_roll_witness,
            "principal": str(self.principal),
            "roll_policy": self.roll_policy,
            "status": self.status.value,
        }
