mod database;

mod book;
use book::add_a_bunch_of_books_to_db;

mod user;
use user::add_a_bunch_of_users_to_db;

mod playback_location;
use playback_location::add_a_bunch_of_playback_times_to_db;

#[tokio::main]
async fn main() {
    let n = 15;
    add_a_bunch_of_books_to_db(true, n).await;
    add_a_bunch_of_users_to_db(false, n).await;
    add_a_bunch_of_playback_times_to_db(false, n, 50).await;
}
