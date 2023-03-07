<template>
    <div class="line-numbers" style="width: 100%" ref="contentDiv">
        <pre><code :class="prismClass">{{ props.content }}</code></pre>
    </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import prismjs from "prismjs";

const props = defineProps<{
    lang: string;
    content: string;
}>();

const contentDiv = ref<HTMLDivElement | null>(null);
const prismClass = computed(() => `language-${props.lang}`);

const highlight = () => {
    const div = contentDiv.value;
    if (div) {
        prismjs.highlightAllUnder(div);
    }
};

watch(
    () => props.content,
    (_newVal, _oldVal, onCleanup) => {
        const timer = setTimeout(highlight, 60);
        onCleanup(() => clearTimeout(timer));
    },
    { immediate: true, flush: "post" }
);
</script>
