import * as wasm from 'nice-key-gen';

// Initialize WASM module
async function init() {
  try {
    await wasm.default();
    console.log('WASM initialized successfully');
    document.getElementById('loading-status').textContent = 'WASM module loaded successfully';
    document.getElementById('generate-button').disabled = false;
  } catch (error) {
    console.error('WASM initialization error:', error);
    document.getElementById('loading-status').textContent = 'Failed to load WASM module';
    document.getElementById('error-message').textContent = `Error: ${error.message}`;
    document.getElementById('error-message').style.display = 'block';
  }
}

// Generate key function
function generateKey() {
  const countInput = document.getElementById('count-input');
  const errorMessage = document.getElementById('error-message');
  const keyDisplay = document.getElementById('key-display');
  
  errorMessage.textContent = '';
  errorMessage.style.display = 'none';
  
  try {
    const repeatCount = parseInt(countInput.value, 10);
    
    if (isNaN(repeatCount) || repeatCount < 1 || repeatCount > 10) {
      errorMessage.textContent = 'Please enter a valid number between 1 and 10';
      errorMessage.style.display = 'block';
      return;
    }
    
    const key = wasm.generate_key(repeatCount);
    
    if (key) {
      document.getElementById('secret-key').textContent = key.secret_key;
      document.getElementById('public-key').textContent = key.public_key;
      document.getElementById('pattern-char').textContent = key.pattern_char;
      document.getElementById('repeating-pattern').textContent = key.pattern_char.repeat(repeatCount);
      keyDisplay.style.display = 'block';
    }
  } catch (error) {
    console.error('Key generation error:', error);
    errorMessage.textContent = `Error generating key: ${error.message}`;
    errorMessage.style.display = 'block';
  }
}

// Set up event listeners when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  init();
  document.getElementById('generate-button').addEventListener('click', generateKey);
}); 