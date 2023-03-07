import { defineStore } from "pinia";
import { reactive } from "vue";

import type { PastebinRecord } from "./dto";

import { DEFAULT_EXPIRATION } from "./expiration";
import { DEFAULT_LANG, findLangExt } from "./lang";
import { downloadFile, isValidFileName } from "./download";

export const useStore = defineStore("store", () => {
    const record: PastebinRecord = reactive({
        title: "",
        lang: DEFAULT_LANG,
        expiration_seconds: DEFAULT_EXPIRATION,
        content: "",
    });

    function triggerDownload(key: string): void {
        const fileExt = findLangExt(record.lang) ?? ".txt";
        const isValidTitle = isValidFileName(record.title);
        const fileName = isValidTitle ? record.title : `pastebin-${key}`;
        downloadFile(fileName + fileExt, record.content);
    }

    return { record, triggerDownload };
});
