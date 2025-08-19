import { Client } from 'pg';
import { sendRoomUpdate, sendParticipantsUpdate, sendGameUpdate} from './websocket_rooms.js';
import { assert } from 'console';

const ROOM_UPDATE_CHANNEL = 'room_update';

const ROOMS_TABLE = "board_games_core_room";
const GAMES_TABLE = "board_games_core_game";
const PARTICIPANT_TABLE = "board_games_core_participant";

export async function listen_pg_notifications(connectionString) {
    const client = new Client({
        connectionString: connectionString
    });

    console.log('[backend] Connecting PG client', connectionString);

    await client.connect();
    await client.query(`LISTEN ${ROOM_UPDATE_CHANNEL}`);

    // in case of error, close the client as well
    client.on('error', (e) => {
        console.log('PG error', e);
    });

    // when client is closed, open a new one
    client.on('end', (d) => {
        console.log('PG end', d);
    });

    client.on('notification', async (msg) => {
        console.log(`[backend] Notification on ${msg.channel}:`, msg.payload);
        if (msg.channel === ROOM_UPDATE_CHANNEL) {
            const room_uuid = msg.payload;

            const room = await get_room_data(client, room_uuid);
            sendRoomUpdate(room_uuid, room);

            const participants = await get_participants(client, room_uuid);
            sendParticipantsUpdate(room_uuid, participants);

            const game = await get_game(client, room_uuid);
            sendGameUpdate(room_uuid, game);
        }
    });


    console.log('[backend] Listening to PostgreSQL NOTIFY on channel "room_update"...');

    return client;
}

export async function get_room_data(client, room_uuid) {
    const rooms = await client.query(`SELECT * FROM ${ROOMS_TABLE} WHERE id = $1 LIMIT 1`, [room_uuid]);
    assert(rooms.rows.length == 1, `Room '${room_uuid}' was not retrieved from the DB`);
    const room = rooms.rows[0];
    console.log(`[backend] room: ${JSON.stringify(room)}`);
    return room;
}

export async function get_participants(client, room_uuid) {
    const participants = await client.query(`SELECT * FROM ${PARTICIPANT_TABLE} WHERE room_id = $1`, [room_uuid]);
    console.log(`[backend] participants: ${JSON.stringify(participants.rows)}`);
    return participants.rows;
}

export async function get_game(client, room_uuid) {
    const games = await client.query(`SELECT * FROM ${GAMES_TABLE} WHERE room_id = $1 LIMIT 1`, [room_uuid]);
    if (games.rows.length != 0) {
        const game = games.rows[0];
        console.log(`[backend] game: ${JSON.stringify(game)}`);
        return game;
    }
    return undefined;
}
