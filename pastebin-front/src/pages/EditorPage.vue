<template>
    <div class="btn-bar">
        <button class="btn" type="button" @click="handleEdit">编辑</button>
        <button class="btn" type="button" @click="handlePreview">预览</button>
        <button class="btn" type="button" @click="handleCopy" :class="btnCopyClass[copyStatus]">
            复制
        </button>
        <button class="btn" type="button" @click="handlePaste">提交</button>
    </div>
    <form autocomplete="off">
        <div class="form-group">
            <label>标题</label>
            <input class="form-control" type="text" v-model="store.record.title" />
        </div>
        <div class="form-group">
            <label>语言</label>
            <select class="form-control" v-model="store.record.lang">
                <option v-for="lang in LANGS" :key="lang.value" :value="lang.value">
                    {{ lang.display }}
                </option>
            </select>
        </div>
        <div class="form-group">
            <label>过期时间</label>
            <select class="form-control" v-model="store.record.expiration_seconds">
                <option v-for="exp in EXPIRATIONS" :key="exp.value" :value="exp.value">
                    {{ exp.display }}
                </option>
            </select>
        </div>
        <div class="form-group">
            <label>内容</label>
            <textarea
                class="form-control"
                rows="15"
                required
                autofocus
                v-model="store.record.content"
            />
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
import { ref } from "vue";
import copyToClipboard from "copy-to-clipboard";

import { useStore } from "@/data/store";
import { LANGS } from "@/data/lang";
import { EXPIRATIONS } from "@/data/expiration";

const store = useStore();

function handleEdit() {
    // TODO
}

function handlePreview() {
    // TODO
}

const btnCopyClass = {
    none: [],
    success: ["btn-success"],
    failure: ["btn-failure"],
};
const copyStatus = ref<keyof typeof btnCopyClass>("none");

function handleCopy() {
    const result = copyToClipboard(store.record.content);
    copyStatus.value = result ? "success" : "failure";
    const resetTime = 600;
    setTimeout(() => (copyStatus.value = "none"), resetTime);
}

function handlePaste() {
    // TODO
}
</script>
