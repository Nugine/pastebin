<template>
    <div class="btn-bar">
        <button class="btn" type="button" @click="handleEdit">编辑</button>
        <button class="btn" type="button" @click="handlePreview">预览</button>
        <button class="btn" type="button" @click="handleCopy">复制</button>
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
import { LANGS } from "@/data/lang";
import { useStore } from "@/data/store";
import { EXPIRATIONS } from "@/data/expiration";

const store = useStore();

function handleEdit() {
    // TODO
}

function handlePreview() {
    // TODO
}

function handleCopy() {
    // TODO
}

function handlePaste() {
    // TODO
}
</script>
