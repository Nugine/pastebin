import copyToClipboard from "copy-to-clipboard";
import { computed, ref } from "vue";

export function useCopyBtn(content: () => string) {
    const btnClasses = {
        none: [],
        success: ["btn-success"],
        failure: ["btn-failure"],
    };
    const copyStatus = ref<keyof typeof btnClasses>("none");

    function handleCopy() {
        const result = copyToClipboard(content());
        copyStatus.value = result ? "success" : "failure";
        const resetTime = 600;
        setTimeout(() => (copyStatus.value = "none"), resetTime);
    }

    const copyBtnClass = computed(() => btnClasses[copyStatus.value]);

    return {
        copyBtnClass,
        handleCopy,
    };
}
