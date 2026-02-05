//! Test Runner and Execution Infrastructure
//!
//! This module provides the test runner and execution infrastructure for the
//! comprehensive property-based testing suite.

use crate::comprehensive::test_config::{PropertyTestResult, TestConfig};
use std::time::{Duration, Instant};

/// Property test runner that orchestrates execution of all property tests
pub struct PropertyTestRunner {
    config: TestConfig,
}

impl PropertyTestRunner {
    /// Create a new property test runner with the given configuration
    pub fn new(config: TestConfig) -> Self {
        Self { config }
    }

    /// Run a single property test with timing and result tracking
    pub fn run_property_test<F>(&self, property_name: &str, test_fn: F) -> PropertyTestResult
    where
        F: FnOnce() -> bool,
    {
        let start_time = Instant::now();
        let success = test_fn();
        let execution_time = start_time.elapsed();

        PropertyTestResult {
            property_name: property_name.to_string(),
            iterations_run: self.config.test_iterations,
            success,
            counterexample: None, // Will be populated when we implement actual property tests
            execution_time,
            memory_usage: None, // Will be implemented when we add memory tracking
        }
    }

    /// Get the current test configuration
    pub fn config(&self) -> &TestConfig {
        &self.config
    }
}

/// Assertion validator for property test results
pub struct AssertionValidator;

impl AssertionValidator {
    /// Validate that a property assertion holds
    pub fn validate_property<T, F>(input: T, property: F) -> bool
    where
        F: FnOnce(T) -> bool,
    {
        property(input)
    }

    /// Validate that an operation produces the expected result
    pub fn validate_result<T: PartialEq>(actual: T, expected: T) -> bool {
        actual == expected
    }

    /// Validate that an operation produces an error of the expected type
    pub fn validate_error<T, E>(result: Result<T, E>, expected_error: bool) -> bool {
        match (result.is_err(), expected_error) {
            (true, true) | (false, false) => true,
            _ => false,
        }
    }
}