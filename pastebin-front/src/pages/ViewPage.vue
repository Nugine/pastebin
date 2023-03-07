<template>
    <div class="btn-bar">
        <XButton class="btn" type="button" @click="handleEdit">
            <IconEdit theme="outline" size="16" fill="#333" />
        </XButton>
        <XButton class="btn" type="button" @click="handleCopy" :class="copyBtnClass">
            <IconCopy theme="outline" size="16" fill="#333" />
        </XButton>
        <XButton class="btn" type="button" @click="qrcodeModal.open()">
            <IconQRCode theme="outline" size="16" fill="#333" :stroke-width="5" />
        </XButton>
        <XModal :show="qrcodeModal.show">
            <div class="modal-header">
                <span class="modal-title">{{ qrcodeModal.title }}</span>
                <XButton class="btn" type="button" @click="qrcodeModal.close()">
                    <IconClose theme="filled" size="16" fill="#333" />
                </XButton>
            </div>
            <div class="modal-content">
                <img :src="qrcodeModal.dataUrl" :alt="currentUrl" />
            </div>
        </XModal>
        <XButton class="btn" type="button" @click="handleDownload">
            <IconDownload theme="outline" size="16" fill="#333" />
        </XButton>
        <XButton class="btn" type="button" disabled>
            <IconView theme="outline" size="16" fill="#333" :stroke-width="5" />
            {{ store.record.view_count }}
        </XButton>
    </div>
</template>

<style scoped>
.modal-header {
    width: 100%;
    padding: 1rem;
    border-bottom: 1px solid #dee2e6;

    display: flex;
    justify-content: space-between;
    align-items: baseline;
}

.modal-title {
    font-size: 1.3rem;
    font-weight: 400;
}

.modal-header .btn {
    border: none;
    box-shadow: none;
}

.modal-content {
    width: 100%;
    padding: 1rem;

    display: flex;
    justify-content: center;
}
</style>

<script setup lang="ts">
import IconEdit from "@icon-park/vue-next/es/icons/FileEditing";
import IconCopy from "@icon-park/vue-next/es/icons/CopyOne";
import IconQRCode from "@icon-park/vue-next/es/icons/PayCodeOne";
import IconDownload from "@icon-park/vue-next/es/icons/Download";
import IconView from "@icon-park/vue-next/es/icons/PreviewOpen";
import IconClose from "@icon-park/vue-next/es/icons/Close";

import qrcode from "qrcode";

import { useRoute, useRouter } from "vue-router";
import { computed, onMounted, reactive, watchEffect } from "vue";

import XModal from "@/components/XModal.vue";
import XButton from "@/components/XButton.vue";
import { useCopyBtn } from "@/hooks/useCopyBtn";
import { useStore } from "@/data/store";
import * as api from "@/data/api";
import { downloadFile, isValidFileName } from "@/data/download";
import { findLangExt } from "@/data/lang";

const store = useStore();
const router = useRouter();

// 载入数据 ----------------------------

const route = useRoute();
const recordkey = computed(() => route.params.key as string);

watchEffect(async () => {
    const result = await api.findRecord({ key: recordkey.value });
    if (result.ok) {
        store.record = result.value;
    } else {
        console.error("查询失败", result.error);
        router.push("/");
    }
});

// 编辑 ----------------------------

function handleEdit() {
    router.push("/");
}

// 复制 ----------------------------

const { copyBtnClass, handleCopy } = useCopyBtn(() => store.record.content);

// 二维码 ----------------------------

const qrcodeModal = reactive({
    show: false,

    title: computed(() => store.record.title || "在线剪贴板"),

    dataUrl: "",

    open() {
        this.show = true;
    },
    close() {
        this.show = false;
    },
});

const currentUrl = window.location.href;
onMounted(() => {
    qrcode.toDataURL(currentUrl, (_e, u) => (qrcodeModal.dataUrl = u));
});

// 下载 ----------------------------

function handleDownload() {
    const record = store.record;
    const isValidTitle = isValidFileName(record.title);
    const fileName = isValidTitle ? record.title : `pastebin-${recordkey.value}`;
    const fileExt = findLangExt(record.lang) ?? ".txt";
    downloadFile(fileName + fileExt, record.content);
}
</script>
