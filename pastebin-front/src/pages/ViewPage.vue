<template>
    <div class="btn-bar">
        <button class="btn" type="button" @click="handleEdit">
            <IconEdit theme="outline" size="16" fill="#333" />
        </button>
        <button class="btn" type="button" @click="handleCopy" :class="copyBtnClass">
            <IconCopy theme="outline" size="16" fill="#333" />
        </button>
        <button class="btn" type="button" @click="handleQRCode">
            <IconQRCode theme="outline" size="16" fill="#333" :stroke-width="5" />
        </button>
        <button class="btn" type="button" @click="handleDownload">
            <IconDownload theme="outline" size="16" fill="#333" />
        </button>
        <button class="btn" type="button" disabled>
            <IconView theme="outline" size="16" fill="#333" :stroke-width="5" />
            1
        </button>
    </div>
</template>

<script setup lang="ts">
import IconEdit from "@icon-park/vue-next/es/icons/FileEditing";
import IconCopy from "@icon-park/vue-next/es/icons/CopyOne";
import IconQRCode from "@icon-park/vue-next/es/icons/PayCodeOne";
import IconDownload from "@icon-park/vue-next/es/icons/Download";
import IconView from "@icon-park/vue-next/es/icons/PreviewOpen";

import { useCopyBtn } from "@/logic";
import { useStore } from "@/data/store";
import { useRouter } from "vue-router";

const store = useStore();
const router = useRouter();

// 编辑 ----------------------------

function handleEdit() {
    router.push("/");
}

// 复制 ----------------------------

const { copyBtnClass, handleCopy } = useCopyBtn(() => store.record.content);

// 二维码 ----------------------------

function handleQRCode() {
    // TODO
}

// 下载 ----------------------------

function handleDownload() {
    // TODO
}
</script>
