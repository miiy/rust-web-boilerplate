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
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
        // rewrite: (path) => path.replace(/^\/api/, '')  // 如果需要去掉 /api 前缀
      }
    }
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
        base: 'src/base.js',
        index: 'src/index.js',
        about: 'src/about.js',
        register: 'src/register.js',
        login: 'src/login.js',
      },
      output: {
        compact: true,
        entryFileNames: 'assets/js/[name]-[hash].js',
        chunkFileNames: 'assets/js/[name]-[hash].js',
        assetFileNames: 'assets/[ext]/[name]-[hash].[ext]',
        manualChunks: (id) => {
          // console.log(id)
          if (id.includes("modulepreload-polyfill.js") || id.includes("plugin-vue") || id.includes("node_modules")) {
            return 'vendor'
          }
        },
      }
    }
  }
})
