use std::io;

use leetcode_practice_queue::FileStore;

fn main() -> io::Result<()> {
    let mut store = FileStore::open()?;
    let mut queue = store.get_queue()?;

    if queue.is_empty() {
        println!("practice Queue is empty");
        return Ok(());
    }

    let popped = queue.pop_front();

    store.save_queue(queue)?;

    // SAFETY: Already ensured that queue was not empty, so popped must be Some
    println!("Popped: {}", popped.unwrap());

    Ok(())
}
// ex: shiftwidth=4 expandtab:
