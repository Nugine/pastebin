import { Key, SaveRecordReq, PastebinRecord, SaveRecordRes, FindRecordRes } from "../index";

const store: Map<string, PastebinRecord> = new Map();

/**
 * @throws {ErrorRes}
 */
export async function saveRecord(
    payload: SaveRecordReq
): Promise<SaveRecordRes> {
    console.log(payload);
    (payload as any).view_count = 0;
    const key = Math.random().toString();
    const rec = Object.assign({ saving_time_seconds: new Date().getTime() / 1000, view_count: 0 }, payload);
    store.set(key, rec);
    return { key };
}

/**
 * @throws {ErrorRes}
 */
export async function findRecord(key: Key): Promise<FindRecordRes> {
    console.log(key);
    const ans = store.get(key)!;
    ans.view_count! += 1;
    return Object.assign({}, ans as any);
}
