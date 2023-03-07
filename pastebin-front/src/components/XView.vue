<template>
    <div class="view">
        <div class="view-title" v-if="hasTitle">
            {{ props.record.title }}
        </div>
        <div class="view-content">
            {{ props.record.content }}
        </div>
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

.view-content {
    width: 100%;
}
</style>

<script setup lang="ts">
import type { PastebinRecord } from "@/data/dto";
import { computed } from "vue";

const props = defineProps<{
    record: PastebinRecord;
}>();

const hasTitle = computed(() => props.record.title !== "");
</script>
