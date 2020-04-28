import React from "react";

interface Props {
    hidden?: boolean
    lang: string,
    content: string
}

const View: React.FC<Props> = ({ hidden, lang, content }: Props) => {

    return (
        <div
            style={hidden ? { display: "none" } : undefined}
            className="code-area"
        >
            <p>{lang}</p>
            <p>{content}</p>
        </div>
    );
};

export default View;