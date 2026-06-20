import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

// Tauri expects a fixed dev port and serves the built assets relatively.
export default defineConfig({
    plugins: [vue()],
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
    },
    build: {
        target: 'es2021',
        outDir: 'dist',
    },
});
