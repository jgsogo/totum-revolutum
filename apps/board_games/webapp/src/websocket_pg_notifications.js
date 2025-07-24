import { Client } from 'pg';
import {broadcastToAllRooms} from './websocket_rooms.js';

export async function listen_pg_notifications(connectionString) {
    const client = new Client({
        connectionString: connectionString
    });

    console.log('Connecting PG client', connectionString);

    await client.connect();
    await client.query('LISTEN game_update');

      // in case of error, close the client as well
        client.on('error', (e) => {
            console.log('PG error', e);
        });

        // when client is closed, open a new one
        client.on('end', (d) => {
            console.log('PG end', d);
        });

    client.on('notification', (msg) => {
        console.log('!!! Notification!!! ')
        const payload = msg.payload;
        const channel = msg.channel;
        console.log(`Notification on ${channel}:`, payload);

        // You can now broadcast this to WebSocket clients here
        broadcastToAllRooms(payload);

    });


    console.log('Listening to PostgreSQL NOTIFY on channel "game_update"...');

    return client;
}
