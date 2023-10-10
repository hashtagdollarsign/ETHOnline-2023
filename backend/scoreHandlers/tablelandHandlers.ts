import { Database } from "@tableland/sdk";
// import { Signer } from "ethers";

declare const window: any;

// async function connectSigner(): Promise<Signer> {
//     // Establish a connection with the browser wallet's provider.
//     const provider = new window.ethereum;
//     // Request the connected accounts, prompting a browser wallet popup to connect.
//     await provider.send("eth_requestAccounts", []);
//     // Create a signer from the returned provider connection.
//     const signer = provider.getSigner();
//     // Return the signer
//     return signer;
// }

interface Scores {
    id: number;
    address: string;
    score: number;
}

const db: Database<Scores> = new Database();

const getNextId = async (tableName: string) => {
    const {id} = await db.prepare(`SELECT MAX(ID) FROM ${tableName};`).first()
    return id + 1;
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

const postScore = async (tableName: string) => {
    const nextId = getNextId(tableName);
    const {meta: insert} = await db.prepare('INSERT INTO ${tableName} (id, address, score) VALUES (?,?,?);')
        .bind(nextId, )
        .run();
}

export {getScores}