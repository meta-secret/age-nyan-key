use age::secrecy::ExposeSecret;
use anyhow::{anyhow, Result};

// Add wasm module for WebAssembly integration
pub mod wasm;

/// Represents the pattern requirements for a "nice" key
pub struct PatternConfig {
    /// Number of consecutive repeating characters required
    pub repeat_count: usize,
}

impl Default for PatternConfig {
    fn default() -> Self {
        Self { repeat_count: 4 }
    }
}

/// Generate a nice Age key that has a specific number of consecutive repeating characters
/// in the encoded secret key.
pub fn generate_nice_key(config: &PatternConfig) -> Result<age::x25519::Identity> {
    if config.repeat_count > 10 {
        return Err(anyhow!(
            "Consecutive characters more than 10 is not supported to avoid excessive wait times"
        ));
    }

    let mut attempts = 0;

    loop {
        attempts += 1;

        // Generate a new key pair
        let identity = age::x25519::Identity::generate();

        // Convert the identity to a string and check if it matches our pattern
        let secret_key = identity.to_string();
        let secret_key_str = secret_key.expose_secret();

        // Check if it matches our pattern (we know age keys start with "AGE-SECRET-KEY-1")
        if let Some(suffix) = secret_key_str.strip_prefix("AGE-SECRET-KEY-1") {
            if let Some(chars) = suffix
                .chars()
                .take(config.repeat_count)
                .collect::<Vec<_>>()
                .first()
            {
                let first_char = *chars;
                if suffix
                    .chars()
                    .take(config.repeat_count)
                    .all(|c| c == first_char)
                {
                    return Ok(identity);
                }
            }
        }
        
        // No timeout checks, just keep trying
    }
}

/// Returns the public key corresponding to a secret key
pub fn get_public_key(identity: &age::x25519::Identity) -> String {
    identity.to_public().to_string()
}
