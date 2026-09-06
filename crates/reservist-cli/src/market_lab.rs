use std::{
    fmt::Write as _,
    path::{Path, PathBuf},
};

use clap::{Args, Subcommand, ValueEnum};
use reservist_core::{
    canon::canonical_bytes,
    market_lab::{MarketLabFixture, MarketLabReport, run_fixture},
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
                            rows.push(json!({"dealer_capacity":dealer,"haircut":haircut,"leverage":leverage,"peak_forced_sale":peak_forced_sale,"peak_residual_imbalance":peak_residual,"reconciliation_passed":report.reconciliation.passed,"regime":report.regime,"residual_demand":residual}));
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
            let result = json!({"schema_version":"reservist.market-lab.compare.v1","control":control_report.fixture_id,"treatment":treatment_report.fixture_id,"periods":rows});
            emit(json_value_bytes(&result)?, output).map_err(|e| e.to_string())
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
    writeln!(out, "final regime={} reconciliation={} cash={} treasuries={} futures={} repo={} collateral={} atomic_failures={}", report.regime, report.reconciliation.passed, report.reconciliation.cash_conserved, report.reconciliation.treasuries_conserved, report.reconciliation.futures_reconciled, report.reconciliation.repo_reconciled, report.reconciliation.collateral_reconciled, report.reconciliation.failed_settlement_atomic).expect("String write cannot fail");
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
