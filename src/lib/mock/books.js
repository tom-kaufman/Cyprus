// Mock calling the Tauri backend later
const mockBooks = [
    { id: 1, title: 'Book 1', author: 'Author 1' },
    { id: 2, title: 'Book 2', author: 'Author 2' },
]

export async function fetchBooks() {
    // Simulate an API call delay
    await new Promise(resolve => setTimeout(resolve, 1500));
  
    return mockBooks;
}