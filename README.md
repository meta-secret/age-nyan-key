# Nice Age Key Generator

A tool for generating age encryption keys with nice-looking patterns.

## Features

- Generates age secret keys that have specific patterns of repeating characters
- Configurable repeat count
- Provides both a library and a command-line tool

## Usage

### Command-line

```bash
# Generate a key with 4 consecutive repeating characters
nice-key-gen

# Generate a key with 3 consecutive repeating characters
nice-key-gen --repeat-count 3
```

### Library

```rust
use nice_key_gen::{generate_nice_key, get_public_key, PatternConfig};

// Create a custom pattern configuration
let config = PatternConfig {
    repeat_count: 5,
};

// Generate a key matching the pattern
let identity = generate_nice_key(&config)?;
println!("Secret key: {}", identity);
println!("Public key: {}", get_public_key(&identity));
```

## How it works

The tool repeatedly generates random age keys until it finds one with a pattern of repeating characters directly after the "AGE-SECRET-KEY-1" prefix. Any character can be the repeating character - the tool finds keys where the first N characters after the prefix are all the same. 