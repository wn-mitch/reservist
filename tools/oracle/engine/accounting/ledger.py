from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from decimal import Decimal
from typing import Any, Iterable

from engine.witness import WitnessLedger


class AccountingError(ValueError):
    pass


def amount(value: Decimal | int | str) -> Decimal:
    return value if isinstance(value, Decimal) else Decimal(str(value))


@dataclass
class Account:
    account_id: str
    owner_id: str
    instrument: str
    unit: str
    balance: Decimal
    account_kind: str
    allow_negative: bool = False
    version: int = 0
    reserved: Decimal = Decimal("0")

    @classmethod
    def from_opening_state(cls, row: dict[str, Any]) -> "Account":
        value = row["value"]
        return cls(
            account_id=row["state_id"],
            owner_id=row["owner_id"],
            instrument=value["instrument"],
            unit=row["unit"],
            balance=amount(value["balance"]),
            account_kind=value["account_kind"],
            allow_negative=bool(value.get("allow_negative", False)),
        )

    @property
    def available(self) -> Decimal:
        return self.balance - self.reserved

    def to_dict(self) -> dict[str, Any]:
        return {
            "account_id": self.account_id,
            "account_kind": self.account_kind,
            "allow_negative": self.allow_negative,
            "balance": str(self.balance),
            "instrument": self.instrument,
            "owner_id": self.owner_id,
            "reserved": str(self.reserved),
            "unit": self.unit,
            "version": self.version,
        }


@dataclass(frozen=True)
class LedgerEntry:
    account_id: str
    delta: Decimal
    instrument: str
    unit: str

    def to_dict(self) -> dict[str, str]:
        return {
            "account_id": self.account_id,
            "delta": str(self.delta),
            "instrument": self.instrument,
            "unit": self.unit,
        }


@dataclass(frozen=True)
class AccountingTransaction:
    transaction_id: str
    committed_at: str
    entries: tuple[LedgerEntry, ...]

    def to_dict(self) -> dict[str, Any]:
        return {
            "committed_at": self.committed_at,
            "entries": [entry.to_dict() for entry in self.entries],
            "transaction_id": self.transaction_id,
        }


@dataclass(frozen=True)
class BalanceSheet:
    owner_id: str
    assets: Decimal
    liabilities: Decimal
    equity: Decimal
    collateral_control: Decimal


