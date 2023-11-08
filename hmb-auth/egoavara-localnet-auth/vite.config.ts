import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import vuetify from 'vite-plugin-vuetify'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        vue(),
        vueJsx(),
        vuetify(),
    ],
    resolve: {
        alias: {
            '@': fileURLToPath(new URL('./src', import.meta.url))
        }
    },
    build: {
        outDir: '../public'
    },
    worker: {
        format: 'iife'
    },
    server: {
        host: true,
        port: 5173,
        strictPort: true,
        https: false,
        proxy: {
            '^/api/.*': {
                target: 'http://localhost:8080',
                changeOrigin: false
            },
            '^/flow/.*': {
                target: 'http://localhost:8080',
                changeOrigin: false
            },
            '^/self-service/.*': {
                target: 'http://localhost:8045',
                changeOrigin: false
            }
        },
        watch: {
            usePolling: true
        }
    }
})
