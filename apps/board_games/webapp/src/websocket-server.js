// Implementation of a custom server: https://svelte.dev/docs/kit/adapter-node#Custom-server
//
// On top of the regular Svelte server, we are adding websockets.
// TODO: Convert this file to typescript .ts

import { createServer } from 'http';
import { handler } from '../build/handler.js';
import { WebSocketServer } from 'ws';
import { parse } from 'url';
import { addToRoom , removeFromRoom} from './websocket_rooms.js';
import {get_game, get_participants, get_room_data, listen_pg_notifications} from './websocket_pg_notifications.js';
import process from 'process';

const server = createServer(handler);

// Create WebSocketServer using the same HTTP server
const wss = new WebSocketServer({ noServer: true });

const connectionString = `postgresql://${process.env.BOARD_GAMES_WEBAPP_SQL_USER}:${process.env.BOARD_GAMES_WEBAPP_SQL_PASSWORD}@${process.env.BOARD_GAMES_WEBAPP_SQL_HOST}:${process.env.BOARD_GAMES_WEBAPP_SQL_PORT}/${process.env.BOARD_GAMES_WEBAPP_SQL_DATABASE}`
const pg_client = await listen_pg_notifications(connectionString);
// /** @type {Map<string, Set<WebSocket>>} */
// const rooms = new Map();

wss.on('connection', async (ws, request, client) => {
	const { pathname } = parse(request.url, true);
	const roomId = String(pathname).split("/")[2];
	console.log('[backend] Client connected to roomID', roomId);
	ws.send(JSON.stringify({ type: 'connected', room: pathname }));

	console.log('[backend] Send initial status');
	const room = await get_room_data(pg_client, roomId);
	const participants = await get_participants(pg_client, roomId);
	const game = await get_game(pg_client, roomId);
	ws.send(JSON.stringify({ type: 'room_update', payload: room }));
	ws.send(JSON.stringify({ type: 'participants_update', payload: participants }));
	ws.send(JSON.stringify({ type: 'game_update', payload: game }));

	console.log('[backend] Add websocket to room subscribers');
	addToRoom(roomId, ws);

	// On close
	ws.on('close', () => {
		console.log('[backend] Client disconnected from roomID', roomId);
		removeFromRoom(roomId, ws);
		// rooms.get(roomId)?.delete(ws);
	});
});



server.on('upgrade', (req, socket, head) => {
	const { pathname } = parse(req.url, true);

	if (pathname?.startsWith('/ws/')) {
		wss.handleUpgrade(req, socket, head, (ws) => {
			wss.emit('connection', ws, req);
		});
	} else {
		socket.destroy();
	}
});

const PORT = process.env.PORT || 3000;
server.listen(PORT, () => {
	console.log(`[backend] Server running at http://localhost:${PORT}`);
});
