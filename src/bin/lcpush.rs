extern crate wild;
use std::io;

use leetcode_practice_queue::{get_queue, save_queue};

fn main() -> Result<(), io::Error> {
    // Skipping 1st argument as it contains executable path
    let urls = wild::args().skip(1);

    if urls.len() == 0 {
        println!("Usage: lcpush <url> [<url> ...]");
        return Ok(());
    }

    let mut queue = get_queue();

    for url in urls {
        queue.push_back(url);
    };

    let len = queue.len();
    save_queue(queue)?;

    println!("Current Length: {}", len);

    Ok(())
}
