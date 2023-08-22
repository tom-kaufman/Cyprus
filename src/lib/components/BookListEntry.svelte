<script>
    export let id;
    export let title;
    export let author;
    export let cover;

    console.log(`cover=${cover}`);
</script>

<a href="/player/{id}" class="container">
    <div class="thumbnail">
        <img src={cover} alt="Thumbnail" />
    </div>
    <div>
        <li class="text">
            {title} by {author}
        </li>
    </div>
</a>

<style>
    .container {
        display: flex;
        justify-content: center;
        align-items: center;
        color: black;
        align-content: flex-start;
    }

    .container:hover {
        background-color: #f0f0f0;
    }

    .thumbnail {
        margin-right: 10px;
        padding: 10px;
    }

    .thumbnail img {
        width: 50px;
        height: auto;
    }

    .text {
        display: block;
        text-decoration: none;
        color: black;
    }


</style>