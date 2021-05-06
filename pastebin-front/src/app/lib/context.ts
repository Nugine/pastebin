import React from "react";
import { observable } from "mobx";

import { PastebinRecord, langs, expirations } from "./data";

export const defaultRecord: PastebinRecord = observable({
    title: "",
    lang: langs[0].value,
    expiration_seconds: expirations[2].value,
    content: ""
});

export const RecordContext = React.createContext(defaultRecord);
