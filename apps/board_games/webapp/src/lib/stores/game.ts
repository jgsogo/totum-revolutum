import { writable } from 'svelte/store';

export const gameState = writable({
	board: null,
	players: [],
	currentTurn: null,
});
