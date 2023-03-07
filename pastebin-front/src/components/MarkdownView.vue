<template>
    <div class="line-numbers" style="width: 100%" v-html="contentHTML"></div>
</template>

<script setup lang="ts">
import MarkdownIt from "markdown-it";
import { computed } from "vue";

const props = defineProps<{
    content: string;
}>();

const markdown = new MarkdownIt();

const contentHTML = computed(() => markdown.render(props.content));
</script>
