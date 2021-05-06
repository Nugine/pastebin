export type Key = string;

type RecordBase = {
    title: string;
    lang: string;
    expiration_seconds: number;
    content: string;
};

type SavedBase = {
    saving_time_seconds: number;
    view_count: number;
};

export type PastebinRecord = RecordBase & Partial<SavedBase>;

export type SaveRecordReq = RecordBase;

export interface SaveRecordRes {
    key: Key;
}

export type FindRecordRes = RecordBase & SavedBase;

export interface ErrorRes {
    code: number;
    message: string;
}

export interface Lang {
    value: string;
    display: string;
    ext: string;
}

export interface Expiration {
    value: number;
    display: string;
}

export const PROJECT_NAME = "Nugine Pastebin";

// -------------------------------------------

export const expirations: Expiration[] = [
    {
        value: 3600,
        display: "one hour",
    },
    {
        value: 3600 * 24,
        display: "one day",
    },
    {
        value: 3600 * 24 * 3,
        display: "three days",
    },
    {
        value: 3600 * 24 * 7,
        display: "one week",
    },
    {
        value: 3600 * 24 * 30,
        display: "one month",
    }
];

// -------------------------------------------

const langs: Lang[] = (() => {
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

    return langArray;
})();

export function findLangExt(langValue: string): string | null {
    for (const lang of langs) {
        if (lang.value === langValue) {
            return lang.ext;
        }
    }
    return null;
}