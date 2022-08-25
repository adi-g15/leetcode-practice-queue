use std::io;

use leetcode_practice_queue::FileStore;

fn main() -> io::Result<()> {
    let mut store = FileStore::open()?;
    let queue = store.get_queue()?;

    if queue.is_empty() {
        println!("No practices :D");
    } else {
        for (i, practice) in queue.iter().enumerate() {
            println!("{}. {}", i + 1, practice);
        }
    }

    Ok(())
}

// ex: shiftwidth=4 expandtab:
