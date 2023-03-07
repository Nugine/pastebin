<template>
    <div class="btn-bar">
        <button class="btn" type="button">
            <IconEdit theme="outline" size="16" fill="#333" />
        </button>
        <button class="btn" type="button">
            <IconCopy theme="outline" size="16" fill="#333" />
        </button>
        <button class="btn" type="button">
            <IconQrcode theme="outline" size="16" fill="#333" :stroke-width="5" />
        </button>
        <button class="btn" type="button">
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
import IconQrcode from "@icon-park/vue-next/es/icons/PayCodeOne";
import IconDownload from "@icon-park/vue-next/es/icons/Download";
import IconView from "@icon-park/vue-next/es/icons/PreviewOpen";
</script>
