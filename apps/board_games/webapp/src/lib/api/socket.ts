// Even though socket is defined at the top-level of the module,
// it's scoped to that browser tab or client session.
//
// Each time a user opens your SvelteKit site in a new tab (or reloads),
// the browser loads a fresh instance of your JavaScript — including the
// socket variable.

let socket: WebSocket;

export function connectToRoom(roomId: string,
	on_room_update: { (payload: JSON): Promise<void>; (arg0: any): any; },
	on_participants_update: { (payload: JSON): Promise<void>; (arg0: any): any; },
	on_game_update: { (payload: JSON): Promise<void>; (arg0: any): any; },
) {
	socket = new WebSocket(`ws://localhost:3000/ws/${roomId}`);

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
