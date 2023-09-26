import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import * as path from 'path';
import FullReload from 'vite-plugin-full-reload';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), FullReload(['config/routes.rb', 'src/graphics/**/*.ts'])],
  server: {
    hmr: true,
  },
  resolve: {
    alias: [{ find: '@', replacement: path.resolve(__dirname, 'src') }],
  },
});
