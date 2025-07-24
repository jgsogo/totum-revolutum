// Even though socket is defined at the top-level of the module,
// it's scoped to that browser tab or client session.
//
// Each time a user opens your SvelteKit site in a new tab (or reloads),
// the browser loads a fresh instance of your JavaScript — including the
// socket variable.

let socket: WebSocket;

export function connectToRoom(roomId: string) {
	socket = new WebSocket(`ws://localhost:3000/ws/${roomId}`);

	socket.onopen = () => {
		console.log("WebSocket connected");
	};

	socket.onmessage = (event) => {
		const data = JSON.parse(event.data);
		// Dispatch to Svelte store or trigger event
		console.log("Received event:", data);
	};

	socket.onclose = () => {
		console.warn("WebSocket disconnected");
	};

	socket.onerror = (err) => {
		console.error("WebSocket error", err);
	};
}
