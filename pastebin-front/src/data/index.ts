export type Key = string;

type RecordBase = {
    title: string;
    lang: string;
    expiration_seconds: number;
    content: string;
};

type SavedBase = {
    saving_time_seconds: number;
    view_count: number;
};

export type PastebinRecord = RecordBase & Partial<SavedBase>;

export type SaveRecordReq = RecordBase;

export interface SaveRecordRes {
    key: Key;
}

export type FindRecordRes = RecordBase & SavedBase;

export interface ErrorRes {
    code: number;
    message: string;
}

export interface Lang {
    value: string;
    display: string;
    ext: string;
}

export interface Expiration {
    value: number;
    display: string;
}

export const PROJECT_NAME = "Nugine Pastebin";
