# Cyprus
⚠️ work in progress ⚠️

I listen to a lot of audio books. I listen on my phone with [BookPlayer](). BookPlayer is a great app, but I find myself wishing I could switch over to my desktop and pick up my book where I left off, without having to seek to the same location, and worry about forgetting to do the same on my phone when I'm done listening at my desktop. 

So, I'm making this app. There are 3 (or 3 and a 1/2, or 4, depending on how you want to count the server application front end) major parts:
- Server backend: Rust application that manages a Postgres database and exposes a web API with Axum. The database keeps track of the user's playback location, and can also serve audio files. Long term, this may also serve up short segments of the audio book, not-quite-streaming the file to the player as needed.
- Server frontend: Either a browser page or [egui]() application for controlling the server application, i.e. to add users, add books, etc. 
- Desktop application: A BookPlayer clone for the desktop, that can fetch files and playback locations from the server
- Mobile application: A BookPlayer clone (perhaps even a fork of the original) that can interact with the server

Cyprus is inspired by [Plex]() and [Sunshine](); any individual can host their own instance of Cyprus, and give access to who they choose. It is a decentralized, self-hosted streaming application. 