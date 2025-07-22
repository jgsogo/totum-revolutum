// server.js
import { createServer } from 'http';
import { handler } from './build/handler.js';
import { WebSocketServer } from 'ws';
import { parse } from 'url';

const server = createServer(handler);

// Create WebSocketServer using the same HTTP server
const wss = new WebSocketServer({ noServer: true });

wss.on('connection', (ws, request, client) => {
	const { pathname } = parse(request.url, true);
	console.log('Client connected to', pathname);

	ws.send(JSON.stringify({ type: 'connected', room: pathname }));
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
