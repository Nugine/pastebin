import React, { useEffect, useRef, useState } from "react";
import prism from "../lib/prism";
import MarkdownIt from "markdown-it";
import renderMathInElement, { RenderMathInElementOptions } from "katex/dist/contrib/auto-render";


const markdown = new MarkdownIt();

const katexOptions: RenderMathInElementOptions = {
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

interface MarkdownViewProps {
    content: string
}

const MarkdownView: React.FC<MarkdownViewProps> = ({ content }: MarkdownViewProps) => {
    const [inner, setInner] = useState("");

    const divRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        setInner(markdown.render(content));
        setImmediate(() => {
            if (divRef.current) {
                prism.highlightAllUnder(divRef.current, false);
            }
        });

        setImmediate(() => {
            if (divRef.current) {
                renderMathInElement(divRef.current, katexOptions);
            }
        });
    }, [content]);


    return (
        <div
            ref={divRef}
            style={{ width: "100%" }}
            dangerouslySetInnerHTML={{ __html: inner }}
        />
    );
};


interface CodeViewProps {
    lang: string
    content: string
}

const CodeView: React.FC<CodeViewProps> = ({ lang, content }: CodeViewProps) => {
    const prismClass = `language-${lang}`;

    const divRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        setImmediate(() => {
            if (divRef.current) {
                prism.highlightAllUnder(divRef.current, false);
            }
        });
    });

    return (
        <div style={{ width: "100%" }} ref={divRef}>
            <pre>
                <code className={prismClass}>
                    {content}
                </code>
            </pre>
        </div>
    );
};

interface ViewProps {
    hidden?: boolean
    title?: string
    lang: string,
    content: string
}

const View: React.FC<ViewProps> = ({ hidden, title, lang, content }: ViewProps) => {
    return (
        <div
            style={{
                display: hidden ? "none" : undefined,
                width: "100%",
                flexGrow: 1
            }}
            className="code-area line-numbers"
        >
            {title !== undefined ? (
                <h1 style={{ textAlign: "center" }}>
                    {title}
                </h1>
            ) : null}
            {lang === "markdown"?(
                <MarkdownView content={content} />
            ):(
                <CodeView lang={lang} content={content} />
            )}
        </div>
    );
};

export default View;