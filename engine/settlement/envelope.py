from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal
from enum import StrEnum
from typing import Any, Iterable

from engine.accounting.ledger import AccountingError, AccountingLedger, LedgerEntry
from engine.markets.treasury_secondary import TreasuryFill
from engine.witness import WitnessLedger


class SettlementStatus(StrEnum):
    PREPARED = "PREPARED"
    COMMITTED = "COMMITTED"
    FAILED_PREPARE = "FAILED_PREPARE"
    FAILED_COMMIT = "FAILED_COMMIT"


@dataclass(frozen=True)
class SettlementResult:
    envelope_id: str
    status: SettlementStatus
    transaction_id: str | None
    failure_reason: str | None
    reservation_ids: tuple[str, ...]

    def to_dict(self) -> dict[str, Any]:
        return {
            "envelope_id": self.envelope_id,
            "failure_reason": self.failure_reason,
            "reservation_ids": list(self.reservation_ids),
            "status": self.status.value,
            "transaction_id": self.transaction_id,
        }


class SettlementEnvelope:
    def __init__(
        self,
        envelope_id: str,
        entries: Iterable[LedgerEntry],
        effective_time: str,
        causal_parent: str | None = None,
        responsible_owner: str = "market.us.treasury.secondary",
    ) -> None:
        self.envelope_id = envelope_id
        self.entries = tuple(entries)
        self.effective_time = effective_time
        self.causal_parent = causal_parent
        self.responsible_owner = responsible_owner
        self._expected_versions: dict[str, int] = {}
        self._reservation_ids: list[str] = []
        self._prepared = False
        self._finished = False

    @classmethod
    def for_treasury_fills(
        cls,
        envelope_id: str,
        fills: Iterable[TreasuryFill],
        account_map: dict[str, dict[str, str]],
        effective_time: str,
        causal_parent: str | None = None,
    ) -> "SettlementEnvelope":
        entries: list[LedgerEntry] = []
        for fill in fills:
            buyer = account_map[fill.buyer_id]
            seller = account_map[fill.seller_id]
            entries.extend(
                (
                    LedgerEntry(buyer["cash"], -fill.cash_amount, "USD_CASH", "USD"),
                    LedgerEntry(seller["cash"], fill.cash_amount, "USD_CASH", "USD"),
                    LedgerEntry(
                        seller["treasury"],
                        -fill.quantity,
                        "TREASURY_5_10Y",
                        "treasury_face",
                    ),
                    LedgerEntry(
                        buyer["treasury"],
                        fill.quantity,
                        "TREASURY_5_10Y",
                        "treasury_face",
                    ),
                )
            )
        return cls(envelope_id, entries, effective_time, causal_parent)

    def prepare(
        self,
        ledger: AccountingLedger,
        witness_ledger: WitnessLedger | None = None,
    ) -> SettlementResult:
        if self._prepared or self._finished:
            raise AccountingError(f"envelope cannot be prepared in its current state: {self.envelope_id}")
        touched = sorted({entry.account_id for entry in self.entries})
        self._expected_versions = {
            account_id: ledger.account(account_id).version for account_id in touched
        }
        outgoing: dict[str, Decimal] = {}
        for entry in self.entries:
            if entry.delta < 0:
                outgoing[entry.account_id] = outgoing.get(entry.account_id, Decimal("0")) - entry.delta
        try:
            for sequence, account_id in enumerate(sorted(outgoing), start=1):
                reservation_id = f"reservation.{self.envelope_id}.{sequence:04d}"
                ledger.reserve(reservation_id, account_id, outgoing[account_id])
                self._reservation_ids.append(reservation_id)
        except AccountingError as exc:
            self._release_all(ledger)
            self._finished = True
            result = SettlementResult(
                self.envelope_id,
                SettlementStatus.FAILED_PREPARE,
                None,
                str(exc),
                (),
            )
            self._witness(witness_ledger, "settlement_prepare_failed", result)
            return result
        self._prepared = True
        result = SettlementResult(
            self.envelope_id,
            SettlementStatus.PREPARED,
            None,
            None,
            tuple(self._reservation_ids),
        )
        self._witness(witness_ledger, "settlement_envelope_prepared", result)
        return result

    def commit(
        self,
        ledger: AccountingLedger,
        witness_ledger: WitnessLedger | None = None,
    ) -> SettlementResult:
        if not self._prepared or self._finished:
            raise AccountingError(f"envelope is not prepared: {self.envelope_id}")
        transaction_id = f"transaction.{self.envelope_id}"
        try:
            ledger.commit(
                transaction_id,
                self.entries,
                self._expected_versions,
                tuple(self._reservation_ids),
                self.effective_time,
                witness_ledger,
                self.causal_parent,
                self.responsible_owner,
            )
        except AccountingError as exc:
            self._release_all(ledger)
            self._finished = True
            result = SettlementResult(
                self.envelope_id,
                SettlementStatus.FAILED_COMMIT,
                None,
                str(exc),
                (),
            )
            self._witness(witness_ledger, "settlement_commit_failed", result)
            return result
        self._reservation_ids.clear()
        self._finished = True
        result = SettlementResult(
            self.envelope_id,
            SettlementStatus.COMMITTED,
            transaction_id,
            None,
            (),
        )
        self._witness(witness_ledger, "settlement_envelope_committed", result)
        return result

    def _release_all(self, ledger: AccountingLedger) -> None:
        for reservation_id in reversed(self._reservation_ids):
            ledger.release(reservation_id)
        self._reservation_ids.clear()

    def _witness(
        self,
        ledger: WitnessLedger | None,
        transition_kind: str,
        result: SettlementResult,
    ) -> None:
        if ledger is not None:
            ledger.append(
                completion_time=self.effective_time,
                transition_kind=transition_kind,
                responsible_owner=self.responsible_owner,
                causal_parent=self.causal_parent,
                payload={"settlement": result.to_dict()},
            )
