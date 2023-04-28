pub mod convert;
pub mod generate;
pub mod namespaces;
pub mod valid;

use uuid::fmt::Hyphenated;

const HYPHENATED_UUID_LEN: usize = Hyphenated::LENGTH;
const HYPHENATED_UUID_LEN_U64: u64 = HYPHENATED_UUID_LEN as u64;
const UUID_BYTES_LEN: usize = 16;
const UUID_BYTES_LEN_U64: u64 = 16;

/// Validate arg count; return a formatted message if not
pub fn validate_arg_count(count: usize, expected: usize, fn_name: &str) -> Result<(), String> {
    if count != expected {
        let pluralized = if expected == 1 {
            "argument"
        } else {
            "arguments"
        };

        Err(format!(
            "{fn_name} takes {expected} {pluralized} but got {count}"
        ))
    } else {
        Ok(())
    }
}
