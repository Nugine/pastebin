import React, { useEffect, useRef } from "react";
import prism from "./prism";

interface Props {
    lang: string
    content: string
}

const CodeView: React.FC<Props> = ({ lang, content }: Props) => {
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

export default CodeView;