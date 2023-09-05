<script>
    import { listen } from '@tauri-apps/api/event';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';

    let coffees = 0;
    
    function toggleCoffees() {
        console.log(`Clicked toggle coffee button`);
        invoke('toggle_coffees', {message: null});
    }

    let books = [];

    onMount(async () => {
        await listen("coffee-gained", () => { coffees += 1});
    });
</script>

<p>This will be the Profile menu. For now, it's testing for Tauri stuff.</p>

<button on:click={toggleCoffees}>Toggle Coffee Buying</button>
<p> We've got {coffees} coffees!</p>

