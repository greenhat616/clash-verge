import react from '@vitejs/plugin-react';
import path from 'path';
import { defineConfig } from 'vite';
import monaco from 'vite-plugin-monaco-editor';
import svgr from 'vite-plugin-svgr';
// console.log(monaco);

// https://vitejs.dev/config/
export default defineConfig({
  root: 'src',
  server: { port: 3000 },
  plugins: [
    svgr(),
    react(),
    // @ts-expect-error due to vite-plugin-monaco-editor typing is wrong
    monaco.default({ languageWorkers: ['editorWorkerService', 'typescript'] }),
  ],
  build: {
    outDir: '../dist',
    emptyOutDir: true,
  },
  resolve: {
    alias: {
      '@': path.resolve('./src'),
      '@root': path.resolve('.'),
    },
  },
  define: {
    OS_PLATFORM: `"${process.platform}"`,
    WIN_PORTABLE: !!process.env.VITE_WIN_PORTABLE,
  },
});
