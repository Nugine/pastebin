import { Key, SaveRecordReq, SaveRecordRes, FindRecordRes } from "./data";

/**
 * @throws {ErrorRes}
 */
export async function saveRecord(
    payload: SaveRecordReq
): Promise<SaveRecordRes> {
    const url = "/api/records";
    const res = await fetch(url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
    });
    if (res.ok) {
        return (await res.json()) as SaveRecordRes;
    } else {
        throw await res.json();
    }
}

/**
 * @throws {ErrorRes}
 */
export async function findRecord(key: Key): Promise<FindRecordRes> {
    const url = `/api/records/${key}`;
    const res = await fetch(url);
    if (res.ok) {
        return (await res.json()) as FindRecordRes;
    } else {
        throw await res.json();
    }
}
