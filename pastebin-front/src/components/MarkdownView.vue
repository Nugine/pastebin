<template>
    <div style="width: 100%" v-html="contentHTML" ref="contentDiv"></div>
</template>

<script setup lang="ts">
import MarkdownIt from "markdown-it";
import { computed, ref, toRef } from "vue";
import { useHighlight } from "@/hooks/useHighlight";
import { useKatex } from "@/hooks/useKatex";

const props = defineProps<{
    content: string;
}>();

const markdown = new MarkdownIt();
const contentHTML = computed(() => markdown.render(props.content));

const contentDiv = ref<HTMLDivElement | null>(null);
const contentRef = toRef(props, "content");
useHighlight(contentDiv, contentRef);
useKatex(contentDiv, contentRef);
</script>
