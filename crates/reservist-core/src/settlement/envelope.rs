use crate::{
    accounting::ledger::{AccountingError, AccountingLedger, LedgerEntry},
    markets::treasury_secondary::TreasuryFill,
    witness::WitnessLedger,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum SettlementStatus {
    Prepared,
    Committed,
    FailedPrepare,
    FailedCommit,
}
impl SettlementStatus {
    fn text(&self) -> &'static str {
        match self {
            Self::Prepared => "PREPARED",
            Self::Committed => "COMMITTED",
            Self::FailedPrepare => "FAILED_PREPARE",
            Self::FailedCommit => "FAILED_COMMIT",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct SettlementResult {
    pub envelope_id: String,
    pub status: SettlementStatus,
    pub transaction_id: Option<String>,
    pub failure_reason: Option<String>,
    pub reservation_ids: Vec<String>,
}
impl SettlementResult {
    pub(crate) fn to_dict(&self) -> Value {
        json!({"envelope_id":self.envelope_id,"failure_reason":self.failure_reason,"reservation_ids":self.reservation_ids,"status":self.status.text(),"transaction_id":self.transaction_id})
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct SettlementEnvelope {
    pub envelope_id: String,
    pub entries: Vec<LedgerEntry>,
    pub effective_time: String,
    pub causal_parent: Option<String>,
    pub responsible_owner: String,
    expected_versions: BTreeMap<String, u64>,
    reservation_ids: Vec<String>,
    prepared: bool,
    finished: bool,
}
impl SettlementEnvelope {
    pub(crate) fn new(
        envelope_id: impl Into<String>,
        entries: impl IntoIterator<Item = LedgerEntry>,
        effective_time: impl Into<String>,
        causal_parent: Option<String>,
        responsible_owner: impl Into<String>,
    ) -> Self {
        Self {
            envelope_id: envelope_id.into(),
            entries: entries.into_iter().collect(),
            effective_time: effective_time.into(),
            causal_parent,
            responsible_owner: responsible_owner.into(),
            expected_versions: BTreeMap::new(),
            reservation_ids: vec![],
            prepared: false,
            finished: false,
        }
    }
    pub(crate) fn for_treasury_fills(
        envelope_id: impl Into<String>,
        fills: &[TreasuryFill],
        accounts: &BTreeMap<String, BTreeMap<String, String>>,
        effective_time: impl Into<String>,
        causal_parent: Option<String>,
    ) -> Result<Self, AccountingError> {
        let mut entries = vec![];
        for fill in fills {
            let buyer = accounts.get(&fill.buyer_id).ok_or_else(|| {
                AccountingError::Domain(format!("unknown participant: {}", fill.buyer_id))
            })?;
            let seller = accounts.get(&fill.seller_id).ok_or_else(|| {
                AccountingError::Domain(format!("unknown participant: {}", fill.seller_id))
            })?;
            let get = |m: &BTreeMap<String, String>, k: &str| {
                m.get(k)
                    .cloned()
                    .ok_or_else(|| AccountingError::Domain(format!("missing {k} account")))
            };
            entries.extend([
                LedgerEntry {
                    account_id: get(buyer, "cash")?,
                    delta: -fill.cash_amount(),
                    instrument: "USD_CASH".into(),
                    unit: "USD".into(),
                },
                LedgerEntry {
                    account_id: get(seller, "cash")?,
                    delta: fill.cash_amount(),
                    instrument: "USD_CASH".into(),
                    unit: "USD".into(),
                },
                LedgerEntry {
                    account_id: get(seller, "treasury")?,
                    delta: -fill.quantity,
                    instrument: "TREASURY_5_10Y".into(),
                    unit: "treasury_face".into(),
                },
                LedgerEntry {
                    account_id: get(buyer, "treasury")?,
                    delta: fill.quantity,
                    instrument: "TREASURY_5_10Y".into(),
                    unit: "treasury_face".into(),
                },
            ]);
        }
        Ok(Self::new(
            envelope_id,
            entries,
            effective_time,
            causal_parent,
            "market.us.treasury.secondary",
        ))
    }
    pub(crate) fn prepare(
        &mut self,
        ledger: &mut AccountingLedger,
        witness: Option<&mut WitnessLedger>,
    ) -> Result<SettlementResult, AccountingError> {
        if self.prepared || self.finished {
            return Err(AccountingError::Domain(format!(
                "envelope cannot be prepared in its current state: {}",
                self.envelope_id
            )));
        }
        let mut touched = self
            .entries
            .iter()
            .map(|e| e.account_id.clone())
            .collect::<Vec<_>>();
        touched.sort();
        touched.dedup();
        for id in touched {
            self.expected_versions
                .insert(id.clone(), ledger.account(&id)?.version);
        }
        let mut outgoing = BTreeMap::<String, Decimal>::new();
        for entry in &self.entries {
            if entry.delta < Decimal::ZERO {
                *outgoing
                    .entry(entry.account_id.clone())
                    .or_insert(Decimal::ZERO) -= entry.delta
            }
        }
        for (id, value) in outgoing {
            let reserve_id = format!(
                "reservation.{}.{:04}",
                self.envelope_id,
                self.reservation_ids.len() + 1
            );
            if let Err(e) = ledger.reserve(&reserve_id, &id, value) {
                self.release_all(ledger)?;
                self.finished = true;
                let r = SettlementResult {
                    envelope_id: self.envelope_id.clone(),
                    status: SettlementStatus::FailedPrepare,
                    transaction_id: None,
                    failure_reason: Some(e.to_string()),
                    reservation_ids: vec![],
                };
                self.witness(witness, "settlement_prepare_failed", &r);
                return Ok(r);
            }
            self.reservation_ids.push(reserve_id)
        }
        self.prepared = true;
        let r = SettlementResult {
            envelope_id: self.envelope_id.clone(),
            status: SettlementStatus::Prepared,
            transaction_id: None,
            failure_reason: None,
            reservation_ids: self.reservation_ids.clone(),
        };
        self.witness(witness, "settlement_envelope_prepared", &r);
        Ok(r)
    }
    pub(crate) fn commit(
        &mut self,
        ledger: &mut AccountingLedger,
        mut witness: Option<&mut WitnessLedger>,
    ) -> Result<SettlementResult, AccountingError> {
        if !self.prepared || self.finished {
            return Err(AccountingError::Domain(format!(
                "envelope is not prepared: {}",
                self.envelope_id
            )));
        }
        let txid = format!("transaction.{}", self.envelope_id);
        let result = ledger.commit(
            &txid,
            &self.entries,
            &self.expected_versions,
            &self.reservation_ids,
            &self.effective_time,
        );
        match result {
            Ok(transaction) => {
                if let Some(witness) = witness.as_deref_mut() {
                    witness.append(
                        &self.effective_time,
                        "accounting_transaction_committed",
                        &self.responsible_owner,
                        json!({"transaction": transaction.to_dict()}),
                        "NONE",
                        self.causal_parent.as_deref(),
                    );
                }
                self.reservation_ids.clear();
                self.finished = true;
                let r = SettlementResult {
                    envelope_id: self.envelope_id.clone(),
                    status: SettlementStatus::Committed,
                    transaction_id: Some(txid),
                    failure_reason: None,
                    reservation_ids: vec![],
                };
                self.witness(witness, "settlement_envelope_committed", &r);
                Ok(r)
            }
            Err(e) => {
                self.release_all(ledger)?;
                self.finished = true;
                let r = SettlementResult {
                    envelope_id: self.envelope_id.clone(),
                    status: SettlementStatus::FailedCommit,
                    transaction_id: None,
                    failure_reason: Some(e.to_string()),
                    reservation_ids: vec![],
                };
                self.witness(witness, "settlement_commit_failed", &r);
                Ok(r)
            }
        }
    }
    fn release_all(&mut self, ledger: &mut AccountingLedger) -> Result<(), AccountingError> {
        for id in self.reservation_ids.clone().into_iter().rev() {
            ledger.release(&id)?
        }
        self.reservation_ids.clear();
        Ok(())
    }
    fn witness(&self, w: Option<&mut WitnessLedger>, kind: &str, result: &SettlementResult) {
        if let Some(w) = w {
            w.append(
                &self.effective_time,
                kind,
                &self.responsible_owner,
                json!({"settlement":result.to_dict()}),
                "NONE",
                self.causal_parent.as_deref(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounting::ledger::Account;

    fn decimal(value: &str) -> Decimal {
        Decimal::from_str_exact(value).unwrap()
    }

    #[test]
    fn prepare_failure_releases_earlier_reservations() {
        let mut ledger = AccountingLedger::new([
            Account {
                account_id: "buyer.cash".into(),
                owner_id: "buyer".into(),
                instrument: "USD_CASH".into(),
                unit: "USD".into(),
                balance: decimal("15"),
                account_kind: "asset".into(),
                allow_negative: false,
                version: 0,
                reserved: Decimal::ZERO,
            },
            Account {
                account_id: "seller.cash".into(),
                owner_id: "seller".into(),
                instrument: "USD_CASH".into(),
                unit: "USD".into(),
                balance: Decimal::ZERO,
                account_kind: "asset".into(),
                allow_negative: false,
                version: 0,
                reserved: Decimal::ZERO,
            },
            Account {
                account_id: "seller.treasury".into(),
                owner_id: "seller".into(),
                instrument: "TREASURY_5_10Y".into(),
                unit: "treasury_face".into(),
                balance: decimal("4"),
                account_kind: "asset".into(),
                allow_negative: false,
                version: 0,
                reserved: Decimal::ZERO,
            },
            Account {
                account_id: "buyer.treasury".into(),
                owner_id: "buyer".into(),
                instrument: "TREASURY_5_10Y".into(),
                unit: "treasury_face".into(),
                balance: Decimal::ZERO,
                account_kind: "asset".into(),
                allow_negative: false,
                version: 0,
                reserved: Decimal::ZERO,
            },
        ])
        .unwrap();
        let accounts = BTreeMap::from([
            (
                "buyer".into(),
                BTreeMap::from([
                    ("cash".into(), "buyer.cash".into()),
                    ("treasury".into(), "buyer.treasury".into()),
                ]),
            ),
            (
                "seller".into(),
                BTreeMap::from([
                    ("cash".into(), "seller.cash".into()),
                    ("treasury".into(), "seller.treasury".into()),
                ]),
            ),
        ]);
        let fill = TreasuryFill {
            fill_id: "fill.0001".into(),
            buyer_id: "buyer".into(),
            seller_id: "seller".into(),
            bucket_id: "TREASURY_5_10Y".into(),
            quantity: decimal("5"),
            price: decimal("3"),
            buy_order_id: "buy".into(),
            sell_order_id: "sell".into(),
        };
        let mut envelope = SettlementEnvelope::for_treasury_fills(
            "market.test",
            &[fill],
            &accounts,
            "2006-01-01T00:00:00Z",
            None,
        )
        .unwrap();
        assert_eq!(
            envelope.prepare(&mut ledger, None).unwrap().status,
            SettlementStatus::FailedPrepare
        );
        assert!(ledger.reservations().is_empty());
        assert!(ledger.transactions().is_empty());
    }
    #[test]
    fn successful_commit_is_balanced_and_atomic() {
        let mut ledger = cash_ledger("10");
        let mut envelope = cash_envelope("success", decimal("5"));
        assert_eq!(
            envelope.prepare(&mut ledger, None).unwrap().status,
            SettlementStatus::Prepared
        );
        assert_eq!(
            envelope.commit(&mut ledger, None).unwrap().status,
            SettlementStatus::Committed
        );
        assert_eq!(ledger.balance("buyer.cash").unwrap(), decimal("5"));
        assert_eq!(ledger.balance("seller.cash").unwrap(), decimal("5"));
        assert!(ledger.assert_conserved().is_ok());
    }

    #[test]
    fn unavailable_cash_fails_prepare_without_mutation() {
        let mut ledger = cash_ledger("4");
        let before = ledger.snapshot_for_hash();
        let mut envelope = cash_envelope("cash_failure", decimal("5"));
        assert_eq!(
            envelope.prepare(&mut ledger, None).unwrap().status,
            SettlementStatus::FailedPrepare
        );
        assert_eq!(ledger.snapshot_for_hash(), before);
    }

    #[test]
    fn version_conflict_fails_commit_and_preserves_concurrent_transaction() {
        let mut ledger = cash_ledger("10");
        let mut envelope = cash_envelope("version_conflict", decimal("5"));
        assert_eq!(
            envelope.prepare(&mut ledger, None).unwrap().status,
            SettlementStatus::Prepared
        );
        ledger
            .commit(
                "concurrent",
                &[
                    LedgerEntry {
                        account_id: "buyer.cash".into(),
                        delta: decimal("1"),
                        instrument: "USD_CASH".into(),
                        unit: "USD".into(),
                    },
                    LedgerEntry {
                        account_id: "seller.cash".into(),
                        delta: decimal("-1"),
                        instrument: "USD_CASH".into(),
                        unit: "USD".into(),
                    },
                ],
                &BTreeMap::from([("buyer.cash".into(), 0), ("seller.cash".into(), 0)]),
                &[],
                "period:concurrent",
            )
            .unwrap();
        assert_eq!(
            envelope.commit(&mut ledger, None).unwrap().status,
            SettlementStatus::FailedCommit
        );
        assert_eq!(ledger.balance("buyer.cash").unwrap(), decimal("11"));
        assert_eq!(ledger.balance("seller.cash").unwrap(), decimal("-1"));
        assert!(ledger.reservations().is_empty());
        assert_eq!(ledger.transactions().len(), 1);
        assert!(ledger.assert_conserved().is_ok());
    }

    fn cash_ledger(buyer_cash: &str) -> AccountingLedger {
        AccountingLedger::new([
            Account {
                account_id: "buyer.cash".into(),
                owner_id: "buyer".into(),
                instrument: "USD_CASH".into(),
                unit: "USD".into(),
                balance: decimal(buyer_cash),
                account_kind: "asset".into(),
                allow_negative: false,
                version: 0,
                reserved: Decimal::ZERO,
            },
            Account {
                account_id: "seller.cash".into(),
                owner_id: "seller".into(),
                instrument: "USD_CASH".into(),
                unit: "USD".into(),
                balance: Decimal::ZERO,
                account_kind: "signed_claim".into(),
                allow_negative: true,
                version: 0,
                reserved: Decimal::ZERO,
            },
        ])
        .unwrap()
    }

    fn cash_envelope(id: &str, amount: Decimal) -> SettlementEnvelope {
        SettlementEnvelope::new(
            id,
            [
                LedgerEntry {
                    account_id: "buyer.cash".into(),
                    delta: -amount,
                    instrument: "USD_CASH".into(),
                    unit: "USD".into(),
                },
                LedgerEntry {
                    account_id: "seller.cash".into(),
                    delta: amount,
                    instrument: "USD_CASH".into(),
                    unit: "USD".into(),
                },
            ],
            "period:test",
            Some("event.test".into()),
            "test",
        )
    }
}
