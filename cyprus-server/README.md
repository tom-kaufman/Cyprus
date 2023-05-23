# Stage 2
- The application should use a postgresql database on the localhost. For the release product, it should be using a postgresql database inside the same docker container.
- The database will have the following structure:
    - books table:
        - id (serial) (primary key)
        - name (text)
        - length (time interval)
        - file_location (text)
    - users table:
        - id (serial) (primary key)
        - username (text)
    - playback_locations table:
        - id (serial) (primary key)
        - book_id (serial) (foreign key, books.id)
        - user_id (serial) (foreign key, books.id)
        - time (time interval)
    - along with the provided primary key/foreign key relationships, the following rules should be enforced:
        - in "playback_locations", every record must have a duplicate pair of book_id and user_id
        - in "playback_locations" the value of "time" must have a value less than or equal to the corresponding value for "length" for the corresponding record in the "books" table (i.e., playback_locations.book_id = books.id)
- The database will use the books table to find the location on the disk of the file to serve
- API endpoints:
    - Update the user's playback time on a certain book
    - Download the entire book
    - Download a section of the book, given a timestamp and length
    - Return a list of the books available
    - Return a list of the books available along with the user's playback location in each book
    - Make a user
- Need a way to specify the locations of books,
    - Case by case basis,
    - All compatible files in a list of dirs
    - Maybe this also needs to be in db?


## Database
For quick iteration and easy testing, we'll connect to the PostgreSQL database running on the local host. You must first create the user `my_user` like so:
```
sudo su postgres
psql
CREATE USER my_user WITH PASSWORD 'password';
```

## Tests
To avoid race conditions in the database, run tests like so:
```
cargo test -- --test-threads=1 --nocapture
```