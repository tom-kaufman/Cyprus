import { fetchBookById } from '$lib/mock/books.js'

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
    return {
        book_id: params.book_id,
        book: {
            promise: fetchBookById(params.book_id),
        }
    }
}