class AccountingLedger:
    def __init__(self, accounts: Iterable[Account]) -> None:
        account_rows = list(accounts)
        self._accounts = {account.account_id: account for account in account_rows}
        if len(self._accounts) != len(account_rows):
            raise AccountingError("duplicate account identifier")
        self._reservations: dict[str, tuple[str, Decimal]] = {}
        self._transactions: list[AccountingTransaction] = []
        self._opening_totals = self.conserved_totals()

    @classmethod
    def from_opening_state(cls, rows: Iterable[dict[str, Any]]) -> "AccountingLedger":
        return cls(
            Account.from_opening_state(row)
            for row in rows
            if row.get("value", {}).get("storage") == "accounting_ledger"
        )

    @property
    def transactions(self) -> tuple[AccountingTransaction, ...]:
        return tuple(self._transactions)

    @property
    def reservations(self) -> dict[str, tuple[str, Decimal]]:
        return dict(self._reservations)

    def account(self, account_id: str) -> Account:
        try:
            return self._accounts[account_id]
        except KeyError as exc:
            raise AccountingError(f"unknown account: {account_id}") from exc

    def balance(self, account_id: str) -> Decimal:
        return self.account(account_id).balance

    def reserve(self, reservation_id: str, account_id: str, value: Decimal) -> None:
        if reservation_id in self._reservations:
            raise AccountingError(f"duplicate reservation: {reservation_id}")
        requested = amount(value)
        if requested <= 0:
            raise AccountingError("reservation amount must be positive")
        account = self.account(account_id)
        if not account.allow_negative and account.available < requested:
            raise AccountingError(
                f"insufficient available balance in {account_id}: "
                f"requested={requested} available={account.available}"
            )
        account.reserved += requested
        self._reservations[reservation_id] = (account_id, requested)

    def release(self, reservation_id: str) -> None:
        reserved = self._reservations.pop(reservation_id, None)
        if reserved is None:
            return
        account_id, value = reserved
        account = self.account(account_id)
        account.reserved -= value
        if account.reserved < 0:
            raise AccountingError(f"negative reservation balance in {account_id}")

    def commit(
        self,
        transaction_id: str,
        entries: Iterable[LedgerEntry],
        expected_versions: dict[str, int],
        reservation_ids: Iterable[str],
        committed_at: str,
        witness_ledger: WitnessLedger | None = None,
        causal_parent: str | None = None,
        responsible_owner: str = "accounting.ledger",
    ) -> AccountingTransaction:
        if any(row.transaction_id == transaction_id for row in self._transactions):
            raise AccountingError(f"duplicate transaction: {transaction_id}")
        rows = tuple(entries)
        if not rows:
            raise AccountingError("accounting transaction contains no entries")

        totals: dict[tuple[str, str], Decimal] = defaultdict(lambda: Decimal("0"))
        deltas: dict[str, Decimal] = defaultdict(lambda: Decimal("0"))
        for entry in rows:
            account = self.account(entry.account_id)
            if (entry.instrument, entry.unit) != (account.instrument, account.unit):
                raise AccountingError(f"entry contract mismatch for {entry.account_id}")
            totals[(entry.instrument, entry.unit)] += entry.delta
            deltas[entry.account_id] += entry.delta
        unbalanced = {key: value for key, value in totals.items() if value != 0}
        if unbalanced:
            raise AccountingError(f"unbalanced accounting transaction: {unbalanced}")
        if set(expected_versions) != set(deltas):
            raise AccountingError("expected versions do not cover every touched account")
        for account_id, expected in expected_versions.items():
            if self.account(account_id).version != expected:
                raise AccountingError(f"stale account version: {account_id}")
        for account_id, delta in deltas.items():
            account = self.account(account_id)
            if not account.allow_negative and account.balance + delta < 0:
                raise AccountingError(f"transaction overdraws {account_id}")

        for reservation_id in reservation_ids:
            self.release(reservation_id)
        for account_id in sorted(deltas):
            account = self.account(account_id)
            account.balance += deltas[account_id]
            account.version += 1
        transaction = AccountingTransaction(transaction_id, committed_at, rows)
        self._transactions.append(transaction)
        if witness_ledger is not None:
            witness_ledger.append(
                completion_time=committed_at,
                transition_kind="accounting_transaction_committed",
                responsible_owner=responsible_owner,
                causal_parent=causal_parent,
                payload={"transaction": transaction.to_dict()},
            )
        self.assert_conserved()
        return transaction

    def conserved_totals(self) -> dict[tuple[str, str], Decimal]:
        totals: dict[tuple[str, str], Decimal] = defaultdict(lambda: Decimal("0"))
        for account in self._accounts.values():
            totals[(account.instrument, account.unit)] += account.balance
        return dict(totals)

    def assert_conserved(self) -> None:
        if self.conserved_totals() != self._opening_totals:
            raise AccountingError("ledger conservation differs from opening totals")

    def balance_sheet(self, owner_id: str, treasury_price: Decimal) -> BalanceSheet:
        assets = Decimal("0")
        liabilities = Decimal("0")
        collateral = Decimal("0")
        for account in self._accounts.values():
            if account.owner_id != owner_id:
                continue
            if account.account_kind == "collateral_control":
                collateral += account.balance
                continue
            value = account.balance
            if account.instrument == "TREASURY_5_10Y":
                value *= treasury_price
            if account.account_kind == "signed_claim" and value < 0:
                liabilities += -value
            else:
                assets += value
        return BalanceSheet(owner_id, assets, liabilities, assets - liabilities, collateral)

    def snapshot_for_hash(self) -> dict[str, Any]:
        return {
            "accounts": [self._accounts[key].to_dict() for key in sorted(self._accounts)],
            "reservations": {
                key: {"account_id": value[0], "amount": str(value[1])}
                for key, value in sorted(self._reservations.items())
            },
            "transactions": [row.to_dict() for row in self._transactions],
        }
