import React, { useContext, useEffect, useState } from "react";

import { RecordContext } from "../context";
import View from "./View";
import ShowBar from "./ShowBar";
import * as api from "../../data/api";
import { useParams, useHistory } from "react-router-dom";
import copy from "copy-to-clipboard";
import Modal from "react-bootstrap/Modal";
import Button from "react-bootstrap/Button";
import qrcode from "qrcode";
import { PROJECT_NAME, PastebinRecord } from "../../data";
import { findExt } from "../../data/lang";

// only for PROJECT_NAME
const toKebabCase = (s: string) => s.split(" ").map((p) => p.toLowerCase()).join("-");

const downloadFile = (record: PastebinRecord, key: string) => {
    const fileExt = findExt(record.lang) ?? ".txt";
    const isValid = record.title !== "" && record.title.split("").reduce(
        (acc, x) => (
            acc && ("~`!@#$%^&*()-+={}[]|:;\"'<>,.?/\b\f\n\r\t\v\\\0".indexOf(x) === -1)
        ), true);
    const fileName = isValid ? (record.title) : (`${toKebabCase(PROJECT_NAME)}-${key}`);
    console.log(record, isValid, fileName, fileExt);
    const a = document.createElement("a");
    a.download = `${fileName}${fileExt}`;
    a.href = URL.createObjectURL(new Blob([record.content]));
    a.style.display = "none";
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
};

const Show: React.FC = () => {
    const record = useContext(RecordContext);

    const { key } = useParams<{ key: string }>();
    const [fetchState, setFetchState] = useState<boolean | null>(null);
    const [enableQRModal, setEnableQRModal] = useState(false);

    const history = useHistory();

    useEffect(() => {
        if (fetchState === null) {
            (async () => {
                try {
                    const res = await api.findRecord(key);
                    Object.assign(record, res);
                    setFetchState(true);
                } catch (err) {
                    console.error(err);
                    setFetchState(false);
                    history.push("/");
                }
            })();
        }
    }, [fetchState, history, key, record]);

    const title = record.title ? `${record.title} - ${PROJECT_NAME}` : PROJECT_NAME;


    useEffect(() => {
        document.title = title;
    });

    const handleEdit = () => history.push("/");
    const handleCopy = () => copy(record.content);
    const handleQRCode = () => setEnableQRModal(true);
    const handleDownload = () => downloadFile(record, key);

    const showBar = (
        <ShowBar
            onEdit={handleEdit}
            onCopy={handleCopy}
            onQRCode={handleQRCode}
            onDownload={handleDownload}
        />
    );

    const view = (
        <View
            title={record.title !== "" ? record.title : undefined}
            lang={record.lang}
            content={record.content}
        />
    );

    const [dataUrl, setDataUrl] = useState<string | null>(null);
    const url = window.location.href;

    useEffect(() => {
        qrcode.toDataURL(url, (e, v) => setDataUrl(v));
    });

    const qrModal = (
        <Modal show={enableQRModal}>
            <Modal.Header style={{
                justifyContent: "space-between",
                alignItems: "baseline"
            }}>
                <Modal.Title>
                    {record.title !== "" ? record.title : PROJECT_NAME}
                </Modal.Title>
                <Button
                    variant="outline-light"
                    onClick={() => setEnableQRModal(false)}
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
                    <img src={dataUrl ?? undefined} alt={url} />
                </div>
            </Modal.Body>
        </Modal>
    );



    const show = (
        <>
            {showBar}
            {view}
            {qrModal}
        </>
    );

    return show;
};

export default Show;