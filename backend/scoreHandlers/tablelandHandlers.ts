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

    await db.prepare('INSERT INTO ${tableName} (id, address, score) VALUES (?,?,?);')
        .bind(nextId, player_address.getAddress(), score)
        .run();
}

interface GameResult {
    player_1_address: string,
    player_2_address: string,
    winner: string, // Address of the winner
    winner_score: number,
    loser_score: number,
}
const postVersusBattle = async (tableName: string, gameResult: GameResult)=> {
    // Grab the provider from the browser
    // Look into replacing this with more generic provider
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    await provider.send("eth_requestAccounts", []);
    const player_address = provider.getSigner();
    const nextId = getNextId(tableName);
    let sql = postBuilder(tableName, gameResult)
    await db.prepare(sql)
        .bind(nextId, Object.keys(gameResult))
        .run();
}

const postBuilder = (tableName: string, insertObj: {}) => {
    const keys = Object.keys(insertObj).join(', ');
    const values = Object.keys(insertObj).map(() => '?').join(', ');

    // Every Post Excepts an ID field.
    let postString = 'INSERT INTO ${tableName} (ID,{keys}) VALUES (?,{values});';
    console.log(postString);
    return postString;
}

export {getScores, postScore, postVersusBattle}