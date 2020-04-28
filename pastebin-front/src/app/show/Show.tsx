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
import { PROJECT_NAME } from "../../data";

const Show: React.FC = () => {
    const record = useContext(RecordContext);

    const { key } = useParams<{ key: string }>();
    const [isLoading, setIsLoading] = useState(true);
    const [enableQRModal, setEnableQRModal] = useState(false);

    const history = useHistory();

    const load = async () => {
        try {
            const res = await api.findRecord(key);
            Object.assign(record, res);
            setIsLoading(false);
        } catch (err) {
            history.push("/");
        }
    };

    useEffect(() => {
        load();
    });

    const title = record.title ? `${record.title} - ${PROJECT_NAME}` : PROJECT_NAME;


    useEffect(() => {
        document.title = title;
    });

    const handleEdit = () => history.push("/");
    const handleCopy = () => copy(record.content);
    const handleQRCode = () => setEnableQRModal(true);

    const showBar = (
        <ShowBar
            onEdit={handleEdit}
            onCopy={handleCopy}
            onQRCode={handleQRCode}
        />
    );

    const view = (
        <View lang={record.lang} content={record.content}></View>
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
                    {title}
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
        <div>
            {showBar}
            {view}
            {qrModal}
        </div>
    );
    return isLoading ? null : show;
};

export default Show;