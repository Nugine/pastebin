import { ref, watch, type Ref } from "vue";
import type { RenderMathInElementOptions } from "katex/contrib/auto-render";
import type renderMathInElement from "katex/contrib/auto-render";

import "katex/dist/katex.min.css";

const katexOptions: RenderMathInElementOptions = {
    delimiters: [
        { left: "$$", right: "$$", display: true },
        { left: "\\(", right: "\\)", display: false },
        { left: "\\[", right: "\\]", display: true },
        { left: "$", right: "$", display: false },
    ],
    errorColor: "#cc0000",
    throwOnError: false,
    strict: "ignore",
};

export function useKatex(div: Ref<HTMLDivElement | null>, content: Ref<string>) {
    const katex = ref<typeof renderMathInElement | null>(null);

    import("katex/contrib/auto-render").then((module) => {
        katex.value = module.default;
    });

    const render = () => {
        const e = div.value;
        const f = katex.value;
        if (e && f) {
            f(e, katexOptions);
        }
    };

    watch(
        [content, katex],
        (_newVal, _oldVal, onCleanup) => {
            const timer = setTimeout(render, 60);
            onCleanup(() => clearTimeout(timer));
        },
        { immediate: true, flush: "post" }
    );
}
