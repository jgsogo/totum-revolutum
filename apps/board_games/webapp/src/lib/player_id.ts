import { persisted } from 'svelte-persisted-store'
import { get, type Writable } from 'svelte/store'
import { browser } from "$app/environment"

const player_id_key = 'board_games:player_id';
const player_id_store: Writable<string> = getPlayerId();
export const player_id = get(player_id_store);

function getPlayerId(): Writable<string> {
    if (!browser) return; //ONLY CLIENT SIDE!!!!

    const initial_value = crypto.randomUUID();
    if (!localStorage.getItem(player_id_key)) {
        localStorage.setItem(player_id_key, initial_value);
    }
    const serializer = {
        stringify: (value: string) => value,
        parse: (json: string) => json,
    }

    return persisted<string>(player_id_key, initial_value, { serializer });
}
