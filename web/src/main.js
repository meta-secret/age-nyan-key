import * as wasm from 'nice-key-gen';

// Initialize WASM module
async function init() {
  try {
    // Try standard initialization first
    await wasm.default();
    console.log('WASM initialized successfully');
    document.getElementById('loading-status').textContent = 'WASM module loaded successfully';
    document.getElementById('generate-button').disabled = false;
  } catch (error) {
    console.error('WASM initialization error:', error);
    
    // Simpler fallback: Try direct fetch with correct MIME type
    try {
      document.getElementById('loading-status').textContent = 'Trying fallback WASM loading...';
      
      // Find the correct WASM file name by looking for script tags that reference it
      const scripts = document.querySelectorAll('script');
      let wasmPath = '';
      
      for (const script of scripts) {
        const src = script.getAttribute('src') || '';
        if (src.includes('index-') && src.endsWith('.js')) {
          // Found the main JS file, now load it to find WASM references
          const response = await fetch(src);
          const text = await response.text();
          
          // Look for WASM file pattern in the JS
          const match = text.match(/nice_key_gen_bg-[A-Za-z0-9]+\.wasm/);
          if (match) {
            wasmPath = `./assets/${match[0]}`;
            console.log('Found WASM path:', wasmPath);
            break;
          }
        }
      }
      
      if (!wasmPath) {
        // Fallback to a guess if we couldn't find it
        wasmPath = './assets/nice_key_gen_bg-HGZLgYOu.wasm';
      }
      
      const response = await fetch(wasmPath, {
        headers: {
          'Content-Type': 'application/wasm'
        }
      });
      
      if (!response.ok) {
        throw new Error(`Failed to fetch WASM file: ${response.status} ${response.statusText}`);
      }
      
      // We don't actually need to instantiate it here, we just want to check if we can access it
      await response.arrayBuffer();
      
      // Try standard initialization again after verifying the file is accessible
      await wasm.default();
      
      console.log('WASM loaded via fallback method');
      document.getElementById('loading-status').textContent = 'WASM module loaded via fallback';
      document.getElementById('generate-button').disabled = false;
    } catch (fallbackError) {
      console.error('Fallback WASM initialization failed:', fallbackError);
      document.getElementById('loading-status').textContent = 'Failed to load WASM module';
      document.getElementById('error-message').textContent = `Error: ${error.message}. Fallback error: ${fallbackError.message}`;
      document.getElementById('error-message').style.display = 'block';
    }
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