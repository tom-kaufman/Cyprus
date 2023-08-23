import { writable } from "svelte/store";

function createPlaying() {
    const { subscribe, set, update } = writable(false);

    return {
        subscribe,
        toggle: () => update((b) => !b)
    }
}

export const playing = createPlaying();