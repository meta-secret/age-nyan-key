use age::secrecy::ExposeSecret;
use anyhow::Result;
use std::time::{Duration, Instant};
use tracing::info;

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

/// Generates an age secret key that starts with a specific pattern
pub fn generate_nice_key(config: &PatternConfig) -> Result<age::x25519::Identity> {
    let start_time = Instant::now();
    let mut attempts = 0;

    loop {
        attempts += 1;
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
                    let elapsed = start_time.elapsed();
                    info!(
                        "Found nice key after {} attempts in {:?}",
                        attempts, elapsed
                    );
                    info!("Found repeating character: {}", first_char);
                    return Ok(identity);
                }
            }
        }

        if attempts % 1_000_000 == 0 {
            println!(
                "{}. Tried {} keys so far...",
                attempts / 1_000_000,
                attempts
            );
        }

        // Optional timeout to avoid infinite loops in extreme cases
        if start_time.elapsed() > Duration::from_secs(900) {
            // 10 minute timeout
            return Err(anyhow::anyhow!(
                "Timeout reached while searching for a nice key"
            ));
        }
    }
}

/// Returns the public key corresponding to a secret key
pub fn get_public_key(identity: &age::x25519::Identity) -> String {
    identity.to_public().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generated_key_matches_pattern() {
        let config = PatternConfig {
            repeat_count: 2, // Use a small value for testing
        };

        let identity = generate_nice_key(&config).unwrap();
        let key = identity.to_string();
        let key_str = key.expose_secret();

        assert!(key_str.starts_with("AGE-SECRET-KEY-1"));

        let suffix = key_str.strip_prefix("AGE-SECRET-KEY-1").unwrap();
        let first_char = suffix.chars().next().unwrap();
        let pattern = std::iter::repeat(first_char)
            .take(config.repeat_count)
            .collect::<String>();

        assert!(suffix.starts_with(&pattern));
    }
}
