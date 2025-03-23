import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  base: './',
  plugins: [
    wasm(),
    topLevelAwait()
  ],
  optimizeDeps: {
    exclude: ['nice-key-gen']
  },
  build: {
    target: 'esnext',
    assetsInlineLimit: 0, // Prevents inlining WASM files as base64
    rollupOptions: {
      output: {
        format: 'es',
        assetFileNames: (assetInfo) => {
          // Keep original name for wasm files to help with debugging
          if (assetInfo.name.endsWith('.wasm')) {
            return 'assets/[name][extname]';
          }
          return 'assets/[name]-[hash][extname]';
        }
      }
    }
  },
  server: {
    fs: {
      allow: ['..']
    },
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp'
    }
  }
}); 