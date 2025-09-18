import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import pkg from './package.json';

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig({
  plugins: [sveltekit()],
  
  logLevel: 'warn', // Reduce verbose logging

  define: {
    __APP_VERSION__: JSON.stringify(pkg.version),
    __APP_NAME__: JSON.stringify(pkg.name),
    __APP_DESCRIPTION__: JSON.stringify(pkg.description),
    __APP_AUTHOR__: JSON.stringify(pkg.author),
    __APP_LICENSE__: JSON.stringify(pkg.license),
    __APP_REPOSITORY__: JSON.stringify(pkg.repository.url),
  },

  build: {
    chunkSizeWarningLimit: 5000, // 5MB - Tauri apps don't have web size constraints
    commonjsOptions: {
      include: [/highlight\.js/, /mark\.js/],
      transformMixedEsModules: true,
      strictRequires: false
    },
    rollupOptions: {
      onwarn(warning, warn) {
        // Suppress CommonJS resolver warnings for known packages
        if (warning.code === 'PLUGIN_WARNING' && 
            warning.message.includes('commonjs--resolver') &&
            warning.message.includes('resolveId')) {
          return;
        }
        warn(warning);
      },
      plugins: [
        {
          name: 'silence-commonjs-warnings',
          buildStart() {
            // Monkey patch console.warn to filter out CommonJS resolver warnings
            const originalWarn = console.warn;
            console.warn = function(...args) {
              const message = args.join(' ');
              if (message.includes('[plugin commonjs--resolver]') && 
                  message.includes('resolveId')) {
                return; // Skip this warning
              }
              originalWarn.apply(console, args);
            };
          }
        }
      ]
    }
  },

  optimizeDeps: {
    include: ['highlight.js', 'mark.js']
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
});
