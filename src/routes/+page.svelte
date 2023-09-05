<script>
    import { listen } from '@tauri-apps/api/event';
    import { invoke } from '@tauri-apps/api/tauri';
    import { onMount } from 'svelte';

    import BookList from './BookList.svelte';
    
    function fetchBooks() {
        console.log(`Clicked fetch books`);
        invoke('get_books', {message: null});
    }

    let books = [];

    onMount(async () => {
        await listen("new-books", (new_books) => { 
            console.log(`Payload: ${new_books.payload}`)
            for (let book in new_books.payload) {
                console.log(`New book: ${book}`)
                console.log(`New book name: ${book.name}`)
            }
            books = new_books;
        });
    });

</script>

<button on:click={fetchBooks}>Fetch Books</button>


<BookList {books}/>
