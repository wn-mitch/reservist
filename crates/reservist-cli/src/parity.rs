use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    path::{Path, PathBuf},
};

use reservist_content::{
    ContentError,
    frozen::{reseed_scenario, validate_scenario},
};
use reservist_core::{
    api::{
        FrozenScenario,
        tooling::{OracleRun, run_scenario},
    },
    canon::{canonical_bytes, canonical_text, load_json, sha256},
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(clap::Args)]
pub(crate) struct ParityArgs {
    #[arg(long, default_value = "tests/oracle/vectors")]
    vectors: PathBuf,
    #[arg(long, default_value = "tests/parity/attributions.toml")]
    attributions: PathBuf,
    #[arg(long, default_value="m1", value_parser=["m1"])]
    fixture: String,
    #[arg(long)]
    vector: Option<String>,
    #[arg(long, default_value = "target/parity")]
    output: PathBuf,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AttributionFile {
    version: u32,
    attribution: Vec<Attribution>,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Attribution {
    path: String,
    category: Category,
    clause: Option<String>,
    justification: String,
}
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Category {
    Normalization,
    AuthorizedDifference,
    NewRuntimeOnly,
}
#[derive(Serialize)]
struct Difference {
    path: String,
    expected: Option<Value>,
    actual: Option<Value>,
    ulp_distance: Option<u64>,
}
#[derive(Serialize)]
struct Normalization {
    path: String,
    expected: String,
    actual: String,
    justification: String,
}
#[derive(Default, Serialize)]
struct Comparison {
    compared_fields: usize,
    differences: Vec<Difference>,
    normalizations: Vec<Normalization>,
}

fn failure(message: impl Into<String>) -> ContentError {
    ContentError::new("parity", message)
}
fn valid_hash(value: &str) -> bool {
    value
        .strip_prefix("sha256:")
        .is_some_and(|hex| hex.len() == 64 && hex.bytes().all(|byte| byte.is_ascii_hexdigit()))
}
fn hash_field(path: &str) -> bool {
    matches!(
        path.rsplit('/').next(),
        Some(
            "state_hash"
                | "canonical_registry_hash"
                | "state_hash_before"
                | "state_hash_after"
                | "material_state_hash"
                | "material_state_hash_before"
                | "material_state_hash_after"
        )
    ) || path == "/material_snapshot/canonical_registry"
}
fn path_matches(pattern: &str, path: &str) -> bool {
    let Some((prefix, rest)) = pattern.split_once('*') else {
        return pattern == path;
    };
    let Some(path) = path.strip_prefix(prefix) else {
        return false;
    };
    path.char_indices()
        .map(|(index, _)| index)
        .chain(std::iter::once(path.len()))
        .any(|index| path_matches(rest, &path[index..]))
}
fn numeric_equal(left: &str, right: &str) -> bool {
    Decimal::from_str_exact(left)
        .ok()
        .zip(Decimal::from_str_exact(right).ok())
        .is_some_and(|(left, right)| left == right)
}
fn float_order(value: f64) -> u64 {
    let bits = value.to_bits();
    if bits & (1 << 63) == 0 {
        bits | (1 << 63)
    } else {
        !bits
    }
}
fn push_segment(path: &mut String, segment: &str) {
    path.push('/');
    for character in segment.chars() {
        match character {
            '~' => path.push_str("~0"),
            '/' => path.push_str("~1"),
            character => path.push(character),
        }
    }
}

impl Comparison {
    fn compare(
        &mut self,
        expected: Option<&Value>,
        actual: Option<&Value>,
        path: &mut String,
        rules: &[Attribution],
    ) {
        self.compared_fields += 1;
        if expected == actual {
            return;
        }
        if path == "/result/transcript" {
            return;
        }
        if let (Some(Value::String(left)), Some(Value::String(right))) = (expected, actual) {
            if hash_field(path) && valid_hash(left) && valid_hash(right) {
                return;
            }
            if numeric_equal(left, right)
                && let Some(rule) = rules.iter().find(|rule| {
                    rule.category == Category::Normalization && path_matches(&rule.path, path)
                })
            {
                self.normalizations.push(Normalization {
                    path: path.clone(),
                    expected: left.clone(),
                    actual: right.clone(),
                    justification: rule.justification.clone(),
                });
                return;
            }
        }
        match (expected, actual) {
            (Some(Value::Object(left)), Some(Value::Object(right))) => {
                let keys: BTreeSet<_> = left.keys().chain(right.keys()).collect();
                for key in keys {
                    let length = path.len();
                    push_segment(path, key);
                    self.compare(left.get(key), right.get(key), path, rules);
                    path.truncate(length);
                }
                return;
            }
            (Some(Value::Array(left)), Some(Value::Array(right))) => {
                for index in 0..left.len().max(right.len()) {
                    let length = path.len();
                    write!(path, "/{index}").expect("String writes cannot fail");
                    self.compare(left.get(index), right.get(index), path, rules);
                    path.truncate(length);
                }
                return;
            }
            (Some(left @ Value::Number(_)), Some(right @ Value::Number(_)))
                if canonical_text(left).expect("JSON number")
                    == canonical_text(right).expect("JSON number") =>
            {
                return;
            }
            _ => {}
        }
        let ulp_distance = expected
            .and_then(Value::as_f64)
            .zip(actual.and_then(Value::as_f64))
            .map(|(left, right)| float_order(left).abs_diff(float_order(right)));
        self.differences.push(Difference {
            path: path.clone(),
            expected: expected.cloned(),
            actual: actual.cloned(),
            ulp_distance,
        });
    }
    fn cli_text(
        &mut self,
        expected: &str,
        actual: &str,
        path: &str,
        replay: bool,
        rules: &[Attribution],
    ) {
        let left = expected.split('\n').collect::<Vec<_>>();
        let right = actual.split('\n').collect::<Vec<_>>();
        for index in 0..left.len().max(right.len()) {
            let (Some(left), Some(right)) = (left.get(index), right.get(index)) else {
                self.compare(
                    left.get(index).map(|v| Value::String((*v).into())).as_ref(),
                    right
                        .get(index)
                        .map(|v| Value::String((*v).into()))
                        .as_ref(),
                    &mut format!("{path}/{index}"),
                    rules,
                );
                continue;
            };
            if left == right {
                continue;
            }
            let a = left.split(' ').collect::<Vec<_>>();
            let b = right.split(' ').collect::<Vec<_>>();
            for token in 0..a.len().max(b.len()) {
                let pair = a
                    .get(token)
                    .and_then(|value| value.split_once('='))
                    .zip(b.get(token).and_then(|value| value.split_once('=')));
                if let Some(((left_key, left_value), (right_key, right_value))) = pair
                    && left_key == right_key
                {
                    if matches!(left_key, "state_hash" | "state")
                        && valid_hash(left_value.trim_end_matches(';'))
                        && valid_hash(right_value.trim_end_matches(';'))
                    {
                        continue;
                    }
                    if replay
                        && left_key == "transcript_bytes"
                        && left_value.parse::<usize>().is_ok()
                        && right_value.parse::<usize>().is_ok()
                    {
                        continue;
                    }
                    self.compare(
                        Some(&Value::String(left_value.into())),
                        Some(&Value::String(right_value.into())),
                        &mut format!("{path}/{index}/{left_key}"),
                        rules,
                    );
                } else {
                    self.compare(
                        a.get(token)
                            .map(|value| Value::String((*value).into()))
                            .as_ref(),
                        b.get(token)
                            .map(|value| Value::String((*value).into()))
                            .as_ref(),
                        &mut format!("{path}/{index}/{token}"),
                        rules,
                    );
                }
            }
        }
    }
}

fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, ContentError> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| failure(format!("vector lacks string {key}")))
}
fn totals(snapshot: &Value) -> Result<Vec<Value>, ContentError> {
    let accounts = snapshot["accounting"]["accounts"]
        .as_array()
        .ok_or_else(|| failure("accounting snapshot lacks accounts"))?;
    let mut totals: BTreeMap<(&str, &str), Decimal> = BTreeMap::new();
    for account in accounts {
        let key = (text(account, "instrument")?, text(account, "unit")?);
        let amount = Decimal::from_str_exact(text(account, "balance")?)
            .map_err(|error| failure(error.to_string()))?;
        *totals.entry(key).or_default() += amount;
    }
    Ok(totals.into_iter().map(|((instrument,unit),amount)|json!({"instrument":instrument,"unit":unit,"amount":amount.normalize().to_string()})).collect())
}
fn verify_witnesses(run: &OracleRun, scenario: &FrozenScenario) -> Vec<String> {
    let mut errors = Vec::new();
    let mut parents: BTreeSet<String> = scenario.initialization["scheduled_events"]
        .as_array()
        .into_iter()
        .flatten()
        .chain(scenario.tape["events"].as_array().into_iter().flatten())
        .filter_map(|event| event["stable_id"].as_str().map(Into::into))
        .collect();
    let mut receipts = Vec::new();
    for (index, event) in run.transcript.iter().enumerate() {
        if event["sequence"].as_u64() != Some(index as u64 + 1)
            || event["event_id"] != format!("event.{:06}", index + 1)
        {
            errors.push(format!("witness sequence mismatch at {}", index + 1));
        }
        if let Some(parent) = event["causal_parent"].as_str()
            && !parents.contains(parent)
        {
            errors.push(format!(
                "witness {} has no earlier causal parent {parent}",
                index + 1
            ));
        }
        if let Some(id) = event["event_id"].as_str() {
            parents.insert(id.into());
        }
        if let Some(transaction_id) = event["payload"]["transaction"]["transaction_id"].as_str() {
            parents.insert(transaction_id.into());
        }
        if event["transition_kind"] == "stage_receipt_recorded" {
            receipts.push(event["payload"]["receipt"].clone());
        }
        if canonical_text(event).ok().as_deref()
            != run
                .transcript_canonical_lines
                .get(index)
                .map(String::as_str)
        {
            errors.push(format!("noncanonical transcript line {}", index + 1));
        }
    }
    if receipts != run.receipts {
        errors.push("witnessed receipt order differs from result receipts".into());
    }
    if run.transcript.len() != run.transcript_canonical_lines.len() {
        errors.push("canonical transcript line count differs from witness count".into());
    }
    if sha256(&run.state_snapshot) != run.state_hash {
        errors.push("native state hash does not identify its snapshot".into());
    }
    errors
}
fn replay_stdout(scenario: &FrozenScenario) -> Result<String, ContentError> {
    let mut output = format!("replay passed: {}\n", scenario.scenario_hash);
    for package in ["WAIT_AND_WARN", "MEASURED_FIRMING", "FIRMING_BIAS"] {
        let first = run_scenario(scenario, package, Some("NORMAL"), None).map_err(failure)?;
        let second = run_scenario(scenario, package, Some("NORMAL"), None).map_err(failure)?;
        if first.result != second.result || first.transcript_bytes() != second.transcript_bytes() {
            return Err(failure(format!("native replay differs for {package}")));
        }
        writeln!(
            output,
            "{package}: state={}; transcript_bytes={}",
            first.state_hash,
            first.transcript_bytes().len()
        )
        .expect("String writes cannot fail");
    }
    Ok(output)
}

