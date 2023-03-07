import { fileURLToPath, URL } from "node:url";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vueJsx from "@vitejs/plugin-vue-jsx";

import prismjs from "vite-plugin-prismjs";
import { LANGUAGES } from "./prismjs.custom";

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        vue(),
        vueJsx(),
        prismjs({
            languages: LANGUAGES.map((tuple) => tuple[0]),
            plugins: ["line-numbers"],
            theme: "coy",
            css: true,
        }),
    ],
    resolve: {
        alias: {
            "@": fileURLToPath(new URL("./src", import.meta.url)),
        },
    },
    server: {
        port: 3000,
    },
    preview: {
        port: 3000,
    },
});
