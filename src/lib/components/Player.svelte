<script>
    // export let id;
    export let title;
    export let author;
    export let cover;
    export let files;

    import PlayerCover from '$components/PlayerCover.svelte';
    import PlayerBar from '$components/PlayerBar.svelte';
    import PlayerTitle from '$components/PlayerTitle.svelte';
    import PlayerRewind from '$components/PlayerRewind.svelte';
    import PlayerPausePlay from '$components/PlayerPausePlay.svelte';
    import PlayerForward from '$components/PlayerForward.svelte';
</script>

<div class="player">
    <PlayerCover {cover} />
    <PlayerTitle {title} {author} />
    <PlayerBar {files} />
    <div class="controls">
        <div class="control">
            <PlayerRewind {files} />
        </div>
        <div class="control">
            <PlayerPausePlay {files} />
        </div>
        <div class="control">
            <PlayerForward {files} />
        </div>            
    </div>
</div>

<style>
    .player {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
    }

    .controls {
        display: flex;
        flex-direction: row;
        justify-content: space-around;
        align-items: center;
    }

    .control {
        padding: 10px;
    }
</style>
