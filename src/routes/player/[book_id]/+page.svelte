<script>
    export let data;
    import Player from '$components/Player.svelte'
</script>

{#await data.book.promise}
    <p>Attempting to load book with id {data.book_id}...</p>
{:then book}
    <Player {... book} />
{:catch error}
    <p style="color: red">{error}</p>
{/await}