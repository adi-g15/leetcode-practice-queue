extern crate wild;
use std::io;

use leetcode_practice_queue::FileStore;

fn main() -> io::Result<()> {
    let mut store = FileStore::open()?;
    let mut queue = store.get_queue()?;

    if queue.is_empty() {
        println!("practice Queue is empty");
        return Ok(());
    }

    // SAFETY: Already ensured that queue was not empty, so popped must be Some
    let popped = queue.pop_front().unwrap();

    // Push to the end
    queue.push_back(popped.clone());

    store.save_queue(queue)?;

    println!("Snoozed: {}", popped);

    Ok(())
}
// ex: shiftwidth=4 expandtab:
