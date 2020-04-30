import React from "react";
import MarkdownView from "./MarkdownView";

interface Props {
    hidden?: boolean
    title?: string
    lang: string,
    content: string
}

const View: React.FC<Props> = ({ hidden, title, lang, content }: Props) => {
    const titleH1 = title !== undefined ? (
        <h1 style={{ textAlign: "center" }}>
            {title}
        </h1>
    ) : null;

    const body = lang === "markdown" ? (
        <MarkdownView content={content} />
    ) : null;

    return (
        <div
            style={hidden ? { display: "none" } : undefined}
            className="code-area line-numbers"
        >
            {titleH1}
            {body}
        </div>
    );
};

export default View;