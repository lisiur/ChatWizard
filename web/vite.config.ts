import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vueJsx from "@vitejs/plugin-vue-jsx";

export default defineConfig({
  plugins: [vue(), vueJsx({})],

  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    proxy: {
      "/api": {
        target: "http://127.0.0.1:23333",
        changeOrigin: true,
        ws: true,
        preserveHeaderKeyCase: true,
        configure(proxy, options) {
          proxy.on("proxyReq", (proxyReq) => {
            const method = proxyReq.method;
            const url = proxyReq.path;
            if (proxyReq.getHeader("authorization")) {
              proxyReq.setHeader(
                "Authorization",
                proxyReq.getHeader("authorization")!
              );
            }
            console.log(`${method} ${url}`);
          });
        },
      },
    },
  },
  envPrefix: ["VITE_", "TAURI_"],
  define: {
    __VUE_I18N_FULL_INSTALL__: true,
    __VUE_I18N_LEGACY_API__: true,
  },
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});
