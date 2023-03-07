<template>
    <div class="btn-bar">
        <button class="btn" type="button" @click="handleEdit" title="切换到编辑界面">编辑</button>
        <button class="btn" type="button" @click="handlePreview" title="切换到预览界面">
            预览
        </button>
        <button
            class="btn"
            type="button"
            @click="handleCopy"
            :class="btnCopyClass[copyStatus]"
            title="将内容复制到剪贴板"
        >
            复制
        </button>
        <button class="btn" type="button" @click="handlePaste" title="保存本条记录">提交</button>
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
                @focusin="formValidated = false"
                :class="[showInvalidFeedback ? 'form-control-invalid' : '']"
            />
            <span class="invalid-feedback" v-if="showInvalidFeedback">内容不能为空</span>
        </div>
    </form>
</template>

<style scoped>
form {
    width: 100%;
    flex-grow: 1;
}

.invalid-feedback {
    color: #dc3545;
    font-size: 80%;
    margin-top: 0.25rem;
}
</style>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useRouter } from "vue-router";

import copyToClipboard from "copy-to-clipboard";

import { useStore } from "@/data/store";
import { LANGS } from "@/data/lang";
import { EXPIRATIONS } from "@/data/expiration";
import * as api from "@/data/api";

const store = useStore();
const router = useRouter();

// 表单校验 ----------------------------

const isValidForm = computed(() => store.record.content !== "");
const formValidated = ref(false);
const showInvalidFeedback = computed(() => !isValidForm.value && formValidated.value);

// 编辑 ----------------------------

const isEditing = ref(true);

function handleEdit() {
    isEditing.value = true;
}

// 预览 ----------------------------

function handlePreview() {
    formValidated.value = true;
    if (isEditing.value && isValidForm.value) {
        isEditing.value = false;
    }
}

// 复制 ----------------------------

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

// 提交 ----------------------------

async function handlePaste() {
    formValidated.value = true;
    if (!isValidForm.value) {
        return;
    }

    const result = await api.saveRecord(store.record);
    if (result.ok) {
        router.push(`/${result.value.key}`);
    } else {
        console.error("提交失败", result.error);
        const msg = result.error?.message ?? "未知错误";
        setTimeout(() => alert(`提交失败: ${msg}`), 100);
    }
}
</script>
