import { createRouter, createWebHistory } from "vue-router";
import EditorPage from "./pages/EditorPage.vue";
import ViewPage from "./pages/ViewPage.vue";

export default createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        { path: "/:key", component: ViewPage },
        { path: "/", component: EditorPage },
        { path: "/:pathMatch(.*)", redirect: "/" },
    ],
    strict: true,
    sensitive: true,
});
