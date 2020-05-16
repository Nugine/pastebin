import { Lang } from "./index";

const langArray: Lang[] = [];

langArray.push({
    value: "markdown",
    display: "Markdown",
    ext: ".md"
});

langArray.push({
    value: "plaintext",
    display: "PlainText",
    ext: ".txt"
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
].sort(([, lhs], [, rhs]) => (lhs < rhs ? -1 : (lhs === rhs ? 0 : 1)));

for (const [value, display, ext] of others) {
    langArray.push({ value, display, ext });
}

export function getLangArray(): Lang[] {
    return langArray;
}

export function findExt(langValue: string): string | null {
    for (const lang of langArray) {
        if (lang.value === langValue) {
            return lang.ext;
        }
    }
    return null;
}