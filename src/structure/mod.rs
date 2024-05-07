//! main structure and computation code

// ------ MODULE DECLARATIONS

mod definitions;
mod implementations;

// ------ RE-EXPORTS

pub use definitions::{Integraal, IntegraalError};

// ------ TESTS

#[cfg(test)]
mod tests;
