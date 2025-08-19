import { Client } from 'pg';

// FIXME: I really want to use this same DB connection from the `websocket-server.js` file

const GAME_TYPE_TABLE = "board_games_core_gametype";

let client: Client = undefined;

export async function connect(connectionString: string): Promise<Client> {
    console.log(`[backend] Connect to PostgreSQL using '${connectionString}'`);
    client = new Client({ connectionString });
    await client.connect();
}

export async function get_game_types(): Promise<{ name: string, slug: string, description: string | undefined }> {
    console.log("[backend] Get game types")
    const game_types = await client.query(`SELECT * FROM ${GAME_TYPE_TABLE}`);
    return game_types.rows;
}
