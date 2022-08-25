extern crate wild;
use std::io;

use leetcode_practice_queue::{get_queue, save_queue};

fn main() -> Result<(), io::Error> {
    let mut queue = get_queue();

    if queue.is_empty() {
        println!("practice Queue is empty");
        return Ok(());
    }

    // SAFETY: Already ensured that queue was not empty, so popped must be Some
    let popped = queue.pop_front().unwrap();

    // Push to the end
    queue.push_back(popped.clone());

    save_queue(queue)?;

    println!("Snoozed: {}", popped);

    Ok(())
}
