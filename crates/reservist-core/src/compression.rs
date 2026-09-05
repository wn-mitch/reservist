use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::clock::SimulationClock;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct CompressionStep {
    pub from_time: String,
    pub to_time: String,
    pub event_id: String,
    pub elapsed_minutes: i64,
}
impl CompressionStep {
    pub(crate) fn to_dict(&self) -> Value {
        json!({"elapsed_minutes": self.elapsed_minutes, "event_id": self.event_id, "from_time": self.from_time, "to_time": self.to_time})
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct IntermeetingRealization {
    pub path_id: String,
    pub draw_key: String,
    pub mechanism_class: String,
    pub annualized_core_inflation: f64,
    pub housing_activity_direction: String,
    pub package_id: String,
}
impl IntermeetingRealization {
    pub(crate) fn to_dict(&self) -> Value {
        json!({"annualized_core_inflation": self.annualized_core_inflation, "draw_key": self.draw_key, "housing_activity_direction": self.housing_activity_direction, "mechanism_class": self.mechanism_class, "package_id": self.package_id, "path_id": self.path_id})
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct IntermeetingCompressor {
    pub seed: u64,
    steps: Vec<CompressionStep>,
    realizations: Vec<IntermeetingRealization>,
}
impl IntermeetingCompressor {
    pub(crate) const PATH_ID: &'static str = "path.intermeeting.inflation_housing";
    pub(crate) const MECHANISM_CLASS: &'static str = "INFLATION_PERSISTENCE_WITH_HOUSING_COOLING";
    pub(crate) fn new(seed: u64) -> Self {
        Self {
            seed,
            steps: Vec::new(),
            realizations: Vec::new(),
        }
    }
    pub(crate) fn next_step(&self, clock: &SimulationClock) -> Option<CompressionStep> {
        let event = clock.next_event()?;
        let start = clock.current_time;
        let end = event.due_time;
        let elapsed_minutes = end.seconds_since(start).div_euclid(60);
        if elapsed_minutes < 24 * 60 {
            return None;
        }
        Some(CompressionStep {
            from_time: start.to_string(),
            to_time: end.to_string(),
            event_id: event.stable_id.clone(),
            elapsed_minutes,
        })
    }
    pub(crate) fn record_step(&mut self, step: CompressionStep) {
        self.steps.push(step);
    }
    #[cfg(test)]
    pub(crate) fn steps(&self) -> &[CompressionStep] {
        &self.steps
    }
    pub(crate) fn realize(&mut self, package_id: &str) -> IntermeetingRealization {
        let draw_key = format!("{}|{}|magnitude", self.seed, Self::PATH_ID);
        let digest = Sha256::digest(draw_key.as_bytes());
        let draw = u64::from_be_bytes(
            digest[..8]
                .try_into()
                .expect("SHA-256 prefix has eight bytes"),
        ) as f64
            / 18_446_744_073_709_551_616.0;
        let policy_adjustment = if package_id == "WAIT_AND_WARN" {
            0.0
        } else {
            -0.15
        };
        let realization = IntermeetingRealization {
            path_id: Self::PATH_ID.into(),
            draw_key,
            mechanism_class: Self::MECHANISM_CLASS.into(),
            annualized_core_inflation: python_round(3.1 + draw * 0.8 + policy_adjustment, 3),
            housing_activity_direction: "cooling".into(),
            package_id: package_id.into(),
        };
        self.realizations.push(realization.clone());
        realization
    }
    #[cfg(any(test, feature = "tooling"))]
    pub(crate) fn realizations(&self) -> &[IntermeetingRealization] {
        &self.realizations
    }
    pub(crate) fn snapshot_for_hash(&self) -> Value {
        json!({"mechanism_class": Self::MECHANISM_CLASS, "path_id": Self::PATH_ID, "realizations": self.realizations.iter().map(IntermeetingRealization::to_dict).collect::<Vec<_>>(), "seed": self.seed, "steps": self.steps.iter().map(CompressionStep::to_dict).collect::<Vec<_>>()})
    }
}

/// Python's `round(value, digits)` uses nearest with ties to the even result.
fn python_round(value: f64, digits: u32) -> f64 {
    format!("{value:.precision$}", precision = digits as usize)
        .parse()
        .expect("formatted finite float parses")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn four_seeds_vary_magnitude_without_varying_mechanism() {
        let rows: Vec<_> = [20060328, 20060329, 20060330, 20060331]
            .into_iter()
            .map(|seed| IntermeetingCompressor::new(seed).realize("WAIT_AND_WARN"))
            .collect();
        assert!(
            rows.iter()
                .all(|row| row.mechanism_class == IntermeetingCompressor::MECHANISM_CLASS)
        );
        assert!(
            rows.iter()
                .all(|row| (2.95..=3.75).contains(&row.annualized_core_inflation))
        );
        assert!(
            rows.windows(2)
                .any(|pair| pair[0].annualized_core_inflation != pair[1].annualized_core_inflation)
        );
    }
    #[test]
    fn rounds_ties_as_python_does() {
        assert_eq!(python_round(3.1245, 3), 3.124);
        assert_eq!(python_round(3.1255, 3), 3.126);
    }
    #[test]
    fn recorded_steps_remain_in_their_observed_order() {
        let mut compressor = IntermeetingCompressor::new(20060328);
        compressor.record_step(CompressionStep {
            from_time: "2006-03-28T14:15:00-05:00".into(),
            to_time: "2006-03-29T08:30:00-05:00".into(),
            event_id: "event.release".into(),
            elapsed_minutes: 1095,
        });

        assert_eq!(compressor.steps().len(), 1);
        assert_eq!(compressor.steps()[0].event_id, "event.release");
    }
}
