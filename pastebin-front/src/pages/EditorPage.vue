<template>
    <div class="btn-bar">
        <button class="btn" type="button">Edit</button>
        <button class="btn" type="button">Preview</button>
        <button class="btn" type="button">Copy</button>
        <button class="btn" type="button">Paste!</button>
    </div>
    <form autocomplete="off">
        <div class="form-group">
            <label>Title</label>
            <input class="form-control" type="text" />
        </div>
        <div class="form-group">
            <label>Language</label>
            <select class="form-control">
                <option>test</option>
            </select>
        </div>
        <div class="form-group">
            <label>Expiration</label>
            <select class="form-control">
                <option>test</option>
            </select>
        </div>
        <div class="form-group">
            <label>Content</label>
            <textarea class="form-control" rows="15" required ref="contentTextArea" />
        </div>
    </form>
</template>

<style scoped>
form {
    width: 100%;
    flex-grow: 1;
}
</style>

<script setup lang="ts">
import { onMounted, ref } from "vue";

const contentTextArea = ref<HTMLTextAreaElement | null>(null);
onMounted(() => contentTextArea.value?.focus());
</script>
