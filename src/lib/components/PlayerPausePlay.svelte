<script>
    import { playing } from "$stores/player";

    $: src = $playing ? '/icons/play.png' : '/icons/pause.png';
</script>

<button on:click={playing.toggle}>
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