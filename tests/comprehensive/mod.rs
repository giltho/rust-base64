//! Comprehensive Property-Based Testing Suite for rust-base64
//!
//! This module contains a comprehensive property-based testing suite for the rust-base64 library
//! using the bolero testing framework. The suite validates correctness, performance characteristics,
//! error handling, and configuration compliance across all supported features.
//!
//! The testing approach leverages bolero's property-based testing capabilities to generate random
//! inputs and verify universal properties that should hold for all valid inputs. This provides
//! much more comprehensive coverage than traditional example-based unit tests.

pub mod generators;
pub mod properties;
pub mod test_config;
pub mod test_runner;

// Re-export key types for convenience
pub use generators::*;
pub use properties::*;
pub use test_config::*;
pub use test_runner::*;