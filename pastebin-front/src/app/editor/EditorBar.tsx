import React, { useState } from "react";

import Button from "react-bootstrap/Button";

interface EditorBarProps {
    onEdit: () => void,
    onCopy: () => boolean,
    onPreview: () => void,
    onPaste: () => void,
}

const EditorBar: React.FC<EditorBarProps> = (props: EditorBarProps) => {
    const { onEdit, onCopy, onPreview, onPaste } = props;

    const editBtn = (
        <Button variant="outline-light" onClick={onEdit} className="bar-item">
            Edit
        </Button>
    );

    const previewBtn = (
        <Button variant="outline-light" onClick={onPreview} className="bar-item">
            Preview
        </Button>
    );

    const [copyState, setCopyState] = useState<boolean | null>(null);
    const copyStateResetTime = 600;

    const handleCopy = () => {
        setCopyState(onCopy());
        setTimeout(() => setCopyState(null), copyStateResetTime);
    };

    const matchVariant = () => (copyState !== null ? copyState ? "success" : "danger" : "outline-light");

    const copyBtn = (
        <Button variant={matchVariant()} onClick={handleCopy} className="bar-item">
            Copy
        </Button>
    );

    const pasteBtn = (
        <Button variant="outline-light" onClick={onPaste} className="bar-item">
            Paste !
        </Button>
    );

    return (
        <div className="bar-container">
            {editBtn}{previewBtn}
            {copyBtn}{pasteBtn}
        </div>
    );
};

export default EditorBar;
