import { Key, SaveRecordReq, SaveRecordRes, FindRecordRes } from "../index";

/**
 * @throws {ErrorRes}
 */
export async function saveRecord(
    payload: SaveRecordReq
): Promise<SaveRecordRes> {
    const url = "/api/records";
    const res = await fetch(url, {
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
    });
    if (res.ok) {
        return res.json();
    } else {
        throw res.json();
    }
}

/**
 * @throws {ErrorRes}
 */
export async function findRecord(key: Key): Promise<FindRecordRes> {
    const url = `/api/records/${key}`;
    const res = await fetch(url);
    if (res.ok) {
        return res.json();
    } else {
        throw res.json();
    }
}
