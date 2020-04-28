import React, { useState, useContext } from "react";

import Button from "react-bootstrap/Button";
import { RecordContext } from "../context";

import "./ShowBar.css";

interface ShowBarProps {
    onEdit: () => void
    onCopy: () => boolean,
    onQRCode: () => void,
}

function formatNow(sec?: number) {
    const padStart = (n: number): string => n.toString().padStart(2, "0");
    const now = (sec && new Date(sec * 1000)) || new Date();
    const date = `${now.getFullYear()}-${padStart(now.getMonth() + 1)}-${padStart(now.getDate())}`;
    const time = `${padStart(now.getHours())}:${padStart(now.getMinutes())}:${padStart(now.getSeconds())}`;
    return `${date} ${time}`;
}

const ShowBar: React.FC<ShowBarProps> = (props: ShowBarProps) => {
    const { onEdit, onCopy, onQRCode } = props;
    const record = useContext(RecordContext);

    const editBtn = (
        <Button variant="outline-light" onClick={onEdit}>
            Edit
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
        <Button variant={matchVariant()} onClick={handleCopy}>
            Copy
        </Button>
    );

    const qrcodeBtn = (
        <Button variant="outline-light" onClick={onQRCode}>
            QRCode
        </Button>
    );

    const viewSpan = (
        <span className="btn btn-outline-light">
            View: {record.view_count}
        </span>
    );

    const timeSpan = (
        <span className="btn btn-outline-light hidden-at-small">
            {formatNow(record.saving_time_seconds)}
        </span>
    );

    return (
        <div className="bar-container">
            <div>{editBtn}{copyBtn}</div><div>{qrcodeBtn}{viewSpan}{timeSpan}</div>
        </div>
    );
};

export default ShowBar;
