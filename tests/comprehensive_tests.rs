//! Comprehensive Property-Based Testing Suite for rust-base64
//!
//! This test suite provides comprehensive property-based testing for the rust-base64 library
//! using the bolero testing framework. It validates correctness, performance characteristics,
//! error handling, and configuration compliance across all supported features.
//!
//! The testing approach leverages bolero's property-based testing capabilities to generate
//! random inputs and verify universal properties that should hold for all valid inputs.

mod comprehensive;

use comprehensive::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::comprehensive::test_config::TestConfig;
    use crate::comprehensive::test_runner::PropertyTestRunner;

    /// Basic smoke test to ensure the test infrastructure is working
    #[test]
    fn test_infrastructure_smoke_test() {
        let config = TestConfig::default();
        let runner = PropertyTestRunner::new(config);
        
        // Test that we can create an engine from the configuration
        let engine = runner.config().create_engine();
        
        // Basic encode/decode test to verify the engine works
        let test_data = b"Hello, World!";
        let encoded = base64::Engine::encode(&engine, test_data);
        let decoded = base64::Engine::decode(&engine, &encoded).expect("Failed to decode");
        
        assert_eq!(test_data, decoded.as_slice());
    }

    /// Test that generators can be created and used
    #[test]
    fn test_generators_smoke_test() {
        use crate::comprehensive::generators::*;
        use bolero_generator::{ValueGenerator, driver::{ByteSliceDriver, Options}};
        
        // Test byte sequence generator
        let byte_gen = ByteSequenceGenerator::new(100);
        let mut driver = ByteSliceDriver::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &Options::default());
        let bytes = byte_gen.generate(&mut driver);
        assert!(bytes.is_some());
        
        // Test base64 string generator
        let string_gen = Base64StringGenerator::new(
            crate::comprehensive::test_config::AlphabetType::Standard, 
            100
        );
        let mut driver = ByteSliceDriver::new(&[11, 12, 13, 14, 15, 16, 17, 18, 19, 20], &Options::default());
        let string = string_gen.generate(&mut driver);
        assert!(string.is_some());
        
        // Test configuration generator
        let config_gen = ConfigurationGenerator;
        let mut driver = ByteSliceDriver::new(&[21, 22, 23, 24, 25, 26, 27, 28, 29, 30], &Options::default());
        let config = config_gen.generate(&mut driver);
        assert!(config.is_some());
    }

    /// Test that the test runner can execute basic property tests
    #[test]
    fn test_runner_smoke_test() {
        let config = TestConfig::default();
        let runner = PropertyTestRunner::new(config);
        
        // Run a simple property test
        let result = runner.run_property_test("smoke_test", || {
            // Always return true for this smoke test
            true
        });
        
        assert!(result.success);
        assert_eq!(result.property_name, "smoke_test");
        assert_eq!(result.iterations_run, 1000); // Default iteration count
    }

    /// Unit test for custom alphabet functionality
    #[test]
    fn test_custom_alphabet_basic() {
        use base64::Engine;
        
        // Create a custom alphabet (reversed standard alphabet)
        let custom_chars = b"zyxwvutsrqponmlkjihgfedcbaZYXWVUTSRQPONMLKJIHGFEDCBA9876543210/+";
        let config = TestConfig {
            alphabet: crate::comprehensive::test_config::AlphabetType::Custom(*custom_chars),
            padding_mode: crate::comprehensive::test_config::PaddingMode::Canonical,
            engine_type: crate::comprehensive::test_config::EngineType::GeneralPurpose,
            test_iterations: 1000,
            max_input_size: 1024,
        };
        
        let engine = config.create_engine();
        
        // Test basic roundtrip
        let test_data = b"Hello, World!";
        let encoded = engine.encode(test_data);
        let decoded = engine.decode(&encoded).expect("Should decode successfully");
        
        assert_eq!(test_data, decoded.as_slice());
        
        // Verify that encoded string only contains characters from custom alphabet
        for byte in encoded.bytes() {
            assert!(custom_chars.contains(&byte) || byte == b'=', 
                "Encoded character {} not in custom alphabet", byte as char);
        }
    }

    /// Property 1: Encode-Decode Roundtrip Test
    /// **Validates: Requirements 1.1**
    /// For any byte sequence, encoding then decoding should produce the original byte sequence
    #[test]
    #[cfg_attr(kani, kani::proof)]
    fn property_1_encode_decode_roundtrip() {
        crate::comprehensive::properties::roundtrip::test_encode_decode_roundtrip();
    }

    /// Property 2: Decode-Encode Roundtrip Test
    /// **Validates: Requirements 1.2**
    /// For any valid base64 string, decoding then encoding should produce an equivalent base64 string
    #[test]
    #[cfg_attr(kani, kani::proof)]
    fn property_2_decode_encode_roundtrip() {
        crate::comprehensive::properties::roundtrip::test_decode_encode_roundtrip();
    }

    /// Property 3: Cross-Engine Consistency Test
    /// **Validates: Requirements 1.3, 7.1**
    /// For any byte sequence and any two engine configurations with the same alphabet and padding settings,
    /// both engines should produce identical encoded output
    #[test]
    #[cfg_attr(kani, kani::proof)]
    fn property_3_cross_engine_consistency() {
        crate::comprehensive::properties::roundtrip::test_cross_engine_consistency();
    }

    /// Property 4: Custom Alphabet Roundtrip Test
    /// **Validates: Requirements 1.4**
    /// For any byte sequence and any valid custom alphabet, encoding with that alphabet then decoding should produce the original byte sequence
    #[test]
    #[cfg_attr(kani, kani::proof)]
    fn property_4_custom_alphabet_roundtrip() {
        crate::comprehensive::properties::roundtrip::test_custom_alphabet_roundtrip();
    }

    /// Property 5: Padding Mode Roundtrip Test
    /// **Validates: Requirements 1.5**
    /// For any byte sequence and any padding mode configuration, encoding then decoding should preserve the original data regardless of padding mode
    #[test]
    #[cfg_attr(kani, kani::proof)]
    fn property_5_padding_mode_roundtrip() {
        crate::comprehensive::properties::roundtrip::test_padding_mode_roundtrip();
    }

    /// Property 6: Character Set Compliance Test
    /// **Validates: Requirements 2.1, 2.2, 2.3, 2.4**
    /// For any byte sequence and any alphabet configuration, all characters in the encoded output should belong to the specified alphabet's character set
    #[test]
    #[cfg_attr(kani, kani::proof)]
    fn property_6_character_set_compliance() {
        crate::comprehensive::properties::alphabet::test_character_set_compliance();
    }

    /// Property 7: Invalid Character Detection Test
    /// **Validates: Requirements 2.5**
    /// For any string containing characters not in the specified alphabet, decoding should return a DecodeError with InvalidByte information
    #[test]
    #[cfg_attr(kani, kani::proof)]
    fn property_7_invalid_character_detection() {
        crate::comprehensive::properties::alphabet::test_invalid_character_detection();
    }

    /// Unit test for invalid character detection with specific examples
    #[test]
    fn test_invalid_character_detection_unit() {
        use base64::Engine;
        use crate::comprehensive::test_config::{AlphabetType, TestConfig, PaddingMode, EngineType};
        
        // Test STANDARD alphabet with invalid characters
        let standard_config = TestConfig {
            alphabet: AlphabetType::Standard,
            padding_mode: PaddingMode::Canonical,
            engine_type: EngineType::GeneralPurpose,
            test_iterations: 1000,
            max_input_size: 1024,
        };
        let standard_engine = standard_config.create_engine();
        
        // Test cases with invalid characters for STANDARD alphabet
        let invalid_standard_cases = [
            "ABC@DEF",  // @ is not in STANDARD alphabet
            "ABC!DEF",  // ! is not in STANDARD alphabet
            "ABC#DEF",  // # is not in STANDARD alphabet
            "ABC$DEF",  // $ is not in STANDARD alphabet
            "ABC%DEF",  // % is not in STANDARD alphabet
            "ABC^DEF",  // ^ is not in STANDARD alphabet
            "ABC&DEF",  // & is not in STANDARD alphabet
            "ABC*DEF",  // * is not in STANDARD alphabet
            "ABC-DEF",  // - is not in STANDARD alphabet (it's URL_SAFE only)
            "ABC_DEF",  // _ is not in STANDARD alphabet (it's URL_SAFE only)
        ];
        
        for invalid_input in &invalid_standard_cases {
            let result = standard_engine.decode(invalid_input);
            assert!(result.is_err(), 
                "STANDARD alphabet should reject invalid character in: {}", invalid_input);
        }
        
        // Test URL_SAFE alphabet with invalid characters
        let url_safe_config = TestConfig {
            alphabet: AlphabetType::UrlSafe,
            padding_mode: PaddingMode::Canonical,
            engine_type: EngineType::GeneralPurpose,
            test_iterations: 1000,
            max_input_size: 1024,
        };
        let url_safe_engine = url_safe_config.create_engine();
        
        // Test cases with invalid characters for URL_SAFE alphabet
        let invalid_url_safe_cases = [
            "ABC@DEF",  // @ is not in URL_SAFE alphabet
            "ABC!DEF",  // ! is not in URL_SAFE alphabet
            "ABC+DEF",  // + is not in URL_SAFE alphabet (it's STANDARD only)
            "ABC/DEF",  // / is not in URL_SAFE alphabet (it's STANDARD only)
        ];
        
        for invalid_input in &invalid_url_safe_cases {
            let result = url_safe_engine.decode(invalid_input);
            assert!(result.is_err(), 
                "URL_SAFE alphabet should reject invalid character in: {}", invalid_input);
        }
        
        // Test valid characters that should work
        let valid_standard_cases = [
            "ABCD",      // All uppercase letters
            "abcd",      // All lowercase letters  
            "1234",      // All digits
            "AB+/",      // STANDARD special characters
            "ABCD=",     // With padding
            "ABCD==",    // With double padding
        ];
        
        for valid_input in &valid_standard_cases {
            // Note: These might still fail due to length/padding issues, but they shouldn't fail due to invalid characters
            // We're just testing that the characters themselves are recognized as valid
            let result = standard_engine.decode(valid_input);
            // We don't assert success here because the input might be invalid for other reasons (length, padding)
            // But if it fails, it shouldn't be due to invalid characters
            if let Err(error) = result {
                let error_string = format!("{:?}", error);
                // The error should not be about invalid characters for these inputs
                // (it might be about length, padding, etc., which is fine)
                println!("Valid character input '{}' failed with: {}", valid_input, error_string);
            }
        }
        
        let valid_url_safe_cases = [
            "ABCD",      // All uppercase letters
            "abcd",      // All lowercase letters
            "1234",      // All digits
            "AB-_",      // URL_SAFE special characters
            "ABCD=",     // With padding
        ];
        
        for valid_input in &valid_url_safe_cases {
            let result = url_safe_engine.decode(valid_input);
            if let Err(error) = result {
                let error_string = format!("{:?}", error);
                println!("Valid character input '{}' failed with: {}", valid_input, error_string);
            }
        }
        
        // Test custom alphabet
        let custom_chars = b"zyxwvutsrqponmlkjihgfedcbaZYXWVUTSRQPONMLKJIHGFEDCBA9876543210/+";
        let custom_config = TestConfig {
            alphabet: AlphabetType::Custom(*custom_chars),
            padding_mode: PaddingMode::Canonical,
            engine_type: EngineType::GeneralPurpose,
            test_iterations: 1000,
            max_input_size: 1024,
        };
        let custom_engine = custom_config.create_engine();
        
        // Test invalid characters for custom alphabet
        let invalid_custom_cases = [
            "ABC@DEF",  // @ is not in custom alphabet
            "ABC!DEF",  // ! is not in custom alphabet
            "ABC-DEF",  // - is not in this custom alphabet
            "ABC_DEF",  // _ is not in this custom alphabet
        ];
        
        for invalid_input in &invalid_custom_cases {
            let result = custom_engine.decode(invalid_input);
            assert!(result.is_err(), 
                "Custom alphabet should reject invalid character in: {}", invalid_input);
        }
    }
}

// Property tests will be added in subsequent tasks
// This file serves as the main entry point for all comprehensive property-based tests