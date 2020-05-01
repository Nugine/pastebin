import React, { useState, useEffect, useRef } from "react";
import prism from "./prism";
import { options as katexOptions } from "./katex";
import markdown from "./markdown";
import renderMathInElement from "katex/dist/contrib/auto-render";

interface Props {
    content: string
}

const MarkdownView: React.FC<Props> = ({ content }: Props) => {
    const [inner, setInner] = useState("");

    const divRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        setInner(markdown.render(content));
        setImmediate(() => {
            if (divRef.current) {
                prism.highlightAllUnder(divRef.current!, false);
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

export default MarkdownView;

