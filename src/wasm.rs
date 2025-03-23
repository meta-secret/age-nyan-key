use wasm_bindgen::prelude::*;
use crate::{generate_nice_key, get_public_key, PatternConfig};
use age::secrecy::ExposeSecret;

// Initialize panic hook for better JS error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct KeyResult {
    secret_key: String,
    public_key: String,
    pattern_char: String,
}

#[wasm_bindgen]
impl KeyResult {
    #[wasm_bindgen(getter)]
    pub fn secret_key(&self) -> String {
        self.secret_key.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> String {
        self.public_key.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn pattern_char(&self) -> String {
        self.pattern_char.clone()
    }
}

#[wasm_bindgen]
pub fn generate_key(repeat_count: usize) -> Result<KeyResult, JsError> {
    let config = PatternConfig { repeat_count };
    
    match generate_nice_key(&config) {
        Ok(identity) => {
            let secret_key = identity.to_string();
            let secret_key_str = secret_key.expose_secret().to_string();
            let public_key = get_public_key(&identity);
            
            let suffix = secret_key_str.strip_prefix("AGE-SECRET-KEY-1").unwrap();
            let pattern_char = suffix.chars().next().unwrap().to_string();
            
            Ok(KeyResult {
                secret_key: secret_key_str,
                public_key,
                pattern_char,
            })
        },
        Err(err) => Err(JsError::new(&err.to_string())),
    }
} 