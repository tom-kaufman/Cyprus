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

    let audio = new Audio(files[0]);
    let isPlaying = false;
    let timeNow = '0:00';
    let timeMax = '0:00';
    audio.ontimeupdate = (event) => {
        timeNow = audio.currentTime;
        timeMax = audio.duration;
    }

    function togglePlay() {
        isPlaying = !isPlaying;
        if (isPlaying) {
            console.log(`playing`);
            audio.play();
        } else {
            console.log(`pausing`);
            audio.pause();
        }
    }
</script>

<div class="player">
    <PlayerCover {cover} />
    <PlayerTitle {title} {author} />
    <PlayerBar {timeNow} {timeMax} />
    <div class="controls">
        <div class="control">
            <PlayerRewind {files} />
        </div>
        <div class="control">
            <PlayerPausePlay {togglePlay} {isPlaying}/>
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
