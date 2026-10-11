//! Reusable CSV and CAS analysis library for the Casper project.

pub mod analysis;
pub mod io;
pub mod model;
pub mod pipeline;

pub use pipeline::{run_default, run_to_directory, run_with_paths, RunSummary};
