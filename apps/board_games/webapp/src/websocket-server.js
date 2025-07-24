// Implementation of a custom server: https://svelte.dev/docs/kit/adapter-node#Custom-server
//
// On top of the regular Svelte server, we are adding websockets.
// TODO: Convert this file to typescript .ts

import { createServer } from 'http';
import { handler } from '../build/handler.js';
import { WebSocketServer } from 'ws';
import { parse } from 'url';
import {addToRoom , removeFromRoom} from './websocket_rooms.js';
import {listen_pg_notifications} from './websocket_pg_notifications.js';
import process from 'process';

const server = createServer(handler);

// Create WebSocketServer using the same HTTP server
const wss = new WebSocketServer({ noServer: true });

const connectionString = `postgresql://${process.env.BOARD_GAMES_WEBAPP_SQL_USER}:${process.env.BOARD_GAMES_WEBAPP_SQL_PASSWORD}@${process.env.BOARD_GAMES_WEBAPP_SQL_HOST}:${process.env.BOARD_GAMES_WEBAPP_SQL_PORT}/${process.env.BOARD_GAMES_WEBAPP_SQL_DATABASE}`
const pg_client = listen_pg_notifications(connectionString);
// /** @type {Map<string, Set<WebSocket>>} */
// const rooms = new Map();

wss.on('connection', (ws, request, client) => {
	const { pathname } = parse(request.url, true);
	console.log('Client connected to', pathname);

	const roomId = String(pathname).split("/")[2];
	console.log('Client connected to roomID', roomId);

	addToRoom(roomId, ws);
	// if (!rooms.has(roomId)) {
	// 	rooms.set(roomId, new Set());
	// }
	// rooms.get(roomId).add(ws);

	ws.send(JSON.stringify({ type: 'connected', room: pathname }));

	// On close
	ws.on('close', () => {
		console.log('Client disconnected from roomID', roomId);
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
	console.log(`Server running at http://localhost:${PORT}`);
});
