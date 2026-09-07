use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    market_ecology::{EcologyFixture, run_ecology},
    market_lab::{MarketLabFixture, MarketLabReport, PreconditionClassification, run_fixture},
    market_lab_runtime::compose_runtime_slices,
};

pub const SUITE_SCHEMA_VERSION: &str = "reservist.market-lab.suite.v1";

#[derive(Clone, Debug, Deserialize)]
pub struct SuiteManifest {
    pub schema_version: String,
    pub suite_id: String,
    pub runtime_fixture: PathBuf,
    pub ecology: EcologySpec,
    #[serde(default)]
    pub provisional_thresholds: BTreeMap<String, String>,
    pub experiments: Vec<ExperimentSpec>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EcologySpec {
    pub concentrated: PathBuf,
    pub distributed: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExperimentSpec {
    pub experiment_id: String,
    pub tier: u8,
    pub control: PathBuf,
    pub treatment: PathBuf,
    #[serde(default)]
    pub declared_treatments: Vec<String>,
    #[serde(default)]
    pub required_mechanics: Vec<String>,
    #[serde(default)]
    pub neutralized_mechanics: Vec<String>,
    #[serde(default)]
    pub expectations: Vec<DirectionalExpectation>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DirectionalExpectation {
    pub metric: Metric,
    pub relation: Relation,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Metric {
    PeakBasisDisplacement,
    PriceDrawdown,
    TotalLiquidation,
    EndogenousSales,
    PeakResidualImbalance,
    PeakRationing,
    FinalMarginHeadroom,
    FinalRepoCapacity,
    FinalFuturesPositionAbs,
    FedPurchases,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Equal,
}

#[derive(Clone, Debug, Serialize)]
pub struct SuiteReport {
    pub schema_version: String,
    pub suite_id: String,
    pub tiers: BTreeMap<u8, TierResult>,
    pub experiments: Vec<ExperimentResult>,
    pub supplemental_results: BTreeMap<u8, serde_json::Value>,
    pub provisional_thresholds: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TierResult {
    pub passed: bool,
    pub experiment_count: usize,
    pub failures: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ExperimentResult {
    pub experiment_id: String,
    pub tier: u8,
    pub passed: bool,
    pub gate_failures: Vec<String>,
    pub declared_treatments: Vec<String>,
    pub required_mechanics: Vec<String>,
    pub neutralized_mechanics: Vec<String>,
    pub pre_treatment_identical: bool,
    pub first_divergence_period: Option<usize>,
    pub first_causal_event: Option<String>,
    pub control: ExperimentOutcome,
    pub treatment: ExperimentOutcome,
    pub directional_checks: Vec<DirectionalCheck>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ExperimentOutcome {
    pub fixture_id: String,
    pub precondition: PreconditionClassification,
    pub regime: String,
    pub reconciliation_passed: bool,
    pub metrics: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DirectionalCheck {
    pub metric: Metric,
    pub relation: Relation,
    pub control: String,
    pub treatment: String,
    pub passed: bool,
}

pub fn load_manifest(path: &Path) -> Result<SuiteManifest, String> {
    let source = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    let manifest: SuiteManifest = toml::from_str(&source).map_err(|error| error.to_string())?;
    if manifest.schema_version != SUITE_SCHEMA_VERSION {
        return Err(format!(
            "unsupported market-lab suite schema: {}",
            manifest.schema_version
        ));
    }
    if manifest.experiments.is_empty() {
        return Err("market-lab suite requires at least one experiment".into());
    }
    Ok(manifest)
}

pub fn run_suite(manifest_path: &Path, selected_tier: Option<u8>) -> Result<SuiteReport, String> {
    let manifest = load_manifest(manifest_path)?;
    let root = manifest_path.parent().unwrap_or(Path::new("."));
    let mut experiments = Vec::new();
    for spec in manifest
        .experiments
        .iter()
        .filter(|spec| selected_tier.is_none_or(|tier| spec.tier <= tier))
    {
        experiments.push(run_experiment(root, spec)?);
    }
    let mut supplemental_results = BTreeMap::new();
    if selected_tier.is_none_or(|tier| tier >= 5) {
        let concentrated = run_ecology(&load_ecology(&root.join(&manifest.ecology.concentrated))?)?;
        let distributed = run_ecology(&load_ecology(&root.join(&manifest.ecology.distributed))?)?;
        let same_aggregates = concentrated.aggregate_cash == distributed.aggregate_cash
            && concentrated.aggregate_treasuries == distributed.aggregate_treasuries
            && concentrated.aggregate_repo == distributed.aggregate_repo;
        let changed = concentrated.total_forced_sales != distributed.total_forced_sales
            || concentrated.cash_clearing_price != distributed.cash_clearing_price
            || concentrated.futures_residual_imbalance != distributed.futures_residual_imbalance;
        let passed = same_aggregates
            && changed
            && concentrated.accounting_reconciled
            && distributed.accounting_reconciled
            && concentrated.causal_parents_complete
            && distributed.causal_parents_complete;
        supplemental_results.insert(
            5,
            serde_json::json!({
                "passed":passed,
                "same_aggregates":same_aggregates,
                "distribution_changes_dynamics":changed,
                "concentrated":concentrated,
                "distributed":distributed
            }),
        );
    }
    if selected_tier.is_none_or(|tier| tier >= 7) {
        let runtime_report = run_fixture(&load_fixture(&root.join(&manifest.runtime_fixture))?)?;
        let slices = compose_runtime_slices(&runtime_report)?;
        let passed = slices.len() == 8
            && slices.iter().all(|slice| {
                slice.evidence_precedes_decisions
                    && !slice.hidden_state_leaked
                    && slice.causal_reconstruction_complete
            });
        supplemental_results.insert(
            7,
            serde_json::json!({"passed":passed,"fixture":runtime_report.fixture_id,"slices":slices}),
        );
    }
    if experiments.is_empty() && supplemental_results.is_empty() {
        return Err(format!(
            "suite contains no experiments for tier {}",
            selected_tier.unwrap_or_default()
        ));
    }
    let mut tiers = BTreeMap::new();
    for tier in experiments.iter().map(|result| result.tier) {
        let rows = experiments
            .iter()
            .filter(|result| result.tier == tier)
            .collect::<Vec<_>>();
        let failures = rows
            .iter()
            .flat_map(|result| {
                result
                    .gate_failures
                    .iter()
                    .map(move |failure| format!("{}: {failure}", result.experiment_id))
            })
            .collect::<Vec<_>>();
        tiers.insert(
            tier,
            TierResult {
                passed: failures.is_empty(),
                experiment_count: rows.len(),
                failures,
            },
        );
    }
    for (tier, result) in &supplemental_results {
        let passed = result["passed"].as_bool().unwrap_or(false);
        tiers.insert(
            *tier,
            TierResult {
                passed,
                experiment_count: 1,
                failures: if passed {
                    vec![]
                } else {
                    vec![format!("tier {tier} supplemental gate failed")]
                },
            },
        );
    }
    let mut failed_prerequisite = None;
    for tier in 0..=7 {
        let Some(result) = tiers.get_mut(&tier) else {
            continue;
        };
        if let Some(prerequisite) = failed_prerequisite {
            result.passed = false;
            result
                .failures
                .push(format!("prerequisite tier {prerequisite} failed"));
        } else if !result.passed {
            failed_prerequisite = Some(tier);
        }
    }
    Ok(SuiteReport {
        schema_version: "reservist.market-lab.suite-result.v1".into(),
        suite_id: manifest.suite_id,
        tiers,
        experiments,
        supplemental_results,
        provisional_thresholds: manifest.provisional_thresholds,
    })
}

fn run_experiment(root: &Path, spec: &ExperimentSpec) -> Result<ExperimentResult, String> {
    let control = run_fixture(&load_fixture(&root.join(&spec.control))?)?;
    let treatment = run_fixture(&load_fixture(&root.join(&spec.treatment))?)?;
    if control.seed != treatment.seed {
        return Err(format!(
            "{}: control and treatment seeds differ",
            spec.experiment_id
        ));
    }
    if control.periods.len() != treatment.periods.len() {
        return Err(format!(
            "{}: control and treatment period counts differ",
            spec.experiment_id
        ));
    }
    let first_divergence = control
        .periods
        .iter()
        .zip(&treatment.periods)
        .position(|(a, b)| market_state(a) != market_state(b));
    let first_causal_event = first_divergence.and_then(|index| {
        treatment.periods[index]
            .shocks
            .first()
            .and_then(|row| row["shock_id"].as_str())
            .or_else(|| {
                treatment.periods[index]
                    .policy_events
                    .first()
                    .and_then(|row| row["proposal_id"].as_str())
            })
            .map(str::to_owned)
    });
    let treatment_period = treatment
        .precondition
        .first_shock_period
        .unwrap_or(u32::MAX) as usize;
    let pre_treatment_identical = control
        .periods
        .iter()
        .zip(&treatment.periods)
        .take(treatment_period.saturating_sub(1))
        .all(|(a, b)| market_state(a) == market_state(b));
    let mut checks = Vec::new();
    for expectation in &spec.expectations {
        let a = metric(&control, expectation.metric)?;
        let b = metric(&treatment, expectation.metric)?;
        checks.push(DirectionalCheck {
            metric: expectation.metric,
            relation: expectation.relation,
            control: a.normalize().to_string(),
            treatment: b.normalize().to_string(),
            passed: relation_holds(a, b, expectation.relation),
        });
    }
    let mut gate_failures = Vec::new();
    for (label, report) in [("control", &control), ("treatment", &treatment)] {
        if !matches!(
            report.precondition.classification,
            PreconditionClassification::ValidEquilibrium
                | PreconditionClassification::ValidNonEquilibrium
        ) {
            gate_failures.push(format!(
                "{label} precondition is {:?}",
                report.precondition.classification
            ));
        }
        if !report.reconciliation.passed {
            gate_failures.push(format!("{label} reconciliation failed"));
        }
        if report
            .periods
            .iter()
            .any(|period| period.causal_parents.is_empty())
        {
            gate_failures.push(format!(
                "{label} contains a material period without causal parents"
            ));
        }
    }
    if !pre_treatment_identical {
        gate_failures.push("control and treatment diverge before treatment".into());
    }
    if first_divergence.is_some() && first_causal_event.is_none() {
        gate_failures.push("first divergence lacks a scheduled causal event".into());
    }
    for check in &checks {
        if !check.passed {
            gate_failures.push(format!(
                "directional check failed for {:?}: control={} treatment={}",
                check.metric, check.control, check.treatment
            ));
        }
    }
    let passed = gate_failures.is_empty();
    Ok(ExperimentResult {
        experiment_id: spec.experiment_id.clone(),
        tier: spec.tier,
        passed,
        gate_failures,
        declared_treatments: spec.declared_treatments.clone(),
        required_mechanics: spec.required_mechanics.clone(),
        neutralized_mechanics: spec.neutralized_mechanics.clone(),
        pre_treatment_identical,
        first_divergence_period: first_divergence.map(|index| index + 1),
        first_causal_event,
        control: outcome(&control)?,
        treatment: outcome(&treatment)?,
        directional_checks: checks,
    })
}
fn load_ecology(path: &Path) -> Result<EcologyFixture, String> {
    let source = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    toml::from_str(&source).map_err(|error| error.to_string())
}

fn load_fixture(path: &Path) -> Result<MarketLabFixture, String> {
    let source = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("toml") => toml::from_str(&source).map_err(|error| error.to_string()),
        Some("json") => serde_json::from_str(&source).map_err(|error| error.to_string()),
        _ => Err(format!("unsupported fixture extension: {}", path.display())),
    }
}

fn market_state(
    period: &crate::market_lab::PeriodReport,
) -> (&str, &str, &str, &str, &str, &str, &str) {
    (
        &period.cash_treasury_price,
        &period.futures_price,
        &period.fund_cash,
        &period.fund_treasury_position,
        &period.fund_futures_position,
        &period.dealer_inventory,
        &period.residual_imbalance,
    )
}

fn metric(report: &MarketLabReport, metric: Metric) -> Result<Decimal, String> {
    let maximum = |values: Vec<&str>| -> Result<Decimal, String> {
        values
            .into_iter()
            .map(parse)
            .collect::<Result<Vec<_>, _>>()
            .map(|values| values.into_iter().max().unwrap_or_default())
    };
    match metric {
        Metric::PeakBasisDisplacement => parse(&report.impulse_response.maximum_basis_displacement),
        Metric::PriceDrawdown => parse(&report.impulse_response.maximum_price_drawdown),
        Metric::TotalLiquidation => parse(&report.impulse_response.total_realized_liquidation),
        Metric::EndogenousSales => parse(&report.impulse_response.cumulative_endogenous_sales),
        Metric::PeakResidualImbalance => maximum(
            report
                .periods
                .iter()
                .map(|period| period.residual_imbalance.as_str())
                .collect(),
        ),
        Metric::PeakRationing => maximum(
            report
                .periods
                .iter()
                .map(|period| period.rationed_quantity.as_str())
                .collect(),
        ),
        Metric::FinalMarginHeadroom => parse(
            &report
                .periods
                .last()
                .ok_or("empty report")?
                .available_margin_headroom,
        ),
        Metric::FinalRepoCapacity => parse(
            report
                .periods
                .last()
                .and_then(|period| period.repo_decision["capacity"].as_str())
                .ok_or("missing final repo capacity")?,
        ),
        Metric::FinalFuturesPositionAbs => parse(
            &report
                .periods
                .last()
                .ok_or("empty report")?
                .fund_futures_position,
        )
        .map(|value| value.abs()),
        Metric::FedPurchases => report
            .periods
            .iter()
            .map(|period| parse(&period.fed_purchase_quantity))
            .sum(),
    }
}

fn relation_holds(control: Decimal, treatment: Decimal, relation: Relation) -> bool {
    match relation {
        Relation::Greater => treatment > control,
        Relation::GreaterOrEqual => treatment >= control,
        Relation::Less => treatment < control,
        Relation::LessOrEqual => treatment <= control,
        Relation::Equal => treatment == control,
    }
}

fn outcome(report: &MarketLabReport) -> Result<ExperimentOutcome, String> {
    let metrics = [
        ("peak_basis_displacement", Metric::PeakBasisDisplacement),
        ("price_drawdown", Metric::PriceDrawdown),
        ("total_liquidation", Metric::TotalLiquidation),
        ("endogenous_sales", Metric::EndogenousSales),
        ("peak_residual_imbalance", Metric::PeakResidualImbalance),
        ("peak_rationing", Metric::PeakRationing),
        ("final_margin_headroom", Metric::FinalMarginHeadroom),
        ("final_repo_capacity", Metric::FinalRepoCapacity),
        (
            "final_futures_position_abs",
            Metric::FinalFuturesPositionAbs,
        ),
        ("fed_purchases", Metric::FedPurchases),
    ]
    .into_iter()
    .map(|(name, metric_name)| {
        metric(report, metric_name).map(|value| (name.into(), value.normalize().to_string()))
    })
    .collect::<Result<_, _>>()?;
    Ok(ExperimentOutcome {
        fixture_id: report.fixture_id.clone(),
        precondition: report.precondition.classification.clone(),
        regime: report.regime.clone(),
        reconciliation_passed: report.reconciliation.passed,
        metrics,
    })
}

fn parse(value: &str) -> Result<Decimal, String> {
    Decimal::from_str_exact(value).map_err(|_| format!("invalid report decimal: {value}"))
}
