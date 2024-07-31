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
