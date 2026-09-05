#![forbid(unsafe_code)]
//! Deterministic causal simulation, independent of client input and rendering.

pub mod api;
pub mod canon;
pub mod fidelity;
pub mod phase;
pub mod save;

pub(crate) mod accounting;
#[cfg(test)]
pub(crate) mod adapters;
pub(crate) mod agreements;
pub(crate) mod authority;
pub(crate) mod bodies;
pub(crate) mod calendar;
pub(crate) mod claims;
pub(crate) mod clock;
pub(crate) mod cognition;
pub(crate) mod commitments;
pub(crate) mod communication;
pub(crate) mod compression;
pub(crate) mod delivery;
pub(crate) mod execution;
pub(crate) mod folder;
pub(crate) mod legal;
pub(crate) mod markets;
pub(crate) mod media;
pub(crate) mod monitoring;
pub(crate) mod observation;
pub(crate) mod packages;
pub(crate) mod participants;
pub(crate) mod player;
pub(crate) mod population;
pub(crate) mod postmortem;
pub(crate) mod records;
pub(crate) mod routing;
pub(crate) mod scenario;
pub(crate) mod settlement;
pub(crate) mod staff;
pub(crate) mod state;
pub(crate) mod time;
pub(crate) mod uncertainty;
pub(crate) mod witness;

#[cfg(test)]
mod conformance_tests;
#[cfg(test)]
mod simulation_gates;
