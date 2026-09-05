use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub(crate) enum AccountingError {
    #[error("{0}")]
    Domain(String),
}
fn err(message: impl Into<String>) -> AccountingError {
    AccountingError::Domain(message.into())
}
pub(crate) fn amount(value: &Value) -> Result<Decimal, AccountingError> {
    Decimal::from_str_exact(
        value
            .as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| value.to_string())
            .as_str(),
    )
    .map_err(|_| err(format!("invalid decimal: {value}")))
}
fn ds(value: Decimal) -> String {
    value.to_string()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Account {
    pub account_id: String,
    pub owner_id: String,
    pub instrument: String,
    pub unit: String,
    pub balance: Decimal,
    pub account_kind: String,
    #[serde(default)]
    pub allow_negative: bool,
    #[serde(default)]
    pub version: u64,
    #[serde(default)]
    pub reserved: Decimal,
}
impl Account {
    pub(crate) fn from_opening_state(row: &Value) -> Result<Self, AccountingError> {
        let value = &row["value"];
        Ok(Self {
            account_id: reqs(row, "state_id")?,
            owner_id: reqs(row, "owner_id")?,
            instrument: reqs(value, "instrument")?,
            unit: reqs(row, "unit")?,
            balance: amount(&value["balance"])?,
            account_kind: reqs(value, "account_kind")?,
            allow_negative: value["allow_negative"].as_bool().unwrap_or(false),
            version: 0,
            reserved: Decimal::ZERO,
        })
    }
    pub(crate) fn available(&self) -> Decimal {
        self.balance - self.reserved
    }
    pub(crate) fn to_dict(&self) -> Value {
        json!({"account_id":self.account_id,"account_kind":self.account_kind,"allow_negative":self.allow_negative,"balance":ds(self.balance),"instrument":self.instrument,"owner_id":self.owner_id,"reserved":ds(self.reserved),"unit":self.unit,"version":self.version})
    }
}
fn reqs(value: &Value, key: &str) -> Result<String, AccountingError> {
    value[key]
        .as_str()
        .map(Into::into)
        .ok_or_else(|| err(format!("missing string {key}")))
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct LedgerEntry {
    pub account_id: String,
    pub delta: Decimal,
    pub instrument: String,
    pub unit: String,
}
impl LedgerEntry {
    pub(crate) fn to_dict(&self) -> Value {
        json!({"account_id":self.account_id,"delta":ds(self.delta),"instrument":self.instrument,"unit":self.unit})
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AccountingTransaction {
    pub transaction_id: String,
    pub committed_at: String,
    pub entries: Vec<LedgerEntry>,
}
impl AccountingTransaction {
    pub(crate) fn to_dict(&self) -> Value {
        json!({"committed_at":self.committed_at,"entries":self.entries.iter().map(LedgerEntry::to_dict).collect::<Vec<_>>(),"transaction_id":self.transaction_id})
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg(test)]
pub(crate) struct BalanceSheet {
    pub owner_id: String,
    pub assets: Decimal,
    pub liabilities: Decimal,
    pub equity: Decimal,
    pub collateral_control: Decimal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AccountingLedger {
    accounts: BTreeMap<String, Account>,
    reservations: BTreeMap<String, (String, Decimal)>,
    transactions: Vec<AccountingTransaction>,
    #[serde(with = "totals_pairs")]
    opening_totals: BTreeMap<(String, String), Decimal>,
}
impl AccountingLedger {
    pub(crate) fn new(
        accounts: impl IntoIterator<Item = Account>,
    ) -> Result<Self, AccountingError> {
        let mut indexed = BTreeMap::new();
        for account in accounts {
            if indexed
                .insert(account.account_id.clone(), account)
                .is_some()
            {
                return Err(err("duplicate account identifier"));
            }
        }
        let opening_totals = totals(&indexed);
        Ok(Self {
            accounts: indexed,
            reservations: BTreeMap::new(),
            transactions: vec![],
            opening_totals,
        })
    }
    pub(crate) fn from_opening_state(rows: &[Value]) -> Result<Self, AccountingError> {
        Self::new(
            rows.iter()
                .filter(|r| r["value"]["storage"] == "accounting_ledger")
                .map(Account::from_opening_state)
                .collect::<Result<Vec<_>, _>>()?,
        )
    }
    #[cfg(test)]
    pub(crate) fn transactions(&self) -> &[AccountingTransaction] {
        &self.transactions
    }
    #[cfg(test)]
    pub(crate) fn reservations(&self) -> BTreeMap<String, (String, Decimal)> {
        self.reservations.clone()
    }
    pub(crate) fn account(&self, id: &str) -> Result<&Account, AccountingError> {
        self.accounts
            .get(id)
            .ok_or_else(|| err(format!("unknown account: {id}")))
    }
    pub(crate) fn account_mut(&mut self, id: &str) -> Result<&mut Account, AccountingError> {
        self.accounts
            .get_mut(id)
            .ok_or_else(|| err(format!("unknown account: {id}")))
    }
    pub(crate) fn balance(&self, id: &str) -> Result<Decimal, AccountingError> {
        Ok(self.account(id)?.balance)
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({"accounts":self.accounts.values().map(Account::to_dict).collect::<Vec<_>>(),"reservations":self.reservations.iter().map(|(k,(a,v))|(k.clone(),json!({"account_id":a,"amount":ds(*v)}))).collect::<serde_json::Map<_,_>>(),"transactions":self.transactions.iter().map(AccountingTransaction::to_dict).collect::<Vec<_>>()})
    }
    pub(crate) fn reserve(
        &mut self,
        reservation_id: &str,
        account_id: &str,
        requested: Decimal,
    ) -> Result<(), AccountingError> {
        if self.reservations.contains_key(reservation_id) {
            return Err(err(format!("duplicate reservation: {reservation_id}")));
        }
        if requested <= Decimal::ZERO {
            return Err(err("reservation amount must be positive"));
        }
        let account = self.account_mut(account_id)?;
        let available = account.balance - account.reserved;
        if !account.allow_negative && available < requested {
            return Err(err(format!(
                "insufficient available balance in {account_id}: requested={requested} available={available}"
            )));
        }
        account.reserved += requested;
        self.reservations
            .insert(reservation_id.into(), (account_id.into(), requested));
        Ok(())
    }
    pub(crate) fn release(&mut self, id: &str) -> Result<(), AccountingError> {
        let Some((account_id, value)) = self.reservations.remove(id) else {
            return Ok(());
        };
        let account = self.account_mut(&account_id)?;
        account.reserved -= value;
        if account.reserved < Decimal::ZERO {
            return Err(err(format!("negative reservation balance in {account_id}")));
        }
        Ok(())
    }
    pub(crate) fn commit(
        &mut self,
        transaction_id: &str,
        entries: &[LedgerEntry],
        expected_versions: &BTreeMap<String, u64>,
        reservation_ids: &[String],
        committed_at: &str,
    ) -> Result<AccountingTransaction, AccountingError> {
        if self
            .transactions
            .iter()
            .any(|x| x.transaction_id == transaction_id)
        {
            return Err(err(format!("duplicate transaction: {transaction_id}")));
        }
        if entries.is_empty() {
            return Err(err("accounting transaction contains no entries"));
        }
        let mut totals = BTreeMap::new();
        let mut deltas = BTreeMap::new();
        for entry in entries {
            let a = self.account(&entry.account_id)?;
            if (entry.instrument.as_str(), entry.unit.as_str())
                != (a.instrument.as_str(), a.unit.as_str())
            {
                return Err(err(format!(
                    "entry contract mismatch for {}",
                    entry.account_id
                )));
            }
            *totals
                .entry((entry.instrument.clone(), entry.unit.clone()))
                .or_insert(Decimal::ZERO) += entry.delta;
            *deltas
                .entry(entry.account_id.clone())
                .or_insert(Decimal::ZERO) += entry.delta;
        }
        if totals.values().any(|x| !x.is_zero()) {
            return Err(err("unbalanced accounting transaction"));
        }
        if expected_versions.keys().collect::<BTreeSet<_>>()
            != deltas.keys().collect::<BTreeSet<_>>()
        {
            return Err(err("expected versions do not cover every touched account"));
        }
        for (id, version) in expected_versions {
            if self.account(id)?.version != *version {
                return Err(err(format!("stale account version: {id}")));
            }
        }
        for (id, delta) in &deltas {
            let a = self.account(id)?;
            if !a.allow_negative && a.balance + *delta < Decimal::ZERO {
                return Err(err(format!("transaction overdraws {id}")));
            }
        }
        for id in reservation_ids {
            self.release(id)?;
        }
        for (id, delta) in deltas {
            let a = self.account_mut(&id)?;
            a.balance += delta;
            a.version += 1;
        }
        let tx = AccountingTransaction {
            transaction_id: transaction_id.into(),
            committed_at: committed_at.into(),
            entries: entries.into(),
        };
        self.transactions.push(tx.clone());
        self.assert_conserved()?;
        Ok(tx)
    }
    pub(crate) fn conserved_totals(&self) -> BTreeMap<(String, String), Decimal> {
        totals(&self.accounts)
    }
    pub(crate) fn assert_conserved(&self) -> Result<(), AccountingError> {
        if self.conserved_totals() != self.opening_totals {
            Err(err("ledger conservation differs from opening totals"))
        } else {
            Ok(())
        }
    }
    #[cfg(test)]
    pub(crate) fn balance_sheet(&self, owner: &str, price: Decimal) -> BalanceSheet {
        let (mut assets, mut liabilities, mut collateral) =
            (Decimal::ZERO, Decimal::ZERO, Decimal::ZERO);
        for a in self.accounts.values().filter(|a| a.owner_id == owner) {
            if a.account_kind == "collateral_control" {
                collateral += a.balance;
                continue;
            }
            let value = if a.instrument == "TREASURY_5_10Y" {
                a.balance * price
            } else {
                a.balance
            };
            if a.account_kind == "signed_claim" && value < Decimal::ZERO {
                liabilities -= value
            } else {
                assets += value
            }
        }
        BalanceSheet {
            owner_id: owner.into(),
            assets,
            liabilities,
            equity: assets - liabilities,
            collateral_control: collateral,
        }
    }
}
fn totals(accounts: &BTreeMap<String, Account>) -> BTreeMap<(String, String), Decimal> {
    let mut r = BTreeMap::new();
    for a in accounts.values() {
        *r.entry((a.instrument.clone(), a.unit.clone()))
            .or_insert(Decimal::ZERO) += a.balance;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    fn account(account_id: &str, instrument: &str, balance: &str, account_kind: &str) -> Account {
        Account {
            account_id: account_id.into(),
            owner_id: "buyer".into(),
            instrument: instrument.into(),
            unit: "test_unit".into(),
            balance: Decimal::from_str_exact(balance).unwrap(),
            account_kind: account_kind.into(),
            allow_negative: account_kind == "signed_claim",
            version: 0,
            reserved: Decimal::ZERO,
        }
    }

    #[test]
    fn balance_sheet_marks_treasuries_and_separates_liabilities_and_collateral() {
        let ledger = AccountingLedger::new([
            account("buyer.cash", "USD_CASH", "5", "asset"),
            account("buyer.treasury", "TREASURY_5_10Y", "4", "asset"),
            account("buyer.claim", "USD_CASH", "-3", "signed_claim"),
            account(
                "buyer.collateral",
                "TREASURY_5_10Y",
                "2",
                "collateral_control",
            ),
        ])
        .unwrap();

        let sheet = ledger.balance_sheet("buyer", Decimal::from_str_exact("1.25").unwrap());

        assert_eq!(sheet.owner_id, "buyer");
        assert_eq!(sheet.assets, Decimal::from_str_exact("10").unwrap());
        assert_eq!(sheet.liabilities, Decimal::from_str_exact("3").unwrap());
        assert_eq!(sheet.equity, Decimal::from_str_exact("7").unwrap());
        assert_eq!(
            sheet.collateral_control,
            Decimal::from_str_exact("2").unwrap()
        );
    }
}

mod totals_pairs {
    use super::*;
    use serde::{Deserializer, Serializer, de::Error as _};

    pub(super) fn serialize<S: Serializer>(
        totals: &BTreeMap<(String, String), Decimal>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(totals.iter())
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<BTreeMap<(String, String), Decimal>, D::Error> {
        let rows = Vec::<((String, String), Decimal)>::deserialize(deserializer)?;
        let mut totals = BTreeMap::new();
        for (key, value) in rows {
            if totals.insert(key, value).is_some() {
                return Err(D::Error::custom("duplicate instrument/unit opening total"));
            }
        }
        Ok(totals)
    }
}
