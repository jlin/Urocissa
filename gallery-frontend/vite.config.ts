import { resolve } from 'path'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src')
    }
  },
  build: {
    rollupOptions: {
      input: {
        app: './index.html',
        login: './login.html'
      }
    }
  },
  server: {
    proxy: {
      '/json': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true
      },
      '/assets': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true
      },
      '/put': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true
      },
      '/delete': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true
      },
      '/edit_album': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true
      },
      '/edit_sync_path': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true
      },
      '/edit_priority_list': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true
      },
      '/import_path': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true
      },
      '/upload': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true
      },
      '/create_album': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true
      },
      '/query': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true
      },
      '/get': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true
      },
      '/object': {
        target: 'http://127.0.0.1:4000',
        changeOrigin: true
      }
    }
  }
})
