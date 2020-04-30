import React, { useState, useContext } from "react";

import Button from "react-bootstrap/Button";
import { RecordContext } from "../context";

import "./ShowBar.css";
import Container from "react-bootstrap/Container";

interface ShowBarProps {
    onEdit: () => void
    onCopy: () => boolean,
    onDownload: () => void,
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
    const { onEdit, onCopy, onQRCode, onDownload } = props;
    const record = useContext(RecordContext);



    const [copyState, setCopyState] = useState<boolean | null>(null);
    const copyStateResetTime = 600;

    const handleCopy = () => {
        setCopyState(onCopy());
        setTimeout(() => setCopyState(null), copyStateResetTime);
    };

    const matchVariant = () => (copyState !== null ? copyState ? "success" : "danger" : "outline-light");

    const copyBtn = (
        <Button variant={matchVariant()} onClick={handleCopy} className="bar-item">
            <span><i className="fa fa-clipboard" aria-hidden="true"></i></span>
        </Button>
    );

    const downloadBtn = (
        <Button variant="outline-light" onClick={onDownload} className="bar-item">
            <span><i className="fa fa-download" aria-hidden="true"></i></span>
        </Button>
    );

    const qrcodeBtn = (
        <Button variant="outline-light" onClick={onQRCode} className="bar-item">
            <span><i className="fa fa-qrcode" aria-hidden="true"></i></span>
        </Button>
    );

    const editBtn = (
        <Button variant="outline-light" onClick={onEdit} className="bar-item">
            <span><i className="fa fa-pencil-square-o" aria-hidden="true"></i></span>
        </Button>
    );

    const viewSpan = (
        <span className="mock-btn bar-item">
            View: {record.view_count}
        </span>
    );

    const timeSpan = (
        <span className="mock-btn hidden-at-small bar-item" >
            {formatNow(record.saving_time_seconds)}
        </span>
    );

    return (
        <div className="bar-container" style={{ justifyContent: "space-around" }}>
            <div >{editBtn}{copyBtn}{qrcodeBtn}{downloadBtn}</div>
            <div className="hidden-at-small-lv2">{viewSpan}{timeSpan}</div>
        </div>
    );
};

export default ShowBar;
