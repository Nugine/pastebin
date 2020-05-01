import React from "react";
import MarkdownView from "./MarkdownView";
import CodeView from "./CodeView";

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

    const getView = () => {
        if (lang === "markdown") {
            return <MarkdownView content={content} />;
        } else {
            return <CodeView lang={lang} content={content} />;
        }
    };

    return (
        <div
            style={{
                display: hidden ? "none" : undefined,
                width: "100%",
                flexGrow: 1
            }}
            className="code-area line-numbers"
        >
            {titleH1}
            {getView()}
        </div>
    );
};

export default View;