import { LANGUAGES } from "../../prismjs.custom";

export interface Lang {
    value: string;
    display: string;
    ext: string;
}

export const LANGS: Lang[] = (() => {
    const langs: Lang[] = [convert(LANGUAGES[0]), convert(["plaintext", "纯文本", ".txt"])];

    const others = [...LANGUAGES.slice(1)];
    others.sort((lhs, rhs) => compareString(lhs[1], rhs[1]));
    others.forEach((tuple) => langs.push(convert(tuple)));

    return langs;
})();

function convert(tuple: [string, string, string]): Lang {
    return { value: tuple[0], display: tuple[1], ext: tuple[2] };
}

function compareString(lhs: string, rhs: string): number {
    return lhs < rhs ? -1 : lhs === rhs ? 0 : 1;
}

export function findLangExt(langValue: string): string | null {
    const ans = LANGS.find((lang) => lang.value === langValue);
    return ans ? ans.ext : null;
}

export const DEFAULT_LANG = LANGS[0].value;
