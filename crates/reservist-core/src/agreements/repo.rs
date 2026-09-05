use crate::{
    accounting::ledger::{AccountingLedger, LedgerEntry, amount},
    settlement::envelope::SettlementEnvelope,
    time::Instant,
    witness::WitnessLedger,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum RepoStatus {
    Active,
    NonRollPending,
    Settled,
}
impl RepoStatus {
    fn text(&self) -> &'static str {
        match self {
            Self::Active => "ACTIVE",
            Self::NonRollPending => "NON_ROLL_PENDING",
            Self::Settled => "SETTLED",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct RepoMaturityResult {
    pub agreement_id: String,
    pub status: String,
    pub maturity_time: String,
    pub liquidity_deficit: Decimal,
    pub trigger_witness: String,
}
impl RepoMaturityResult {
    pub(crate) fn to_dict(&self) -> Value {
        json!({"agreement_id":self.agreement_id,"liquidity_deficit":self.liquidity_deficit.to_string(),"maturity_time":self.maturity_time,"status":self.status,"trigger_witness":self.trigger_witness})
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BilateralRepoAgreement {
    pub agreement_id: String,
    pub lender_id: String,
    pub borrower_id: String,
    pub principal: Decimal,
    pub collateral_quantity: Decimal,
    pub haircut: Decimal,
    pub maturity_time: String,
    pub roll_policy: String,
    pub accounts: std::collections::BTreeMap<String, String>,
    pub status: RepoStatus,
    pub non_roll_witness: Option<String>,
}
impl BilateralRepoAgreement {
    pub(crate) fn from_state(id: impl Into<String>, v: &Value) -> Result<Self, String> {
        let s = |k: &str| {
            v[k].as_str()
                .map(Into::into)
                .ok_or_else(|| format!("missing string {k}"))
        };
        let status = match v["status"].as_str().unwrap_or("ACTIVE") {
            "ACTIVE" => RepoStatus::Active,
            "NON_ROLL_PENDING" => RepoStatus::NonRollPending,
            "SETTLED" => RepoStatus::Settled,
            _ => return Err("invalid repo status".into()),
        };
        let accounts = v["accounts"]
            .as_object()
            .ok_or("missing accounts")?
            .iter()
            .map(|(k, v)| {
                v.as_str()
                    .map(|x| (k.clone(), x.into()))
                    .ok_or("invalid account")
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            agreement_id: id.into(),
            lender_id: s("lender_id")?,
            borrower_id: s("borrower_id")?,
            principal: amount(&v["principal"]).map_err(|e| e.to_string())?,
            collateral_quantity: amount(&v["collateral_quantity"]).map_err(|e| e.to_string())?,
            haircut: amount(&v["haircut"]).map_err(|e| e.to_string())?,
            maturity_time: s("maturity_time")?,
            roll_policy: s("roll_policy")?,
            accounts,
            status,
            non_roll_witness: None,
        })
    }
    pub(crate) fn process_non_roll(
        &mut self,
        at: &str,
        ledger: &AccountingLedger,
        trigger: &str,
        witness: Option<&mut WitnessLedger>,
    ) -> Result<RepoMaturityResult, String> {
        if self.status != RepoStatus::Active {
            return Err("repo maturity may be processed only once".into());
        }
        if Instant::parse(at).map_err(|e| e.to_string())?
            < Instant::parse(&self.maturity_time).map_err(|e| e.to_string())?
        {
            return Err("repo non-roll cannot be processed before maturity".into());
        }
        if self.roll_policy != "COUNTERPARTY_DECIDES_NO_AUTOMATIC_RENEWAL" {
            return Err("repo agreement lacks the no-automatic-renewal contract".into());
        }
        let cash = ledger
            .account(
                self.accounts
                    .get("borrower_cash")
                    .ok_or("missing borrower cash")?,
            )
            .map_err(|e| e.to_string())?
            .available();
        let deficit = (self.principal - cash).max(Decimal::ZERO);
        self.status = RepoStatus::NonRollPending;
        self.non_roll_witness = Some(trigger.into());
        let r = RepoMaturityResult {
            agreement_id: self.agreement_id.clone(),
            status: if deficit.is_zero() {
                "NON_ROLL_READY_TO_SETTLE"
            } else {
                "NON_ROLL_LIQUIDITY_DEFICIT"
            }
            .into(),
            maturity_time: at.into(),
            liquidity_deficit: deficit,
            trigger_witness: trigger.into(),
        };
        if let Some(w) = witness {
            w.append(
                at,
                "repo_non_roll_recorded",
                &self.agreement_id,
                json!({"repo_maturity":r.to_dict()}),
                "NONE",
                Some(trigger),
            );
        }
        Ok(r)
    }
    pub(crate) fn settlement_envelope(
        &self,
        at: &str,
        parent: Option<String>,
    ) -> Result<SettlementEnvelope, String> {
        if self.status != RepoStatus::NonRollPending {
            return Err("repo settlement requires a witnessed non-roll decision".into());
        }
        let a = |k: &str| {
            self.accounts
                .get(k)
                .cloned()
                .ok_or_else(|| format!("missing repo account: {k}"))
        };
        Ok(SettlementEnvelope::new(
            format!("repo.{}.maturity", self.agreement_id),
            [
                LedgerEntry {
                    account_id: a("borrower_cash")?,
                    delta: -self.principal,
                    instrument: "USD_CASH".into(),
                    unit: "USD".into(),
                },
                LedgerEntry {
                    account_id: a("lender_cash")?,
                    delta: self.principal,
                    instrument: "USD_CASH".into(),
                    unit: "USD".into(),
                },
                LedgerEntry {
                    account_id: a("lender_repo_claim")?,
                    delta: -self.principal,
                    instrument: "REPO_CLAIM".into(),
                    unit: "USD".into(),
                },
                LedgerEntry {
                    account_id: a("borrower_repo_obligation")?,
                    delta: self.principal,
                    instrument: "REPO_CLAIM".into(),
                    unit: "USD".into(),
                },
                LedgerEntry {
                    account_id: a("lender_collateral_control")?,
                    delta: -self.collateral_quantity,
                    instrument: "TREASURY_COLLATERAL_CONTROL".into(),
                    unit: "treasury_face".into(),
                },
                LedgerEntry {
                    account_id: a("borrower_collateral_encumbrance")?,
                    delta: self.collateral_quantity,
                    instrument: "TREASURY_COLLATERAL_CONTROL".into(),
                    unit: "treasury_face".into(),
                },
            ],
            at,
            parent,
            self.agreement_id.clone(),
        ))
    }
    pub(crate) fn mark_settled(&mut self) -> Result<(), String> {
        if self.status != RepoStatus::NonRollPending {
            return Err("repo is not pending settlement".into());
        }
        self.status = RepoStatus::Settled;
        Ok(())
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({"accounts":self.accounts,"agreement_id":self.agreement_id,"borrower_id":self.borrower_id,"collateral_quantity":self.collateral_quantity.to_string(),"haircut":self.haircut.to_string(),"lender_id":self.lender_id,"maturity_time":self.maturity_time,"non_roll_witness":self.non_roll_witness,"principal":self.principal.to_string(),"roll_policy":self.roll_policy,"status":self.status.text()})
    }
}
