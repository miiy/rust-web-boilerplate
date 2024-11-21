import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  server: {
    port: 8081,
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
  build: {
    manifest: true,
    rollupOptions: {
      input: {
        main: 'src/main.js',
        home: 'home/index.html',
        about: 'about/index.html',
      },
      output: {
        compact: true,
        entryFileNames: 'assets/js/[name]-[hash].js',
        chunkFileNames: 'assets/js/[name]-[hash].js',
        assetFileNames: 'assets/[ext]/[name]-[hash].[ext]',
        manualChunks: (id) => {
          if (id.includes("modulepreload-polyfill")) {
            return 'vendor'
          }
          if (id.includes("plugin-vue")) {
            return 'vendor'
          }
          if (id.includes('node_modules')) {
            return 'vendor'
          }
        },
      }
    }
  }
})
