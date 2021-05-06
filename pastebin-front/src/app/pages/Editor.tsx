import React, { useContext, useRef, useEffect, } from "react";
import Button from "react-bootstrap/Button";
import { observer, useLocalObservable } from "mobx-react-lite";
import copy from "copy-to-clipboard";
import { useHistory } from "react-router-dom";
import Form from "react-bootstrap/Form";

import * as api from "../lib/api";
import { RecordContext } from "../lib/context";
import View from "../components/View";
import { PastebinRecord, langs, expirations } from "../lib/data";


const Editor: React.FC = observer(() => {
    const record = useContext(RecordContext);

    const v = useLocalObservable(() => ({
        isEditing: true,
        isValid: (record.content !== "" ? true : null) as boolean | null,
        copyState: null as boolean | null,
    }));
    const copyStateResetTime = 600;

    const history = useHistory();

    const handleEdit = () => {
        v.isEditing = true;
    };

    const handleCopy = () => {
        v.copyState = copy(record.content);
        setTimeout(() => v.copyState = null, copyStateResetTime);
    };

    const handlePreview = () => {
        if (v.isEditing) {
            if (v.isValid) {
                v.isEditing = false;
            } else {
                v.isValid = record.content !== "";
            }
        }
    };

    const handlePaste = async () => {
        if (!v.isValid) {
            v.isValid = false;
            return;
        }
        try {
            const res = await api.saveRecord(record);
            history.push(`/${res.key}/`);
        } catch (err) {
            console.error(err);
            // eslint-disable-next-line @typescript-eslint/restrict-template-expressions, @typescript-eslint/no-unsafe-member-access
            setImmediate(() => alert(`Failed: ${err?.message}`));
        }
    };


    const editorBar = (
        <div className="bar-container">
            <Button variant="outline-light" onClick={handleEdit} className="bar-item">
                Edit
            </Button>
            <Button variant="outline-light" onClick={handlePreview} className="bar-item">
                Preview
            </Button>
            <Button
                variant={v.copyState !== null ? v.copyState ? "success" : "danger" : "outline-light"}
                onClick={handleCopy} className="bar-item"
            >
                Copy
            </Button>
            <Button variant="outline-light" onClick={handlePaste} className="bar-item">
                Paste !
            </Button>
        </div>
    );

    type HandleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => void;
    function syncField<K extends keyof PastebinRecord, V extends PastebinRecord[K]>(key: K, cvt: (val: string) => V): HandleChange {
        return (e) => {
            const prev = record[key];
            const now = cvt(e.currentTarget.value);
            if (prev !== now) {
                record[key] = now;
            }
        };
    }

    const contentTextAreaRef = useRef<HTMLTextAreaElement>(null);
    useEffect(() => contentTextAreaRef.current?.focus(), []);

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        e.stopPropagation();
    };

    const editorForm = (
        <Form
            onSubmit={handleSubmit}
            autoComplete="off"
            style={{
                display: !v.isEditing ? "none" : undefined,
                width: "100%",
                flexGrow: 1,
            }}
        >
            <Form.Group>
                <Form.Label>
                    Title
                </Form.Label>
                <Form.Control
                    type="text"
                    defaultValue={record.title}
                    onChange={syncField("title", (s) => (s))}
                    className="code-area"
                >
                </Form.Control>
            </Form.Group>
            <Form.Group>
                <Form.Label>
                    Language
                </Form.Label>
                <Form.Control
                    as="select"
                    defaultValue={record.lang}
                    onChange={syncField("lang", (s) => (s))}
                    custom
                >
                    {langs.map((lang) => (
                        <option key={lang.value} value={lang.value}>{lang.display}</option>
                    ))}
                </Form.Control>
            </Form.Group>
            <Form.Group>
                <Form.Label>
                    Expiration
                </Form.Label>
                <Form.Control
                    as="select"
                    defaultValue={record.expiration_seconds}
                    onChange={syncField("expiration_seconds", (s) => {
                        const n = parseInt(s, 10);
                        if (isNaN(n)) { throw new Error("invalid number"); }
                        return n;
                    })}
                    custom
                >
                    {expirations.map((exp) => (
                        <option key={exp.value} value={exp.value}>{exp.display}</option>
                    ))}
                </Form.Control>
            </Form.Group>
            <div className={v.isValid === false ? "was-validated" : undefined}>
                <Form.Group>
                    <Form.Label>
                        Content
                    </Form.Label>
                    <Form.Control
                        as="textarea"
                        rows={15}
                        defaultValue={record.content}
                        isInvalid={v.isValid === false ? true : undefined}
                        onChange={syncField("content", (s) => {
                            if ((s !== "") !== v.isValid) { v.isValid = !v.isValid; }
                            return s;
                        })}
                        required
                        ref={contentTextAreaRef}
                        className="code-area"
                    >
                    </Form.Control>
                    <Form.Control.Feedback type='invalid'>This field is required.</Form.Control.Feedback>
                </Form.Group>
            </div>
        </Form>
    );

    const editorView = v.isEditing ? null : (
        <View
            hidden={v.isEditing}
            title={record.title !== "" ? record.title : undefined}
            lang={record.lang}
            content={record.content}
        />
    );


    return (
        <>
            {editorBar}
            {editorForm}
            {editorView}
        </>
    );
});

export default Editor;
