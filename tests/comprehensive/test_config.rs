//! Test Configuration Types and Utilities
//!
//! This module defines the configuration types and utilities used throughout the comprehensive
//! property-based testing suite.

use base64::alphabet::{Alphabet, STANDARD, URL_SAFE};
use base64::engine::{GeneralPurpose, GeneralPurposeConfig};
use std::time::Duration;

/// Test configuration for property-based tests
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub alphabet: AlphabetType,
    pub padding_mode: PaddingMode,
    pub engine_type: EngineType,
    pub test_iterations: usize,
    pub max_input_size: usize,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            alphabet: AlphabetType::Standard,
            padding_mode: PaddingMode::Canonical,
            engine_type: EngineType::GeneralPurpose,
            test_iterations: 1000,
            max_input_size: 1024 * 1024, // 1MB default
        }
    }
}

/// Alphabet types supported in testing
#[derive(Debug, Clone)]
pub enum AlphabetType {
    Standard,
    UrlSafe,
    Custom([u8; 64]),
}

/// Padding modes for testing
#[derive(Debug, Clone)]
pub enum PaddingMode {
    Canonical,
    None,
    Indifferent,
    RequireCanonical,
    RequireNone,
}

/// Engine types for testing
#[derive(Debug, Clone)]
pub enum EngineType {
    GeneralPurpose,
}

/// Test result for property-based tests
#[derive(Debug)]
pub struct PropertyTestResult {
    pub property_name: String,
    pub iterations_run: usize,
    pub success: bool,
    pub counterexample: Option<TestInput>,
    pub execution_time: Duration,
    pub memory_usage: Option<usize>,
}

/// Test input data structure
#[derive(Debug)]
pub struct TestInput {
    pub raw_input: Vec<u8>,
    pub config: TestConfig,
    pub expected_behavior: ExpectedBehavior,
}

/// Expected behavior for test validation
#[derive(Debug)]
pub enum ExpectedBehavior {
    Success,
    Error(ExpectedError),
}

/// Expected error types for validation
#[derive(Debug, PartialEq)]
pub enum ExpectedError {
    InvalidByte { position: usize, byte: u8 },
    InvalidLength { length: usize },
    InvalidLastSymbol { position: usize },
    InvalidPadding { position: usize },
    BufferTooSmall { required: usize, provided: usize },
}

impl TestConfig {
    /// Create a GeneralPurpose engine from this configuration
    pub fn create_engine(&self) -> GeneralPurpose {
        let alphabet = match &self.alphabet {
            AlphabetType::Standard => &STANDARD,
            AlphabetType::UrlSafe => &URL_SAFE,
            AlphabetType::Custom(chars) => {
                // Create a custom alphabet from the character array
                let alphabet_str = std::str::from_utf8(chars).expect("Custom alphabet should be valid UTF-8");
                // For property testing, we'll create the alphabet dynamically
                // This is not ideal for performance but necessary for testing
                Box::leak(Box::new(Alphabet::new(alphabet_str).expect("Custom alphabet should be valid")))
            }
        };

        let config = match self.padding_mode {
            PaddingMode::Canonical => GeneralPurposeConfig::new().with_encode_padding(true).with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent),
            PaddingMode::None => GeneralPurposeConfig::new().with_encode_padding(false).with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent),
            PaddingMode::Indifferent => GeneralPurposeConfig::new().with_encode_padding(true).with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent),
            PaddingMode::RequireCanonical => GeneralPurposeConfig::new().with_encode_padding(true).with_decode_padding_mode(base64::engine::DecodePaddingMode::RequireCanonical),
            PaddingMode::RequireNone => GeneralPurposeConfig::new().with_encode_padding(false).with_decode_padding_mode(base64::engine::DecodePaddingMode::RequireNone),
        };

        GeneralPurpose::new(alphabet, config)
    }
}