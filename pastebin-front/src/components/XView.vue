<template>
    <div class="view">
        <div class="view-title" v-if="hasTitle">
            {{ props.record.title }}
        </div>
        <MarkdownView v-if="isMarkdown" :content="props.record.content" />
        <CodeView v-else :lang="props.record.lang" :content="props.record.content" />
    </div>
</template>

<style scoped>
.view-title {
    width: 100%;
    font-size: 2em;
    font-weight: 500;
    margin: 0;
    margin-bottom: 0.5em;
    text-align: center;
}
</style>

<script setup lang="ts">
import type { PastebinRecord } from "@/data/dto";
import { computed } from "vue";
import CodeView from "./CodeView.vue";
import MarkdownView from "./MarkdownView.vue";

const props = defineProps<{
    record: PastebinRecord;
}>();

const hasTitle = computed(() => props.record.title !== "");
const isMarkdown = computed(() => props.record.lang === "markdown");
</script>
