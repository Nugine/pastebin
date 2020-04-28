import { Key, SaveRecordReq, SaveRecordRes, FindRecordRes } from "../index";

/**
 * @throws {ErrorRes}
 */
export async function saveRecord(
    payload: SaveRecordReq
): Promise<SaveRecordRes> {
    console.log(payload);
    return { key: "mockimpl" };
}

/**
 * @throws {ErrorRes}
 */
export async function findRecord(key: Key): Promise<FindRecordRes> {
    console.log(key);
    return {
        title: "mockimpl",
        lang: "mock",
        expiration_seconds: 0,
        saving_time_seconds: new Date().getTime()/1000,
        content: "mockimpl",
        view_count: 1
    };
}
