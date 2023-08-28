<script>
    export let togglePlay;
    export let isPlaying;

    $: src = isPlaying ? '/icons/pause.png' : '/icons/play.png';
</script>

<button on:click={togglePlay}>
        <img {src} alt="Play/Pause button" />
</button>

<style>
    button {
        background-color: transparent;
        border: none;
        padding: 0;
		transform: translate(0, -8px);
		transition: all 0.1s;
    }

    button:active {
		transform: translate(0, -2px);
    }
</style>