import React, { useState, useContext } from "react";
import { RecordContext } from "../context";
import { EditorContext } from "./context";
import copy from "copy-to-clipboard";
import * as api from "../../data/api";
import { useHistory } from "react-router-dom";
import EditorBar from "./EditorBar";
import EditorForm from "./EditorForm";
import View from "../show/View";

const Editor: React.FC = () => {
    const record = useContext(RecordContext);

    const [isEditing, setIsEditing] = useState(true);
    const [isValid, setIsValid] = useState<boolean | null>(record.content !== "" ? true : null);


    const handleEdit = () => (!isEditing && setIsEditing(true));

    const handleCopy = () => (copy(record.content));

    const handlePreview = () => {
        if (isEditing) {
            if (isValid) {
                setIsEditing(false);
            } else {
                setIsValid(record.content !== "");
            }
        }
    };

    const history = useHistory();

    const handlePaste = async () => {
        if (!isValid) { setIsValid(false); return; }
        try {
            const res = await api.saveRecord(record);
            history.push(`/${res.key}/`);
        } catch (err) {
            console.error(err);
            setImmediate(() => alert(`Failed: ${err?.message}`));
        }
    };

    const editorBar = (
        <EditorBar
            onEdit={handleEdit}
            onCopy={handleCopy}
            onPreview={handlePreview}
            onPaste={handlePaste}
        ></EditorBar>
    );

    const editorForm = (
        <EditorForm hidden={!isEditing}></EditorForm>
    );

    const editorView = isEditing ? null : (
        <View
            hidden={isEditing}
            title={record.title !== "" ? record.title : undefined}
            lang={record.lang}
            content={record.content}
        />
    );

    return (
        <EditorContext.Provider
            value={{
                isValid,
                updateIsValid: (v) => setIsValid(v)
            }}
        >
            {editorBar}
            {editorForm}
            {editorView}
        </EditorContext.Provider>
    );
};

export default Editor;