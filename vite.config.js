import { defineConfig } from "vite";
import wasmPack from "vite-plugin-wasm-pack";
import vue from "@vitejs/plugin-vue";
import postcssMixins from "postcss-mixins";
import postcssUtilities from "postcss-utilities";
import postcssNesting from "postcss-nesting";


export default defineConfig({
    worker: {
        format: "es",
    },
    plugins: [
        vue(),
        wasmPack(["./crate"])
    ],
    css: {
        postcss: {
            plugins: [
                postcssMixins,
                postcssUtilities,
                postcssNesting
            ],
        },
    },
});
