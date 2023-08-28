// Mock calling the Tauri backend later

// TODO replace these with mock data from other books
const mockBooks = [
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
    { 
        id: 1, 
        title: 'The Amateur', 
        author: 'Richard Harding Davis', 
        cover: '/books/Amateur_1211.jpg',
        files: [
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3', 
            '/books/amateur_1206_librivox/amateur_1_davis_64kb.mp3'
        ] 
    },
]

export async function fetchBooks() {
    // Simulate an API call delay
    await new Promise(resolve => setTimeout(resolve, 50));
  
    return mockBooks;
}

export async function fetchBookById(id) {
    // Simulate an API call delay
    await new Promise(resolve => setTimeout(resolve, 50));

    let id_int = parseInt(id, 10);

    if (isNaN(id_int)) {
        throw new Error('Invalid book ID, must be an integer');
    }

    const res = mockBooks.find((book) => book.id === id_int);

    if (res === undefined) {
        throw new Error('Book ID not found');
    } else {
        return res;
    }
}