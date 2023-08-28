<script>
    // export let id;
    export let title;
    export let author;
    export let cover;
    export let files;

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

    $: play_pause_src = isPlaying ? '/icons/pause.png' : '/icons/play.png';
</script>

<div class="player">
    <img src={cover} alt="Cover Art" />
    <p>{title} by {author}</p>
    <p>{timeNow} / {timeMax}</p>
    <div class="controls">
        <div class="control">
            <button>
                <img src='/icons/rewind.png' alt="Rewind button" />
            </button>
        </div>
        <div class="control">
            <button on:click={togglePlay}>
                <img src={play_pause_src} alt="Play/Pause button" />
            </button>
        </div>
        <div class="control">
            <button>
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

    .controls {
        display: flex;
        flex-direction: row;
        justify-content: space-around;
        align-items: center;
    }

    .control {
        padding: 10px;
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
