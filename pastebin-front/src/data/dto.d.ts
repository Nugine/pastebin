interface RecordBase {
    title: string;
    lang: string;
    content: string;
    expiration_seconds: number;
}

interface SavedRecordMixin {
    saving_time: number;
    view_count: number;
}

export type PastebinRecord = RecordBase & Partial<SavedRecordMixin>;

export type SaveRecordInput = RecordBase;

export interface SaveRecordOutput {
    key: string;
}

export interface FindRecordInput {
    key: string;
}

export type FindRecordOutput = RecordBase & SavedRecordMixin;

export interface PastebinError {
    code: number;
    message: string;
}
