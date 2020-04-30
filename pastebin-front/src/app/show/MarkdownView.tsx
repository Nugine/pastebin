import React, { useState, useEffect, useRef } from "react";
import MarkdownIt from "markdown-it";
import Prism from "./prism";

let md: MarkdownIt | null = null;

const getMd: () => MarkdownIt = () => {
    if (md === null) {
        md = new MarkdownIt();
    }
    return md;
};

const render: (content: string) => (string) = (content) => {
    return getMd().render(content);
};

interface Props {
    content: string
}

const MarkdownView: React.FC<Props> = ({ content }: Props) => {
    const [inner, setInner] = useState("");

    const divRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        setInner(render(content));
        console.log("markdown view effect");
        console.log(Prism);
        setImmediate(() => {
            console.log("ready to highlight");
            console.log("div: ", divRef.current);
            if (divRef.current) {
                Prism.highlightAllUnder(divRef.current!, false, (e) => console.log("highlight: completed", e));
            }
        });
    }, [content]);


    return (
        <div
            ref={divRef}
            dangerouslySetInnerHTML={{ __html: inner }}
        />
    );
};

export default MarkdownView;

