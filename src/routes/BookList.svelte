<script>
    export let books;
</script>

{#each books as book, i }
    <a href="/player/{book.id}" class="container">
        <div class="thumbnail">
            <img src={book.cover} alt="Thumbnail" />
        </div>
        <div>
            <li class="text">
                {book.title} by {book.author}
            </li>
        </div>
    </a>
    <br>
{/each}

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