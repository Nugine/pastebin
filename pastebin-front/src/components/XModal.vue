<template>
    <Teleport to="body" v-if="mountTeleport">
        <div class="fade backdrop" :class="{ show: show }"></div>
        <div class="fade modal" :class="{ show: show }">
            <div class="modal-dialog">
                <slot />
            </div>
        </div>
    </Teleport>
</template>

<style scoped>
.fade {
    transition: opacity 0.15s linear;
}

.fade:not(.show) {
    opacity: 0;
}

.backdrop {
    position: fixed;
    top: 0;
    left: 0;
    z-index: 1040;
    width: 100vw;
    height: 100vh;
    background-color: #000;
}

.backdrop.show {
    opacity: 0.5;
}

.modal {
    position: fixed;
    top: 0;
    left: 0;
    z-index: 1050;
    width: 100%;
    overflow: hidden;
    outline: 0;
    display: block;
    padding: 1rem 0.5rem;
}

.modal-dialog {
    width: auto;
    max-width: 500px;
    margin: 0 auto;
    pointer-events: auto;

    display: flex;
    flex-direction: column;
    background-color: #fff;
    background-clip: padding-box;
    border: 1px solid rgba(0, 0, 0, 0.2);
    border-radius: 0.3rem;
    outline: 0;

    transition: transform 0.2s ease-out;
    transform: translateY(-50px);
}

.modal.show .modal-dialog {
    transform: none;
}
</style>

<script setup lang="ts">
import { ref, watchPostEffect } from "vue";

const props = defineProps<{ show: boolean }>();

const mountTeleport = ref(props.show);
const show = ref(false);

watchPostEffect((onCleanup) => {
    if (props.show) {
        mountTeleport.value = true;
        show.value = false;

        // 挂载 Teleport 后，显示 Modal，触发 CSS 动画
        const timer = setTimeout(() => (show.value = true), 60);
        onCleanup(() => clearTimeout(timer));
    } else {
        // 触发 CSS 动画
        show.value = false;

        // 当动画结束后，卸载 Teleport
        const timer = setTimeout(() => (mountTeleport.value = false), 400);
        onCleanup(() => clearTimeout(timer));
    }
});
</script>
