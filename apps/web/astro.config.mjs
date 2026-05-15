// @ts-check
import { defineConfig } from 'astro/config';

// https://astro.build/config
export default defineConfig({
  output: 'static',
  build: {
    format: 'file' // Asegura que los archivos se generen como index.html para compatibilidad con WebView
  }
});