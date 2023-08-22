import { fetchBooks } from '$lib/mock/books.js'

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
    return {
        books: {
            promise: fetchBooks(),
        }
    }
}