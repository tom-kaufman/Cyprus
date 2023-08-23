<script>
    import { playing } from "$stores/player";

    $: src = $playing ? '/icons/play.png' : '/icons/pause.png';
</script>

<button type="submit" on:click={playing.toggle}>
        <img {src} alt="Play/Pause button" />
</button>

<style>
    button {
        background-color: transparent;
        border: none;
        padding: 0;
    }
</style>