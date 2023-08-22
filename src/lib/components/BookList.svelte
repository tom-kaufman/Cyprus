<script>
    import BookListEntry from '$components/BookListEntry.svelte';
    export let books;
</script>

{#each books as book, i }
    <BookListEntry {... book} />
    {#if i != books.length}
        <br>
    {/if}
{/each}