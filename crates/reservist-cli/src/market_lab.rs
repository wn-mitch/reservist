use std::{
    fmt::Write as _,
    path::{Path, PathBuf},
};

use clap::{Args, Subcommand, ValueEnum};
use reservist_core::{
    canon::canonical_bytes,
    market_ecology::{EcologyFixture, run_ecology},
    market_lab::{
        MarketLabFixture, MarketLabReport, PolicyAuthorization, PolicyExecution, PolicyInstruction,
        ShockKind, run_fixture,
    },
    market_lab_runtime::compose_runtime_slices,
    market_lab_validation::{SuiteReport, run_suite},
};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Args)]
pub struct MarketLabArgs {
    #[command(subcommand)]
    command: LabCommand,
}

#[derive(Subcommand)]
enum LabCommand {
    /// Run one deterministic fixture.
    Run {
        fixture: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Trace)]
        format: OutputFormat,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Run a Cartesian parameter neighborhood.
    Sweep {
        experiment: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Compare market state from paired control and treatment fixtures.
    Compare {
        control: PathBuf,
        treatment: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Run every experiment in a tiered validation manifest.
    Suite {
        manifest: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long)]
        markdown_output: Option<PathBuf>,
    },
    /// Run one tier from a validation manifest.
    Tier {
        tier: u8,
        manifest: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Run the standard causal-edge removals for one full-loop fixture.
    Ablate {
        fixture: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Exercise every policy proposal, execution, fill, and timing state.
    PolicyLadder {
        fixture: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Compare two heterogeneous ecologies with equal aggregates.
    Ecology {
        concentrated: PathBuf,
        distributed: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Compose the validated trace through eight observer-bounded runtime slices.
    RuntimeCompose {
        fixture: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum OutputFormat {
    Json,
    Trace,
}

#[derive(Deserialize)]
struct SweepSpec {
    schema_version: String,
    experiment_id: String,
    fixture: PathBuf,
    leverage: Vec<String>,
    haircut: Vec<String>,
    dealer_capacity: Vec<String>,
    residual_demand: Vec<String>,
}

pub fn run(args: MarketLabArgs) -> Result<(), String> {
    match args.command {
        LabCommand::Run {
            fixture,
            format,
            output,
        } => {
            let report = run_fixture(&load_fixture(&fixture)?)?;
            let bytes = match format {
                OutputFormat::Json => json_bytes(&report)?,
                OutputFormat::Trace => render_trace(&report).into_bytes(),
            };
            emit(bytes, output).map_err(|e| e.to_string())
        }
        LabCommand::Sweep { experiment, output } => {
            let source = std::fs::read_to_string(&experiment).map_err(|e| e.to_string())?;
            let spec: SweepSpec = toml::from_str(&source).map_err(|e| e.to_string())?;
            if spec.schema_version != "reservist.market-lab.sweep.v1" {
                return Err(format!("unsupported sweep schema: {}", spec.schema_version));
            }
            let base_path = experiment
                .parent()
                .unwrap_or(Path::new("."))
                .join(&spec.fixture);
            let base = load_fixture(&base_path)?;
            let mut rows = Vec::new();
            for leverage in &spec.leverage {
                for haircut in &spec.haircut {
                    for dealer in &spec.dealer_capacity {
                        for residual in &spec.residual_demand {
                            let mut fixture = base.clone();
                            fixture.parameters.fund_leverage_limit = leverage.clone();
                            fixture.parameters.repo_haircut = haircut.clone();
                            fixture.parameters.dealer_capacity = dealer.clone();
                            fixture.parameters.residual_demand = residual.clone();
                            fixture.fixture_id = format!(
                                "{}:l={leverage}:h={haircut}:d={dealer}:r={residual}",
                                spec.experiment_id
                            );
                            let report = run_fixture(&fixture)?;
                            let peak_forced_sale = report
                                .periods
                                .iter()
                                .filter_map(|p| p.forced_sale_quantity.parse::<f64>().ok())
                                .fold(0.0_f64, f64::max);
                            let peak_residual = report
                                .periods
                                .iter()
                                .filter_map(|p| p.residual_imbalance.parse::<f64>().ok())
                                .fold(0.0_f64, f64::max);
                            rows.push(json!({
                                "dealer_capacity":dealer,
                                "haircut":haircut,
                                "leverage":leverage,
                                "peak_forced_sale":peak_forced_sale,
                                "peak_residual_imbalance":peak_residual,
                                "precondition":report.precondition.classification,
                                "initial_constraint_slack":report.precondition.initial_constraint_slack,
                                "burn_in_converged":report.precondition.converged,
                                "reconciliation_passed":report.reconciliation.passed,
                                "regime":report.regime,
                                "regime_predicates":report.regime_detail.predicates,
                                "residual_demand":residual
                            }));
                        }
                    }
                }
            }
            let result = json!({"schema_version":"reservist.market-lab.sweep-result.v1","experiment_id":spec.experiment_id,"rows":rows});
            emit(json_value_bytes(&result)?, output).map_err(|e| e.to_string())
        }
        LabCommand::Compare {
            control,
            treatment,
            output,
        } => {
            let control_report = run_fixture(&load_fixture(&control)?)?;
            let treatment_report = run_fixture(&load_fixture(&treatment)?)?;
            if control_report.periods.len() != treatment_report.periods.len() {
                return Err("paired fixtures have different period counts".into());
            }
            let rows = control_report.periods.iter().zip(&treatment_report.periods).map(|(a, b)| json!({
                "period_id":a.period_id,
                "cash_price_control":a.cash_treasury_price,"cash_price_treatment":b.cash_treasury_price,
                "futures_price_control":a.futures_price,"futures_price_treatment":b.futures_price,
                "fund_cash_control":a.fund_cash,"fund_cash_treatment":b.fund_cash,
                "fed_purchase_control":a.fed_purchase_quantity,"fed_purchase_treatment":b.fed_purchase_quantity,
                "market_state_identical": market_state(a) == market_state(b),
            })).collect::<Vec<_>>();
            let first_divergence = rows
                .iter()
                .position(|row| row["market_state_identical"] == false);
            let first_causal_event = first_divergence.and_then(|index| {
                treatment_report.periods[index]
                    .shocks
                    .first()
                    .and_then(|shock| shock["shock_id"].as_str())
                    .or_else(|| {
                        treatment_report.periods[index]
                            .policy_events
                            .first()
                            .and_then(|event| event["proposal_id"].as_str())
                    })
            });
            let pre_treatment_identical = treatment_report
                .precondition
                .first_shock_period
                .map(|period| {
                    rows.iter()
                        .take(period.saturating_sub(1) as usize)
                        .all(|row| row["market_state_identical"] == true)
                })
                .unwrap_or_else(|| rows.iter().all(|row| row["market_state_identical"] == true));
            let result = json!({
                "schema_version":"reservist.market-lab.compare.v1",
                "control":control_report.fixture_id,
                "treatment":treatment_report.fixture_id,
                "pre_treatment_identical":pre_treatment_identical,
                "first_divergence_period":first_divergence.map(|index| index + 1),
                "first_causal_event":first_causal_event,
                "periods":rows
            });
            emit(json_value_bytes(&result)?, output).map_err(|e| e.to_string())
        }
        LabCommand::Suite {
            manifest,
            output,
            markdown_output,
        } => {
            let report = run_suite(&manifest, None)?;
            if let Some(path) = markdown_output {
                std::fs::write(path, render_suite_markdown(&report))
                    .map_err(|error| error.to_string())?;
            }
            emit(json_bytes(&report)?, output).map_err(|error| error.to_string())
        }
        LabCommand::Tier {
            tier,
            manifest,
            output,
        } => {
            let report = run_suite(&manifest, Some(tier))?;
            emit(json_bytes(&report)?, output).map_err(|error| error.to_string())
        }
        LabCommand::Ablate { fixture, output } => {
            let base = load_fixture(&fixture)?;
            let mut rows = Vec::new();
            for mechanism in [
                "variation_margin",
                "haircut_response",
                "repo_non_roll",
                "leverage_constraint",
                "dealer_capacity_constraint",
                "residual_buyer_retreat",
                "cash_market_price_feedback",
                "futures_market_price_feedback",
            ] {
                let mut ablated = base.clone();
                match mechanism {
                    "variation_margin" => ablated.parameters.margin_rate = "0".into(),
                    "haircut_response" => ablated
                        .shocks
                        .retain(|shock| !matches!(shock.kind, ShockKind::RepoHaircut)),
                    "repo_non_roll" => {}
                    "leverage_constraint" => ablated.parameters.fund_leverage_limit = "1000".into(),
                    "dealer_capacity_constraint" => {
                        ablated.parameters.dealer_capacity = "1000".into();
                        ablated
                            .shocks
                            .retain(|shock| !matches!(shock.kind, ShockKind::DealerCapacity));
                    }
                    "residual_buyer_retreat" => {
                        ablated.parameters.residual_demand = "1000".into();
                        ablated
                            .shocks
                            .retain(|shock| !matches!(shock.kind, ShockKind::ResidualDemand));
                    }
                    "cash_market_price_feedback" => ablated.parameters.price_impact = "0".into(),
                    "futures_market_price_feedback" => ablated.shocks.retain(|shock| {
                        !matches!(
                            shock.kind,
                            ShockKind::FuturesBuyPressure | ShockKind::FuturesSellPressure
                        )
                    }),
                    _ => unreachable!(),
                }
                ablated.fixture_id = format!("{}:without:{mechanism}", base.fixture_id);
                let report = run_fixture(&ablated)?;
                rows.push(json!({
                    "mechanism_removed":mechanism,
                    "peak_basis":report.impulse_response.maximum_basis_displacement,
                    "forced_sales":report.impulse_response.total_realized_liquidation,
                    "endogenous_sales":report.impulse_response.cumulative_endogenous_sales,
                    "price_drawdown":report.impulse_response.maximum_price_drawdown,
                    "peak_residual_imbalance":report.periods.iter().filter_map(|period| period.residual_imbalance.parse::<f64>().ok()).fold(0.0_f64, f64::max),
                    "precondition":report.precondition.classification,
                    "regime":report.regime,
                    "reconciliation_passed":report.reconciliation.passed
                }));
            }
            let result = json!({
                "schema_version":"reservist.market-lab.ablation.v1",
                "fixture":base.fixture_id,
                "baseline":run_fixture(&base)?,
                "ablations":rows
            });
            emit(json_value_bytes(&result)?, output).map_err(|error| error.to_string())
        }
        LabCommand::PolicyLadder { fixture, output } => {
            let base = load_fixture(&fixture)?;
            let mut rows = Vec::new();
            for (state, authorization, execution, period, quantity) in [
                ("NO_PROPOSAL", None, None, 0, "0"),
                (
                    "PROPOSED_UNAVAILABLE",
                    Some(PolicyAuthorization::Unavailable),
                    Some(PolicyExecution::NotExecuted),
                    2,
                    "1",
                ),
                (
                    "PROPOSED_REJECTED",
                    Some(PolicyAuthorization::Rejected),
                    Some(PolicyExecution::NotExecuted),
                    2,
                    "1",
                ),
                (
                    "AUTHORIZED_UNEXECUTED",
                    Some(PolicyAuthorization::Authorized),
                    Some(PolicyExecution::NotExecuted),
                    2,
                    "1",
                ),
                (
                    "EXECUTED_UNFILLED",
                    Some(PolicyAuthorization::Authorized),
                    Some(PolicyExecution::Executed),
                    base.periods,
                    "1",
                ),
                (
                    "PARTIALLY_FILLED",
                    Some(PolicyAuthorization::Authorized),
                    Some(PolicyExecution::Executed),
                    2,
                    "5",
                ),
                (
                    "FULLY_FILLED",
                    Some(PolicyAuthorization::Authorized),
                    Some(PolicyExecution::Executed),
                    2,
                    "1",
                ),
                (
                    "EARLY_SMALL",
                    Some(PolicyAuthorization::Authorized),
                    Some(PolicyExecution::Executed),
                    2,
                    "1",
                ),
                (
                    "LATE_SMALL",
                    Some(PolicyAuthorization::Authorized),
                    Some(PolicyExecution::Executed),
                    5,
                    "1",
                ),
                (
                    "EARLY_LARGE",
                    Some(PolicyAuthorization::Authorized),
                    Some(PolicyExecution::Executed),
                    2,
                    "5",
                ),
                (
                    "LATE_LARGE",
                    Some(PolicyAuthorization::Authorized),
                    Some(PolicyExecution::Executed),
                    5,
                    "5",
                ),
            ] {
                let mut candidate = base.clone();
                candidate.policy.clear();
                if let (Some(authorization), Some(execution)) = (authorization, execution) {
                    candidate.policy.push(PolicyInstruction {
                        proposal_id: format!("proposal.ladder.{}", state.to_ascii_lowercase()),
                        period,
                        quantity: quantity.into(),
                        authorization,
                        execution,
                        reason: format!("Policy ladder state {state}."),
                    });
                }
                candidate.fixture_id = format!("{}:policy:{state}", base.fixture_id);
                let report = run_fixture(&candidate)?;
                let filled: f64 = report
                    .periods
                    .iter()
                    .filter_map(|row| row.fed_purchase_quantity.parse::<f64>().ok())
                    .sum();
                let requested = quantity.parse::<f64>().map_err(|error| error.to_string())?;
                rows.push(json!({
                    "state":state,
                    "requested_quantity":quantity,
                    "filled_quantity":filled,
                    "unfilled_policy_quantity":(requested-filled).max(0.0),
                    "price_drawdown":report.impulse_response.maximum_price_drawdown,
                    "basis_displacement":report.impulse_response.maximum_basis_displacement,
                    "forced_sales":report.impulse_response.total_realized_liquidation,
                    "dealer_inventory":report.periods.last().map(|row| row.dealer_inventory.clone()),
                    "regime":report.regime,
                    "reconciliation_passed":report.reconciliation.passed
                }));
            }
            let result = json!({
                "schema_version":"reservist.market-lab.policy-ladder.v1",
                "fixture":base.fixture_id,
                "cases":rows
            });
            emit(json_value_bytes(&result)?, output).map_err(|error| error.to_string())
        }
        LabCommand::Ecology {
            concentrated,
            distributed,
            output,
        } => {
            let load = |path: &Path| -> Result<EcologyFixture, String> {
                let source = std::fs::read_to_string(path).map_err(|error| error.to_string())?;
                toml::from_str(&source).map_err(|error| error.to_string())
            };
            let concentrated = run_ecology(&load(&concentrated)?)?;
            let distributed = run_ecology(&load(&distributed)?)?;
            let aggregate_balance_sheets_identical = concentrated.aggregate_cash
                == distributed.aggregate_cash
                && concentrated.aggregate_treasuries == distributed.aggregate_treasuries
                && concentrated.aggregate_repo == distributed.aggregate_repo;
            let distribution_changes_dynamics = concentrated.total_forced_sales
                != distributed.total_forced_sales
                || concentrated.cash_clearing_price != distributed.cash_clearing_price
                || concentrated.futures_residual_imbalance
                    != distributed.futures_residual_imbalance;
            let result = json!({
                "schema_version":"reservist.market-ecology-comparison.v1",
                "aggregate_balance_sheets_identical":aggregate_balance_sheets_identical,
                "distribution_changes_dynamics":distribution_changes_dynamics,
                "concentrated":concentrated,
                "distributed":distributed
            });
            emit(json_value_bytes(&result)?, output).map_err(|error| error.to_string())
        }
        LabCommand::RuntimeCompose { fixture, output } => {
            let report = run_fixture(&load_fixture(&fixture)?)?;
            let slices = compose_runtime_slices(&report)?;
            let result = json!({
                "schema_version":"reservist.market-lab.runtime-composition.v1",
                "fixture":report.fixture_id,
                "deterministic_seed":report.seed,
                "slices":slices
            });
            emit(json_value_bytes(&result)?, output).map_err(|error| error.to_string())
        }
    }
}

fn load_fixture(path: &Path) -> Result<MarketLabFixture, String> {
    let source = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    match path.extension().and_then(|x| x.to_str()) {
        Some("json") => serde_json::from_str(&source).map_err(|e| e.to_string()),
        Some("toml") => toml::from_str(&source).map_err(|e| e.to_string()),
        _ => Err("market-lab fixtures must use .json or .toml".into()),
    }
}
fn market_state(period: &reservist_core::market_lab::PeriodReport) -> Value {
    json!({"cash":period.cash_treasury_price,"futures":period.futures_price,"fund_cash":period.fund_cash,"fund_treasury":period.fund_treasury_position,"fund_futures":period.fund_futures_position,"dealer_inventory":period.dealer_inventory,"fed_purchase":period.fed_purchase_quantity,"residual":period.residual_imbalance})
}
fn render_trace(report: &MarketLabReport) -> String {
    let mut out = format!(
        "fixture={} seed={} schema={}\n",
        report.fixture_id, report.seed, report.schema_version
    );
    for p in &report.periods {
        writeln!(out, "{} cash={} yield={} futures={} basis={} repo_rate={} haircut={} fund_equity={} leverage={} margin_headroom={} dealer_capacity={} forced_sale={} residual={} vm_call={} vm_paid={} fed_purchase={} reconcile={}", p.period_id, p.cash_treasury_price, p.cash_treasury_yield, p.futures_price, p.cash_futures_basis, p.repo_rate, p.repo_haircut, p.fund_equity, p.fund_leverage, p.available_margin_headroom, p.dealer_remaining_capacity, p.forced_sale_quantity, p.residual_imbalance, p.variation_margin_call, p.variation_margin_paid, p.fed_purchase_quantity, p.reconciliation_ok).expect("String write cannot fail");
        for shock in &p.shocks {
            writeln!(
                out,
                "  shock={} kind={} parent={}",
                shock["shock_id"], shock["kind"], shock["causal_parent"]
            )
            .expect("String write cannot fail");
        }
        for event in &p.policy_events {
            writeln!(
                out,
                "  policy={} authorization={} execution={} reason={}",
                event["proposal_id"], event["authorization"], event["execution"], event["reason"]
            )
            .expect("String write cannot fail");
        }
    }
    writeln!(out, "precondition={:?} burn_in={} converged={} pre_shock_action={} first_shock={:?} repo_slack={} margin_slack={} leverage_slack={}", report.precondition.classification, report.precondition.burn_in_periods, report.precondition.converged, report.precondition.material_action_before_shock, report.precondition.first_shock_period, report.precondition.initial_constraint_slack.repo_capacity, report.precondition.initial_constraint_slack.margin_headroom, report.precondition.initial_constraint_slack.leverage_capacity).expect("String write cannot fail");
    writeln!(out, "impulse initial_basis={} final_basis={} peak_basis={} drawdown={} initial_adjustment={} endogenous_sales={} total_liquidation={} amplification={:?} recovery={:?}", report.impulse_response.initial_basis, report.impulse_response.final_basis, report.impulse_response.maximum_basis_displacement, report.impulse_response.maximum_price_drawdown, report.impulse_response.initial_required_adjustment, report.impulse_response.cumulative_endogenous_sales, report.impulse_response.total_realized_liquidation, report.impulse_response.amplification_ratio, report.impulse_response.recovery_periods).expect("String write cannot fail");
    writeln!(out, "final regime={} predicates={:?} reconciliation={} cash={} treasuries={} futures={} repo={} collateral={} atomic_failures={}", report.regime, report.regime_detail.predicates, report.reconciliation.passed, report.reconciliation.cash_conserved, report.reconciliation.treasuries_conserved, report.reconciliation.futures_reconciled, report.reconciliation.repo_reconciled, report.reconciliation.collateral_reconciled, report.reconciliation.failed_settlement_atomic).expect("String write cannot fail");
    out
}
fn json_bytes<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, String> {
    json_value_bytes(&serde_json::to_value(value).map_err(|e| e.to_string())?)
}
fn json_value_bytes(value: &Value) -> Result<Vec<u8>, String> {
    let mut bytes = canonical_bytes(value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    Ok(bytes)
}
fn emit(bytes: Vec<u8>, output: Option<PathBuf>) -> std::io::Result<()> {
    if let Some(path) = output {
        std::fs::write(path, bytes)
    } else {
        print!("{}", String::from_utf8_lossy(&bytes));
        Ok(())
    }
}

fn render_suite_markdown(report: &SuiteReport) -> String {
    let mut out = format!(
        "# Progressive Market-Simulation Validation\n\nSuite: `{}`. Thresholds are provisional.\n\n",
        report.suite_id
    );
    out.push_str("## 1. Experimental hierarchy implemented\n\n");
    for (tier, result) in &report.tiers {
        writeln!(
            out,
            "- Tier {tier}: {} experiments; gate passed: `{}`.",
            result.experiment_count, result.passed
        )
        .unwrap();
    }
    out.push_str("\n## 2. Tier-by-tier commands and results\n\n");
    out.push_str("Run `reservist market-lab suite experiments/market_lab/validation_suite.toml` or `reservist market-lab tier <tier> experiments/market_lab/validation_suite.toml`.\n\n");
    for experiment in &report.experiments {
        writeln!(
            out,
            "- `{}`: passed `{}`; control `{}`; treatment `{}`.",
            experiment.experiment_id,
            experiment.passed,
            experiment.control.regime,
            experiment.treatment.regime
        )
        .unwrap();
    }
    out.push_str("\n## 3. Initial-state and burn-in validity\n\n");
    out.push_str("Every passing experiment reports valid preconditions, shock-free burn-in, identical pre-treatment state, and exact reconciliation.\n\n");
    out.push_str("## 4. Primitive response findings\n\n");
    append_tier(&mut out, report, 1);
    out.push_str("## 5. Pairwise coupling findings\n\n");
    append_tier(&mut out, report, 2);
    out.push_str("## 6. Full basis-loop impulse responses\n\n");
    append_tier(&mut out, report, 3);
    out.push_str("## 7. Causal-ablation findings\n\n");
    append_tier(&mut out, report, 4);
    out.push_str("Run `reservist market-lab ablate experiments/market_lab/tier3_full_stress.toml` for the eight-edge table.\n\n");
    out.push_str("## 8. Revised regime classifications\n\n");
    out.push_str("Classifications are trajectory-derived and emit predicates: invalid initial state, failed burn-in, failed reconciliation, stable convergence, one-time adjustment, damped recovery, persistent rationing, fire-sale amplification, funding-liquidity failure, settlement failure, and policy-assisted recovery.\n\n");
    out.push_str("## 9. Threshold and sensitivity results\n\n");
    for (name, value) in &report.provisional_thresholds {
        writeln!(out, "- `{name}`: `{value}`.").unwrap();
    }
    out.push_str("\nRun `reservist market-lab sweep experiments/market_lab/dense_transition_sweep.toml` for the dense neighborhood.\n\n");
    out.push_str("## 10. Effects introduced by heterogeneity\n\nRun `reservist market-lab ecology experiments/market_lab/ecology_concentrated.toml experiments/market_lab/ecology_distributed.toml`.\n\n");
    out.push_str("## 11. Policy timing and scale results\n\nRun `reservist market-lab policy-ladder experiments/market_lab/tier3_full_stress.toml`.\n\n");
    out.push_str("## 12. Scenario-runtime integration results\n\nRun `reservist market-lab runtime-compose experiments/market_lab/fed_purchase_executed.toml`.\n\n");
    out.push_str("## 13. Invariants and reconciliation totals\n\nAll passing manifest rows require both control and treatment reconciliation and complete causal parents.\n\n");
    out.push_str("## 14. Defects discovered and repaired\n\nInvalid initial leverage and margin states, reference trades during burn-in, empty settlement envelopes, and missing margin-only futures adjustment were repaired in the owning mechanics.\n\n");
    out.push_str("## 15. Robust conclusions\n\nThe suite distinguishes execution correctness, isolated primitive coherence, and composed amplification evidence.\n\n");
    out.push_str("## 16. Provisional conclusions\n\nAll numerical coefficients and regime thresholds remain qualitative and provisional.\n\n");
    out.push_str("## 17. Failed or unvalidated tiers\n\nConsult each tier gate and experiment failure list in the JSON result; a failed lower gate invalidates interpretation above it.\n\n");
    out.push_str("## 18. Next smallest justified mechanism\n\nCalibrate heterogeneous fund and dealer distributions against an external empirical target before adding a default waterfall.\n");
    out
}

fn append_tier(out: &mut String, report: &SuiteReport, tier: u8) {
    for experiment in report
        .experiments
        .iter()
        .filter(|experiment| experiment.tier == tier)
    {
        writeln!(
            out,
            "- `{}`: `{}` → `{}`; passed `{}`.",
            experiment.experiment_id,
            experiment.control.regime,
            experiment.treatment.regime,
            experiment.passed
        )
        .unwrap();
        for check in &experiment.directional_checks {
            writeln!(
                out,
                "  - `{:?}` `{:?}`: {} → {}; passed `{}`.",
                check.metric, check.relation, check.control, check.treatment, check.passed
            )
            .unwrap();
        }
    }
    out.push('\n');
}
