<script>
    /** @type {import('./$types').PageData} */
    export let data;

    import BookList from '$components/BookList.svelte';
</script>

{#await data.books.promise}
    <p>loading books...</p>
{:then books}
    <BookList {books}/>
{/await} 
