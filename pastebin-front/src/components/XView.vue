<template>
    <div class="view">
        <div class="view-title" v-if="hasTitle">
            {{ props.record.title }}
        </div>
        <div class="view-content line-numbers" ref="viewContentDiv">
            <pre><code :class="prismClass">{{ props.record.content }}</code></pre>
        </div>
    </div>
</template>

<style>
/* 修改 prismjs theme "coy" */
pre[class*="language-"]::after,
pre[class*="language-"]::before {
    box-shadow: none !important;
    display: none;
}
</style>

<style scoped>
.view-title {
    width: 100%;
    font-size: 2em;
    font-weight: 500;
    margin: 0;
    margin-bottom: 0.5em;
    text-align: center;
}

.view-content {
    width: 100%;
}
</style>

<script setup lang="ts">
import type { PastebinRecord } from "@/data/dto";
import { computed, ref, watch } from "vue";
import prismjs from "prismjs";

const props = defineProps<{
    record: PastebinRecord;
}>();

const hasTitle = computed(() => props.record.title !== "");

const prismClass = computed(() => `language-${props.record.lang}`);

const viewContentDiv = ref<HTMLDivElement | null>(null);
const highlight = () => {
    const div = viewContentDiv.value;
    if (div) {
        prismjs.highlightAllUnder(div);
    }
};

watch(
    () => props.record.content,
    (_newVal, _oldVal, onCleanup) => {
        const timer = setTimeout(highlight, 60);
        onCleanup(() => clearTimeout(timer));
    },
    { immediate: true, flush: "post" }
);
</script>
