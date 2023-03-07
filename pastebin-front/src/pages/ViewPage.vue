<template>
    <div class="btn-bar">
        <button class="btn" type="button" @click="handleEdit">
            <IconEdit theme="outline" size="16" fill="#333" />
        </button>
        <button class="btn" type="button" @click="handleCopy" :class="copyBtnClass">
            <IconCopy theme="outline" size="16" fill="#333" />
        </button>
        <button class="btn" type="button" @click="qrcodeModal.open()">
            <IconQRCode theme="outline" size="16" fill="#333" :stroke-width="5" />
        </button>
        <AppModal :show="qrcodeModal.show">
            <div class="modal-header">
                <span class="modal-title">{{ qrcodeModal.title }}</span>
                <button class="btn" type="button" @click="qrcodeModal.close()">
                    <IconClose theme="filled" size="16" fill="#333" />
                </button>
            </div>
            <div class="modal-content">
                <img :src="qrcodeModal.dataUrl" :alt="currentUrl" />
            </div>
        </AppModal>
        <button class="btn" type="button" @click="handleDownload">
            <IconDownload theme="outline" size="16" fill="#333" />
        </button>
        <button class="btn" type="button" disabled>
            <IconView theme="outline" size="16" fill="#333" :stroke-width="5" />
            1
        </button>
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
    flex: "1 1 auto";
}
</style>

<script setup lang="ts">
import IconEdit from "@icon-park/vue-next/es/icons/FileEditing";
import IconCopy from "@icon-park/vue-next/es/icons/CopyOne";
import IconQRCode from "@icon-park/vue-next/es/icons/PayCodeOne";
import IconDownload from "@icon-park/vue-next/es/icons/Download";
import IconView from "@icon-park/vue-next/es/icons/PreviewOpen";
import IconClose from "@icon-park/vue-next/es/icons/Close";

import { useRouter } from "vue-router";
import { computed, onMounted, reactive } from "vue";

import qrcode from "qrcode";

import AppModal from "@/components/AppModal.vue";

import { useCopyBtn } from "@/logic";
import { useStore } from "@/data/store";

const store = useStore();
const router = useRouter();

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
    // TODO
}
</script>
