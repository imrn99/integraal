//! # Integraal
//!
//! *Integraal* aims to provide generic and efficient tools for [numerical integration][NI] in
//! the Rust Programming Language.
//!
//! # Quickstart
//!
//! Multiple standalone examples are provided in the GitHub [repository][GH]. You can also look at
//! the example provided for the [`Integraal`] structure for a very concise overview of the crate's
//! usage.
//!
//! # Features
//!
//! ## Policy
//!
//! As a rule of thumb, a computation method will be gated behind a feature if (a) it requires
//! additional dependencies, or (b) it is not implemented for all integral definitions (e.g.
//! a method that requires a uniform domain).
//!
//! ## Feature list
//!
//! - `montecarlo` -- enable the Monte-Carlo computation method.
//! - `romberg` -- enable the Romberg computation method.
//!
//! [NI]: https://en.wikipedia.org/wiki/Numerical_integration
//! [GH]: https://github.com/imrn99/integraal

// --- CUSTOM LINTS

// if compiling using nightly, enable auto feature-gates documentation
#![allow(unexpected_cfgs)]
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
