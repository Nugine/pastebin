import { PastebinRecord } from "../data/index";
import { getLangArray } from "../data/lang";
import { getExpirationArray } from "../data/expiration";
import React from "react";

export const defaultRecord: PastebinRecord = {
    title: "",
    lang: getLangArray()[0].value,
    expiration_seconds: getExpirationArray()[2].value,
    content: ""
};

export const RecordContext = React.createContext(defaultRecord);
