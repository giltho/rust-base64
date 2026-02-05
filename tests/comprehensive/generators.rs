//! Input Generators for Property-Based Testing
//!
//! This module provides generators for creating test inputs for property-based testing
//! of the rust-base64 library. The generators create various types of inputs including
//! byte sequences, base64 strings, invalid inputs, and engine configurations.

use crate::comprehensive::test_config::{AlphabetType, PaddingMode, TestConfig};
use bolero_generator::{gen, ValueGenerator};

/// Generator for arbitrary byte sequences
#[derive(Debug)]
pub struct ByteSequenceGenerator {
    max_size: usize,
}

impl ByteSequenceGenerator {
    pub fn new(max_size: usize) -> Self {
        Self { max_size }
    }
}

impl ValueGenerator for ByteSequenceGenerator {
    type Output = Vec<u8>;

    fn generate<D>(&self, driver: &mut D) -> Option<Self::Output>
    where
        D: bolero_generator::driver::Driver,
    {
        let size = gen::<usize>().generate(driver)? % (self.max_size + 1);
        let mut bytes = Vec::with_capacity(size);
        for _ in 0..size {
            bytes.push(gen::<u8>().generate(driver)?);
        }
        Some(bytes)
    }
}

/// Generator for valid base64 strings
#[derive(Debug)]
pub struct Base64StringGenerator {
    alphabet_type: AlphabetType,
    max_size: usize,
}

impl Base64StringGenerator {
    pub fn new(alphabet_type: AlphabetType, max_size: usize) -> Self {
        Self {
            alphabet_type,
            max_size,
        }
    }

    fn get_alphabet_chars(&self) -> &[u8] {
        match &self.alphabet_type {
            AlphabetType::Standard => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
            AlphabetType::UrlSafe => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
            AlphabetType::Custom(chars) => chars,
        }
    }
}

impl ValueGenerator for Base64StringGenerator {
    type Output = String;

    fn generate<D>(&self, driver: &mut D) -> Option<Self::Output>
    where
        D: bolero_generator::driver::Driver,
    {
        let alphabet = self.get_alphabet_chars();
        let size = gen::<usize>().generate(driver)? % (self.max_size + 1);
        
        // Generate a valid base64 string by creating groups of 4 characters
        let mut result = String::new();
        let groups = size / 4;
        
        for _ in 0..groups {
            for _ in 0..4 {
                let char_index = gen::<usize>().generate(driver)? % alphabet.len();
                result.push(alphabet[char_index] as char);
            }
        }
        
        // Handle remaining characters (should be 0, 1, 2, or 3)
        let remaining = size % 4;
        for _ in 0..remaining {
            let char_index = gen::<usize>().generate(driver)? % alphabet.len();
            result.push(alphabet[char_index] as char);
        }
        
        // Add padding if needed for standard base64
        match remaining {
            2 => result.push_str("=="),
            3 => result.push('='),
            _ => {}
        }
        
        Some(result)
    }
}

/// Generator for invalid base64 inputs (for error testing)
#[derive(Debug)]
pub struct InvalidInputGenerator {
    max_size: usize,
}

impl InvalidInputGenerator {
    pub fn new(max_size: usize) -> Self {
        Self { max_size }
    }
}

impl ValueGenerator for InvalidInputGenerator {
    type Output = String;

    fn generate<D>(&self, driver: &mut D) -> Option<Self::Output>
    where
        D: bolero_generator::driver::Driver,
    {
        let size = gen::<usize>().generate(driver)? % (self.max_size + 1);
        let mut result = String::new();
        
        // Generate string with potentially invalid characters
        let invalid_chars = b"!@#$%^&*()[]{}|\\:;\"'<>?,./~`";
        let valid_chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        
        for _ in 0..size {
            let use_invalid = gen::<bool>().generate(driver)?;
            if use_invalid && !invalid_chars.is_empty() {
                let char_index = gen::<usize>().generate(driver)? % invalid_chars.len();
                result.push(invalid_chars[char_index] as char);
            } else {
                let char_index = gen::<usize>().generate(driver)? % valid_chars.len();
                result.push(valid_chars[char_index] as char);
            }
        }
        
        Some(result)
    }
}

/// Generator for test configurations
#[derive(Debug)]
pub struct ConfigurationGenerator;

impl ValueGenerator for ConfigurationGenerator {
    type Output = TestConfig;

    fn generate<D>(&self, driver: &mut D) -> Option<Self::Output>
    where
        D: bolero_generator::driver::Driver,
    {
        let alphabet = match gen::<u8>().generate(driver)? % 2 {
            0 => AlphabetType::Standard,
            1 => AlphabetType::UrlSafe,
            _ => AlphabetType::Standard,
        };
        
        let padding_mode = match gen::<u8>().generate(driver)? % 5 {
            0 => PaddingMode::Canonical,
            1 => PaddingMode::None,
            2 => PaddingMode::Indifferent,
            3 => PaddingMode::RequireCanonical,
            4 => PaddingMode::RequireNone,
            _ => PaddingMode::Canonical,
        };
        
        Some(TestConfig {
            alphabet,
            padding_mode,
            engine_type: crate::comprehensive::test_config::EngineType::GeneralPurpose,
            test_iterations: 1000,
            max_input_size: 1024,
        })
    }
}

/// Generator for custom alphabets
#[derive(Debug)]
pub struct CustomAlphabetGenerator;

impl ValueGenerator for CustomAlphabetGenerator {
    type Output = [u8; 64];

    fn generate<D>(&self, driver: &mut D) -> Option<Self::Output>
    where
        D: bolero_generator::driver::Driver,
    {
        // Start with the standard alphabet as a base
        let base_chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut chars = *base_chars;
        
        // Shuffle the characters to create a custom alphabet
        // We'll use a simple Fisher-Yates shuffle
        for i in (1..64).rev() {
            let j = gen::<usize>().generate(driver)? % (i + 1);
            chars.swap(i, j);
        }
        
        Some(chars)
    }
}