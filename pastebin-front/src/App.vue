<template>
    <div style="flex-grow: 1; width: 100%; margin-top: 1em">
        <RouterView />
    </div>
    <footer style="margin-bottom: 1em">
        <span>
            © 2019 - 2023
            <a href="https://github.com/Nugine/pastebin" target="_blank">Nugine/pastebin</a>
            AGPL-3.0
        </span>
    </footer>
</template>

<style scoped></style>

<script setup lang="ts">
import { RouterView } from "vue-router";
</script>
