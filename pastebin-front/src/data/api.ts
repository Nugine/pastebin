import type {
    FindRecordInput,
    FindRecordOutput,
    PastebinError,
    SaveRecordInput,
    SaveRecordOutput,
} from "./dto";
import { mande, type MandeError } from "mande";

const records = mande("/api/records");

export type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };

function resultOk<T, E>(value: T): Result<T, E> {
    return { ok: true, value };
}

function resultErr<T, E>(error: E): Result<T, E> {
    return { ok: false, error };
}

export async function saveRecord(
    input: SaveRecordInput
): Promise<Result<SaveRecordOutput, PastebinError>> {
    try {
        const value: SaveRecordOutput = await records.put(input);
        return resultOk(value);
    } catch (exc) {
        const error = (exc as MandeError<PastebinError>).body;
        return resultErr(error);
    }
}

export async function findRecord(
    input: FindRecordInput
): Promise<Result<FindRecordOutput, PastebinError>> {
    try {
        const value: FindRecordOutput = await records.get(input.key);
        return resultOk(value);
    } catch (exc) {
        const error = (exc as MandeError<PastebinError>).body;
        return resultErr(error);
    }
}
