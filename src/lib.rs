//! crate doc

// --- CUSTOM LINTS

// if compiling using nightly, enable auto feature-gates documentation
#![cfg_attr(nightly, feature(doc_auto_cfg))]
// more lints
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

// --- MODULES DECLARATION

mod parameters;
mod structure;

// --- RE-EXPORTS

pub use parameters::{ComputeMethod, DomainDescriptor, FunctionDescriptor};
pub use structure::Integraal;
