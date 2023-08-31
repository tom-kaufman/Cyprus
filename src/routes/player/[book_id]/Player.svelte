<script>
    // export let id;
    export let title;
    export let author;
    export let cover;
    export let files;

    let audio = new Audio(files[0]);

    // Play/Pause controls
    let isPlaying = false;
    $: play_pause_src = isPlaying ? '/icons/pause.png' : '/icons/play.png';
    function togglePlay() {
        isPlaying = !isPlaying;
        if (isPlaying) {
            audio.play();
        } else {
            audio.pause();
        }
    }

    // Playback time
    let timeMax = 0;
    let timeMaxStr = '00:00'
    let timeNow = 0;
    let timeNowStr = '00:00'
    audio.onloadeddata = () => {
        timeMax = audio.duration;
        timeMaxStr = `${(audio.duration > 3600) ? (String(Math.floor(audio.duration / 3600)) + ':') : ''}${String(Math.floor((audio.duration % 3600) / 60)).padStart(2, '0')}:${String(Math.floor(audio.duration % 60)).padStart(2, '0')}`;
    }
    audio.ontimeupdate = () => {
        timeNow = audio.currentTime;
        timeNowStr = `${(audio.currentTime > 3600) ? (String(Math.floor(audio.currentTime / 3600)) + ':') : ''}${String(Math.floor((audio.currentTime % 3600) / 60)).padStart(2, '0')}:${String(Math.floor(audio.currentTime % 60)).padStart(2, '0')}`;
    }

    // Skip/rewind
    function skipForward() {
        audio.currentTime = Math.min(audio.currentTime + 30.0, audio.duration);
    }
    function skipBackward() {
        audio.currentTime = Math.max(audio.currentTime - 30.0, 0);
    }
</script>

<div class="player">
    <img src={cover} alt="Cover Art" />

    <p>{title} by {author}</p>

    <input type="range" id="seek" name="seek" min=0 max={timeMax} bind:value={audio.currentTime}/>
    <div class="playback">
        <p>{timeNowStr}</p> <p>{timeMaxStr}</p>
    </div>

    <div class="controls">
        <div class="control">
            <button on:click={skipBackward}>
                <img src='/icons/rewind.png' alt="Rewind button" />
            </button>
        </div>
        <div class="control">
            <button on:click={togglePlay}>
                <img src={play_pause_src} alt="Play/Pause button" />
            </button>
        </div>
        <div class="control">
            <button on:click={skipForward}>
                <img src='/icons/forward.png' alt="Fast forward button" />
            </button>
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

    .playback {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;
        width: 360px;
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

    input[type="range"] {
        width: 360px;
    }
    
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
