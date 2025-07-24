

/** @type {Map<string, Set<WebSocket>>} */
const rooms = new Map();

export function addToRoom(roomId, ws) {
    if (!rooms.has(roomId)) {
		rooms.set(roomId, new Set());
	}
	rooms.get(roomId).add(ws);

    broadcastToRoom(roomId, "new client joined!"); // TODO: Remove this broadcast call
}

export function removeFromRoom(roomId, ws) {
    rooms.get(roomId)?.delete(ws);

    broadcastToRoom(roomId, "some client left!"); // TODO: Remove this broadcast call
}


export function broadcastToRoom(roomId, msg) {
  const clients = rooms.get(roomId);
  if (!clients) return;
  for (const ws of clients) {
    if (ws.readyState === ws.OPEN) {
      ws.send(JSON.stringify({ type: 'broadcast', msg }));
    }
  }
}
