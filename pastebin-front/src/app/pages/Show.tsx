import React, { useContext, useEffect, } from "react";
import { observer, useLocalObservable } from "mobx-react-lite";
import { useParams, useHistory } from "react-router-dom";
import copy from "copy-to-clipboard";
import Modal from "react-bootstrap/Modal";
import Button from "react-bootstrap/Button";
import qrcode from "qrcode";

import { RecordContext } from "../lib/context";
import View from "../components/View";
import * as api from "../lib/api";
import { PROJECT_NAME, downloadFile } from "../lib/data";

function formatNow(sec?: number) {
    const padStart = (n: number): string => n.toString().padStart(2, "0");
    const now = (sec && new Date(sec * 1000)) || new Date();
    const date = `${now.getFullYear()}-${padStart(now.getMonth() + 1)}-${padStart(now.getDate())}`;
    const time = `${padStart(now.getHours())}:${padStart(now.getMinutes())}:${padStart(now.getSeconds())}`;
    return `${date} ${time}`;
}

const Show: React.FC = observer(() => {
    const record = useContext(RecordContext);

    const { key } = useParams<{ key: string }>();

    const v = useLocalObservable(() => ({
        fetchState: null as boolean | null,
        displayQRModal: false,
        dataUrl: null as string | null,
        copyState: null as boolean | null
    }));

    const copyStateResetTime = 600;

    const history = useHistory();
    const url = window.location.href;

    useEffect(() => {
        if (v.fetchState === null) {
            api.findRecord(key).then((res) => {
                Object.assign(record, res);
                v.fetchState = true;
            }).catch((err) => {
                console.error(err);
                v.fetchState = false;
                history.push("/");
            });
        }
        document.title = record.title ? `${record.title} - ${PROJECT_NAME}` : PROJECT_NAME;
        qrcode.toDataURL(url, (e, u) => v.dataUrl = u);
    }, [v, history, key, record, url]);

    const handleEdit = () => history.push("/");
    const handleCopy = () => {
        v.copyState = copy(record.content);
        setTimeout(() => v.copyState = null, copyStateResetTime);
    };
    const handleQRCode = () => v.displayQRModal = true;
    const handleDownload = () => downloadFile(record, key);

    const showBar = (
        <div className="bar-container" style={{ justifyContent: "space-around" }}>
            <div>
                <Button variant="outline-light" onClick={handleEdit} className="bar-item">
                    <span><i className="fa fa-pencil-square-o" aria-hidden="true"></i></span>
                </Button>
                <Button
                    variant={v.copyState !== null ? v.copyState ? "success" : "danger" : "outline-light"}
                    onClick={handleCopy} className="bar-item"
                >
                    <span><i className="fa fa-clipboard" aria-hidden="true"></i></span>
                </Button>
                <Button variant="outline-light" onClick={handleQRCode} className="bar-item">
                    <span><i className="fa fa-qrcode" aria-hidden="true"></i></span>
                </Button>
                <Button variant="outline-light" onClick={handleDownload} className="bar-item">
                    <span><i className="fa fa-download" aria-hidden="true"></i></span>
                </Button>
            </div>
            <div className="hidden-at-small-lv2">
                <span className="mock-btn bar-item">
                    View: {record.view_count}
                </span>
                <span className="mock-btn hidden-at-small bar-item" >
                    {formatNow(record.saving_time_seconds)}
                </span>
            </div>
        </div>
    );

    const view = (
        <View
            title={record.title !== "" ? record.title : undefined}
            lang={record.lang}
            content={record.content}
        />
    );

    const qrModal = (
        <Modal show={v.displayQRModal}>
            <Modal.Header style={{
                justifyContent: "space-between",
                alignItems: "baseline"
            }}>
                <Modal.Title>
                    {record.title !== "" ? record.title : PROJECT_NAME}
                </Modal.Title>
                <Button
                    variant="outline-light"
                    onClick={() => v.displayQRModal = false}
                    style={{ border: "none" }}
                >
                    <span className="h4">×</span>
                </Button>
            </Modal.Header>
            <Modal.Body>
                <div style={{
                    display: "flex",
                    justifyContent: "center",
                    flex: "1 1 auto"
                }}>
                    <img src={v.dataUrl ?? undefined} alt={url} />
                </div>
            </Modal.Body>
        </Modal>
    );

    return (
        <>
            {showBar}
            {view}
            {qrModal}
        </>
    );
});

export default Show;
