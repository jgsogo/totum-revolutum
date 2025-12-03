// Even though socket is defined at the top-level of the module,
// it's scoped to that browser tab or client session.
//
// Each time a user opens your SvelteKit site in a new tab (or reloads),
// the browser loads a fresh instance of your JavaScript — including the
// socket variable.

let socket: WebSocket;

const PORT = process.env.PORT || 3000;
const DOMAIN_NAME = process.env.DOMAIN_NAME || "192.168.1.38";  // FIXME: This is my IP!!

export function connectToRoom(roomId: string,
	on_room_update: { (payload: JSON): Promise<void>; (arg0: any): any; },
	on_participants_update: { (payload: JSON): Promise<void>; (arg0: any): any; },
	on_game_update: { (payload: JSON): Promise<void>; (arg0: any): any; },
) {
	const ws_address = `ws://${DOMAIN_NAME}:${PORT}/ws/${roomId}`
	console.log(`[frontend] Connect to ws '${ws_address}'`);
	socket = new WebSocket(ws_address);

	socket.onopen = () => {
		console.log("[frontend] WebSocket connected");
	};

	socket.onmessage = async (event) => {
		const data = JSON.parse(event.data);
		console.log("[frontend] Received event:", data);

		const {type, payload} = data;
		if (type === 'room_update') {
			await on_room_update(payload);
		} else if (type === 'participants_update') {
			await on_participants_update(payload);
		} else if (type === 'game_update') {
			await on_game_update(payload);
		}
	};

	socket.onclose = () => {
		console.warn("[frontend] WebSocket disconnected");
	};

	socket.onerror = (err) => {
		console.error("[frontend] WebSocket error", err);
	};
}
