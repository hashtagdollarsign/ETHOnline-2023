import { Database } from "@tableland/sdk";
import { ethers } from "ethers";

declare const window: any;


const db: Database<Scores> = new Database();

const getNextId = async (tableName: string) => {
    const {id} = await db.prepare(`SELECT MAX(ID) FROM ${tableName};`).first()
    return id + 1;
}

interface Scores {
    id: number;
    address: string;
    score: number;
}
/**
 * Takes a tableName and returns an array of Scores.
 *
 * @param tableName - A fully qualified table Name: `{prefix}_{chain_id}_{contract_id}`
*/
const getScores = async (tableName: string) => {
    const { results } = await db.prepare(`SELECT * FROM ${tableName};`).all();
    return results;
}

/**
 * Takes a tableName and Score and attempts to post to the Table
 *
 * @param tableName - A fully qualified table Name: `{prefix}_{chain_id}_{contract_id}`
 * @param score - A number representative of the game's native scoring mechanism
 */
const postScore = async (tableName: string, score: number) => {
    const nextId = getNextId(tableName);
    // Grab the provider from the browser
    // Look into replacing this with more generic provider
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    await provider.send("eth_requestAccounts", []);
    const player_address = provider.getSigner();

    const {meta: insert} = await db.prepare('INSERT INTO ${tableName} (id, address, score) VALUES (?,?,?);')
        .bind(nextId, player_address.getAddress(), score)
        .run();
}

export {getScores, postScore}