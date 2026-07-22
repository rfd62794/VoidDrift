import { defineConfig } from 'vite';

export default defineConfig({
  server: {
    port: 5000,
    host: '0.0.0.0',
    allowedHosts: 'all',
    strictPort: true,
  },
  preview: {
    port: 5000,
    host: '0.0.0.0',
    allowedHosts: 'all',
  },
});
