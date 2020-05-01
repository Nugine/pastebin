import katex from "katex";
import { RenderMathInElementOptions } from "katex/dist/contrib/auto-render";

export default katex;


export const options: RenderMathInElementOptions = {
    delimiters: [
        { left: "$$", right: "$$", display: true },
        { left: "\\(", right: "\\)", display: false },
        { left: "\\[", right: "\\]", display: true },
        { left: "$", right: "$", display: false },
    ],
    errorColor: "#cc0000",
    throwOnError: false,
    strict: "ignore",
};
