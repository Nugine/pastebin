<template>
    <div class="code-area line-numbers" style="width: 100%" ref="contentDiv">
        <pre><code :class="prismClass">{{ props.content }}</code></pre>
    </div>
</template>

<script setup lang="ts">
import { computed, ref, toRef } from "vue";
import { useHighlight } from "@/hooks/useHighlight";

const props = defineProps<{
    lang: string;
    content: string;
}>();

const prismClass = computed(() => `language-${props.lang}`);

const contentDiv = ref<HTMLDivElement | null>(null);
useHighlight(contentDiv, toRef(props, "content"));
</script>
