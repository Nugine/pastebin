import { Lang } from "./index";

const langArray: Lang[] = [];

langArray.push({
    value: "markdown",
    display: "Markdown",
});

langArray.push({
    value: "plaintext",
    display: "PlainText",
});

export function getLangArray(): Lang[] {
    return langArray;
}
