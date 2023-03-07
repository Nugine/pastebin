<template>
    <main class="container-lg">
        <RouterView />
    </main>
    <footer>
        <span>
            © 2019 - 2023
            <a href="https://github.com/Nugine/pastebin" target="_blank">Nugine/pastebin</a>
            AGPL-3.0
        </span>
    </footer>
</template>

<style scoped>
main {
    flex-grow: 1;

    display: flex;
    flex-direction: column;

    padding: 0 15px;
}

footer {
    margin-bottom: 1em;
}
</style>

<script setup lang="ts">
import { RouterView } from "vue-router";
</script>
