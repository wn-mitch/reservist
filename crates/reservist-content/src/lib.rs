#![forbid(unsafe_code)]
//! Offline compilation of authored catalogs and frozen scenarios.

mod assets;
pub mod authoring;
pub mod bindings;
pub mod catalog;
#[cfg(test)]
mod catalog_fixture;
mod composition;
pub mod frozen;
pub mod generate;
pub mod initialization;
mod inventory;
#[cfg(test)]
mod inventory_tests;
pub mod manifest;
mod projects;
pub mod slice;
mod source_comparison;
mod sovereign;
#[cfg(test)]
mod sovereign_tests;
mod templates;

#[cfg(test)]
mod m2_dialogue_tests;
#[cfg(test)]
mod m2_tests;
#[cfg(test)]
mod m3_tests;
#[cfg(test)]
mod volcker_november_tests;
#[cfg(test)]
mod volcker_tests;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub type Row = BTreeMap<String, String>;
pub type Tables = BTreeMap<String, Vec<Row>>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Issue {
    pub severity: String,
    pub category: String,
    pub record_id: String,
    pub issue: String,
}

#[derive(Debug, thiserror::Error)]
#[error("[{category}] {message}")]
pub struct ContentError {
    pub category: String,
    pub message: String,
}

impl ContentError {
    pub fn new(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            message: message.into(),
        }
    }
}

impl From<std::io::Error> for ContentError {
    fn from(error: std::io::Error) -> Self {
        Self::new("io", error.to_string())
    }
}

impl From<serde_json::Error> for ContentError {
    fn from(error: serde_json::Error) -> Self {
        Self::new("schema", error.to_string())
    }
}

impl From<reservist_core::canon::CanonError> for ContentError {
    fn from(error: reservist_core::canon::CanonError) -> Self {
        Self::new("schema", error.to_string())
    }
}
