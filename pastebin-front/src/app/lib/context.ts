import { PastebinRecord, langs, expirations } from "./data";
import React from "react";

export const defaultRecord: PastebinRecord = {
    title: "",
    lang: langs[0].value,
    expiration_seconds: expirations[2].value,
    content: ""
};

export const RecordContext = React.createContext(defaultRecord);
