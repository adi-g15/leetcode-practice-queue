extern crate wild;
use std::io;

use leetcode_practice_queue::FileStore;

fn main() -> io::Result<()> {
    // Skipping 1st argument as it contains executable path
    let urls = wild::args().skip(1);

    if urls.len() == 0 {
        println!("Usage: lcpush <url> [<url> ...]");
        return Ok(());
    }

    let mut store = FileStore::open()?;
    let mut queue = store.get_queue()?;

    for url in urls {
        queue.push_back(url);
    };

    let len = queue.len();
    store.save_queue(queue)?;

    println!("Current Length: {}", len);

    Ok(())
}
// ex: shiftwidth=4 expandtab:
