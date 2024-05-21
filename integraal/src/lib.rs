//! # Integraal
//!
//! *Integraal* aims to provide generic and efficient tools for [numerical integration][NI] in
//! the Rust Programming Language.
//!
//! The crate currently implements a very specific subsection of its ambitious scope. It roughly
//! corresponds to the example provided for the [`Integraal`] example.
//!
//! [NI]: https://en.wikipedia.org/wiki/Numerical_integration

// --- CUSTOM LINTS

// if compiling using nightly, enable auto feature-gates documentation
#![cfg_attr(nightly, feature(doc_auto_cfg))]
// more lints
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
// allow some exceptions
#![allow(clippy::cast_precision_loss)]

// --- MODULES DECLARATION

mod parameters;
mod structure;
mod traits;

// --- RE-EXPORTS

pub use parameters::{ComputeMethod, DomainDescriptor, FunctionDescriptor};
pub use structure::{Integraal, IntegraalError};
pub use traits::Scalar;
