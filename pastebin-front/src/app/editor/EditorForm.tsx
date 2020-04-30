import React, { useRef, useEffect, useContext } from "react";

import Form from "react-bootstrap/Form";

import { PastebinRecord } from "../../data";
import { getLangArray } from "../../data/lang";
import { getExpirationArray } from "../../data/expiration";
import { RecordContext } from "../context";
import { EditorContext } from "./context";

interface Props {
    hidden?: boolean
}

const EditorForm: React.FC<Props> = ({ hidden }: Props) => {
    const record = useContext(RecordContext);
    const editorContext = useContext(EditorContext);
    const isValid = editorContext.isValid;
    const setIsValid = editorContext.updateIsValid;

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

    const titleInput = (
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
    );


    const langOptions = getLangArray().map((lang) => (
        <option key={lang.value} value={lang.value}>{lang.display}</option>
    ));

    const langSelect = (
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
                {langOptions}
            </Form.Control>
        </Form.Group>
    );

    const expirationOptions = getExpirationArray().map((exp) => (
        <option key={exp.value} value={exp.value}>{exp.display}</option>
    ));

    const expirationSelect = (
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
                {expirationOptions}
            </Form.Control>
        </Form.Group>
    );


    const contentTextAreaRef = useRef<HTMLTextAreaElement>(null);
    useEffect(() => contentTextAreaRef.current?.focus());


    const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
        if (e.keyCode === "\t".charCodeAt(0)) {
            console.log(e);
            e.preventDefault();
        }
    };

    const contentTextArea = (
        <div className={isValid === false ? "was-validated" : undefined}>
            <Form.Group>
                <Form.Label>
                    Content
                </Form.Label>
                <Form.Control
                    as="textarea"
                    rows={15}
                    defaultValue={record.content}
                    isInvalid={isValid === false ? true : undefined}
                    onChange={syncField("content", (s) => {
                        const v = s !== "";
                        if (v !== isValid) { setIsValid(!isValid); }
                        return s;
                    })}
                    required
                    ref={contentTextAreaRef}
                    className="code-area"
                    onKeyDown={handleKeyDown}
                >
                </Form.Control>
                <Form.Control.Feedback type='invalid'>This field is required.</Form.Control.Feedback>
            </Form.Group>
        </div>
    );

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        e.stopPropagation();
    };

    return (
        <Form
            onSubmit={handleSubmit}
            autoComplete="off"
            style={{
                display: hidden ? "none" : undefined,
                width: "100%",
                flexGrow: 1,
            }}
        >
            {titleInput}
            {langSelect}
            {expirationSelect}
            {contentTextArea}
        </Form>
    );
};

export default EditorForm;
