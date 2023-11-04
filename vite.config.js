import {defineConfig} from "vite";

export default defineConfig({
    build: {
        rollupOptions: {
            output: {
                entryFileNames: '[name].[hash].js',
                chunkFileNames: '[name].[hash].js'
            }
        }
    }
})