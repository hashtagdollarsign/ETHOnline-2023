import { Database } from "@tableland/sdk";

interface Scores {
    id: number;
    address: string;
    score: number;
}

const db: Database<Scores> = new Database();

/**
 * Takes a tableName and returns an array of Scores.
 *
 * @param tableName - A fully qualified table Name: `{prefix}_{chain_id}_{contract_id}`
*/
const getScores = async (tableName: string) => {
    const { results } = await db.prepare(`SELECT * FROM ${tableName};`).all();
    return results;
}

export {getScores}