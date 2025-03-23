use clap::Parser;
use nice_key_gen::{generate_nice_key, get_public_key, PatternConfig};
use std::process;
use age::secrecy::ExposeSecret;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate nice-looking age keys", long_about = None)]
struct Args {
    /// Number of consecutive repeating characters to find
    #[arg(short = 'n', long, default_value_t = 4)]
    repeat_count: usize,
}

fn main() {
    let args = Args::parse();
    
    let config = PatternConfig {
        repeat_count: args.repeat_count,
    };
    
    println!("Generating an age key that starts with {} consecutive repeating characters...", 
        config.repeat_count);
    println!("This may take some time depending on the pattern...");
    
    match generate_nice_key(&config) {
        Ok(identity) => {
            let secret_key = identity.to_string();
            let secret_key_str = secret_key.expose_secret();
            let suffix = secret_key_str.strip_prefix("AGE-SECRET-KEY-1").unwrap();
            let repeating_char = suffix.chars().next().unwrap();
            
            println!("\nSuccess! Found a matching key:");
            println!("Repeating character: '{}'", repeating_char);
            println!("Secret key: {}", secret_key.expose_secret());
            println!("Public key: {}", get_public_key(&identity));
        },
        Err(e) => {
            eprintln!("Error generating key: {}", e);
            process::exit(1);
        }
    }
} 