pub(crate) fn run(args: ParityArgs) -> Result<(), ContentError> {
    let rules: AttributionFile = toml::from_str(&std::fs::read_to_string(&args.attributions)?)
        .map_err(|error| failure(format!("invalid attribution document: {error}")))?;
    if rules.version != 1
        || rules.attribution.iter().any(|rule| {
            !rule.path.starts_with('/')
                || rule.justification.trim().is_empty()
                || (rule.category == Category::AuthorizedDifference
                    && rule
                        .clause
                        .as_ref()
                        .is_none_or(|clause| clause.trim().is_empty()))
        })
    {
        return Err(failure(
            "invalid attribution version, path, justification, or adoption clause",
        ));
    }
    let scenario = match args.fixture.as_str() {
        "m1" => validate_scenario(Path::new("scenarios/mvp_2006_cycle_m1"))?,
        _ => return Err(failure("unknown fixture")),
    };
    let mut paths = std::fs::read_dir(&args.vectors)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.retain(|path| {
        path.extension()
            .is_some_and(|extension| extension == "json")
            && args
                .vector
                .as_ref()
                .is_none_or(|name| path.file_stem().is_some_and(|stem| stem == name.as_str()))
    });
    paths.sort();
    if paths.is_empty() {
        return Err(failure("no selected parity vectors"));
    }
    std::fs::create_dir_all(&args.output)?;
    let mut failed = 0;
    let count = paths.len();
    let mut replay_cache: BTreeMap<i64, String> = BTreeMap::new();
    for path in paths {
        let vector = load_json(&path)?;
        let name = path
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| failure("non-UTF8 vector name"))?;
        let seed = vector["seed"]
            .as_i64()
            .ok_or_else(|| failure("vector lacks integer seed"))?;
        let variation = reseed_scenario(&scenario, seed)?;
        let mut comparison = Comparison::default();
        let mut integrity = Vec::new();
        let mut conservation = Value::Null;
        if text(&vector, "vector_kind")? == "play_script" {
            let commands = vector["script"]
                .as_array()
                .ok_or_else(|| failure("play vector lacks script"))?
                .iter()
                .map(|command| {
                    command
                        .as_str()
                        .ok_or_else(|| failure("play script command is not a string"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let stdout = crate::play::scripted(&variation, text(&vector, "package")?, &commands)?;
            comparison.compare(
                vector.get("stdout"),
                Some(&Value::String(stdout)),
                &mut "/stdout".into(),
                &rules.attribution,
            );
            comparison.compare(
                vector.get("exit_code"),
                Some(&json!(0)),
                &mut "/exit_code".into(),
                &rules.attribution,
            );
        } else {
            let native = run_scenario(
                &variation,
                text(&vector, "package")?,
                vector["request"].as_str(),
                None,
            )
            .map_err(failure)?;
            let actual = serde_json::to_value(&native)?;
            for field in [
                "scenario_hash",
                "transcript",
                "result",
                "state_snapshot",
                "material_snapshot",
                "receipts",
                "canonical_registry",
            ] {
                comparison.compare(
                    vector.get(field),
                    actual.get(field),
                    &mut format!("/{field}"),
                    &rules.attribution,
                );
            }
            integrity = verify_witnesses(&native, &variation);
            let native_totals = totals(&native.state_snapshot)?;
            let oracle_totals = totals(&vector["state_snapshot"])?;
            if native_totals != oracle_totals {
                integrity.push("conserved accounting totals differ from oracle".into());
            }
            conservation = json!({"native":native_totals,"oracle":oracle_totals});
            let console = crate::render_run(&native, false);
            if !console
                .lines()
                .any(|line| line == format!("state_hash={}", native.state_hash))
            {
                integrity.push("CLI state hash differs from native result".into());
            }
            comparison.cli_text(
                text(&vector["cli_stdout"], "run")?,
                &console,
                "/cli_stdout/run",
                false,
                &rules.attribution,
            );
            if let std::collections::btree_map::Entry::Vacant(entry) = replay_cache.entry(seed) {
                entry.insert(replay_stdout(&variation)?);
            }
            comparison.cli_text(
                text(&vector["cli_stdout"], "replay_check")?,
                &replay_cache[&seed],
                "/cli_stdout/replay_check",
                true,
                &rules.attribution,
            );
            let mut bytes = canonical_bytes(&actual)?;
            bytes.push(b'\n');
            std::fs::write(args.output.join(format!("{name}.native.json")), bytes)?;
        }
        let passed = comparison.differences.is_empty() && integrity.is_empty();
        println!(
            "{name}: {} ({} differences, {} integrity failures, {} decimal normalizations)",
            if passed { "PASS" } else { "FAIL" },
            comparison.differences.len(),
            integrity.len(),
            comparison.normalizations.len()
        );
        if !passed {
            failed += 1;
        }
        let report = json!({"vector":name,"passed":passed,"comparison":comparison,"integrity_failures":integrity,"conserved_totals":conservation,"state_hash_comparison":"native snapshot identity and repeat-run identity; cross-runtime hashes excluded by plan section 7.2"});
        let mut bytes = canonical_bytes(&report)?;
        bytes.push(b'\n');
        std::fs::write(args.output.join(format!("{name}.report.json")), bytes)?;
    }
    if failed != 0 {
        return Err(failure(format!(
            "{failed} of {count} vectors failed; reports: {}",
            args.output.display()
        )));
    }
    println!(
        "parity passed: {count} vectors; reports: {}",
        args.output.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn rules() -> Vec<Attribution> {
        vec![Attribution {
            path: "/*/price".into(),
            category: Category::Normalization,
            clause: None,
            justification: "Exact decimal scale normalization".into(),
        }]
    }
    #[test]
    fn decimal_scale_is_attributed_but_value_changes_and_identifiers_fail() {
        let mut compared = Comparison::default();
        compared.compare(
            Some(&json!("1.00")),
            Some(&json!("1")),
            &mut "/order/price".into(),
            &rules(),
        );
        assert!(compared.differences.is_empty());
        assert_eq!(compared.normalizations.len(), 1);
        compared.compare(
            Some(&json!("1.00")),
            Some(&json!("1.01")),
            &mut "/order/price".into(),
            &rules(),
        );
        compared.compare(
            Some(&json!("1.00")),
            Some(&json!("1")),
            &mut "/order/id".into(),
            &rules(),
        );
        assert_eq!(compared.differences.len(), 2);
    }
    #[test]
    fn one_ulp_of_float_drift_is_not_decimal_normalization() {
        let mut compared = Comparison::default();
        compared.compare(
            Some(&json!(1.0)),
            Some(&json!(1.0_f64.next_up())),
            &mut "/belief/estimate".into(),
            &rules(),
        );
        assert_eq!(compared.differences[0].ulp_distance, Some(1));
    }
    #[test]
    fn missing_field_and_null_are_distinct() {
        let mut compared = Comparison::default();
        compared.compare(
            Some(&json!({"expected":null})),
            Some(&json!({})),
            &mut String::new(),
            &[],
        );
        assert_eq!(compared.differences[0].path, "/expected");
    }
}
