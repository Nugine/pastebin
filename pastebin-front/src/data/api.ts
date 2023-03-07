import type { FindRecordInput, FindRecordOutput, SaveRecordInput, SaveRecordOutput } from "./dto";
import { mande } from "mande";

const records = mande("/api/records");

export async function saveRecord(input: SaveRecordInput): Promise<SaveRecordOutput> {
    return await records.put(input);
}

export async function findRecord(input: FindRecordInput): Promise<FindRecordOutput> {
    return await records.get(input.key);
}
