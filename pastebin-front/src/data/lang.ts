export interface Lang {
    value: string;
    display: string;
    ext: string;
}

export const LANGS: Lang[] = (() => {
    const langs: Lang[] = [];

    langs.push({
        value: "markdown",
        display: "Markdown",
        ext: ".md",
    });

    langs.push({
        value: "plaintext",
        display: "纯文本",
        ext: ".txt",
    });

    const others: Array<Array<string>> = [
        ["html", "HTML", ".html"],
        ["css", "CSS", ".css"],
        ["javascript", "JavaScript", ".js"],
        ["bash", "Bash", ".sh"],
        ["c", "C", ".c"],
        ["cpp", "C++", ".cpp"],
        ["cs", "C#", ".cs"],
        ["erlang", "Erlang", ".erl"],
        ["go", "Go", ".go"],
        ["haskell", "Haskell", ".hs"],
        ["rust", "Rust", ".rs"],
        ["java", "Java", ".java"],
        ["json", "JSON", ".json"],
        ["kotlin", "Kotlin", ".kt"],
        ["latex", "LaTeX", ".tex"],
        ["php", "PHP", ".php"],
        ["python", "Python", ".py"],
        ["scala", "Scala", ".scala"],
        ["sql", "SQL", ".sql"],
        ["toml", "TOML", ".toml"],
        ["typescript", "TypeScript", ".ts"],
    ];

    others.sort(([, lhs], [, rhs]) => (lhs < rhs ? -1 : lhs === rhs ? 0 : 1));

    for (const [value, display, ext] of others) {
        langs.push({ value, display, ext });
    }

    return langs;
})();

export function findLangExt(langValue: string): string | null {
    const ans = LANGS.find((lang) => lang.value === langValue);
    return ans ? ans.ext : null;
}

export const DEFAULT_LANG = LANGS[0].value;
