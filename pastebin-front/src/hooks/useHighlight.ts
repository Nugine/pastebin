import { watch, type Ref } from "vue";
import prismjs from "prismjs";

export function useHighlight(div: Ref<HTMLDivElement | null>, content: Ref<string>) {
    const highlight = () => {
        const e = div.value;
        if (e) {
            prismjs.highlightAllUnder(e);
        }
    };

    watch(
        content,
        (_newVal, _oldVal, onCleanup) => {
            const timer = setTimeout(highlight, 60);
            onCleanup(() => clearTimeout(timer));
        },
        { immediate: true, flush: "post" }
    );
}
