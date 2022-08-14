use std::{thread, time::Duration};

use database::Database;

fn main() {
    let mut database = Database::new(50, 100, 100);
    loop {
        thread::sleep(Duration::from_millis(500));
        database.advance(1);
    }
}
