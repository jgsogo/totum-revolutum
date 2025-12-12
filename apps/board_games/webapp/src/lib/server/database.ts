import { Client } from 'pg';

// FIXME: I really want to use this same DB connection from the `websocket-server.js` file

const GAME_TYPE_TABLE = "board_games_core_gametype";
const GAMES_TABLE = "board_games_core_game";
const ROOMS_TABLE = "board_games_core_room";

let client: Client = undefined;

export async function connect(connectionString: string): Promise<Client> {
    console.log(`[backend] Connect to PostgreSQL using '${connectionString}'`);
    client = new Client({ connectionString });
    await client.connect();
}

export async function get_game_types(): Promise<{ name: string, slug: string, description: string | undefined }> {
    console.log("[backend] Get game types, only enabled ones")
    const game_types = await client.query(`SELECT * FROM ${GAME_TYPE_TABLE} WHERE enabled=true`);
    return game_types.rows;
}

// export async function get_room_data(room_uuid: string) {
//     const rooms = await client.query(`SELECT * FROM ${ROOMS_TABLE} WHERE id = $1 LIMIT 1`, [room_uuid]);
//     assert(rooms.rows.length == 1, `Room '${room_uuid}' was not retrieved from the DB`);
//     const room = rooms.rows[0];
//     console.log(`[backend] room: ${JSON.stringify(room)}`);
//     return room;
// }

// export async function get_participants(room_uuid: string) {
//     const participants = await client.query(`SELECT * FROM ${PARTICIPANTS_TABLE} WHERE room_id = $1`, [room_uuid]);
//     console.log(`[backend] participants: ${JSON.stringify(participants.rows)}`);
//     return participants.rows;
// }

export async function get_game(room_uuid: string): Promise<{ game_type_id: string } | undefined> {
    console.log("[backend] Get game in room")
    const games = await client.query(`SELECT * FROM ${GAMES_TABLE} WHERE room_id = $1 LIMIT 1`, [room_uuid]);
    if (games.rows.length != 0) {
        const game = games.rows[0];
        return game;
    }
    return undefined;
}

export async function room_exists(room_uuid: string): Promise<boolean> {
    const r = await client.query(`SELECT 1 FROM ${ROOMS_TABLE} WHERE id = $1`, [room_uuid]);
    return r.rows.length != 0;
}
