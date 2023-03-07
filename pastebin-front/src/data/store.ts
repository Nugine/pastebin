import { defineStore } from "pinia";
import { reactive } from "vue";

import type { PastebinRecord } from "./dto";

import { DEFAULT_EXPIRATION } from "./expiration";
import { DEFAULT_LANG } from "./lang";

export const useStore = defineStore("store", () => {
    const record: PastebinRecord = reactive({
        title: "",
        lang: DEFAULT_LANG,
        expiration_seconds: DEFAULT_EXPIRATION,
        content: "",
    });

    return { record };
});
