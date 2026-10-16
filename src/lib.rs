//! PreTxSim adapter for Aleo Privacy Virtual Hackathon - Private dApps.
//!
//! Air-gapped structural inspection for the Aleo Privacy Virtual Hackathon hackathon.
//! Exposes the AleoAdapter adapter plus the shared Adapter core.

pub mod adapters;
pub use adapters::{Adapter, AdapterResult, AdapterVerdict};
pub use adapters::aleo::AleoAdapter;
