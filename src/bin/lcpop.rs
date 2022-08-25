use std::io;

use leetcode_practice_queue::{get_queue, save_queue};

fn main() -> Result<(), io::Error> {
    let mut queue = get_queue();

    if queue.is_empty() {
        println!("practice Queue is empty");
        return Ok(());
    }

    let popped = queue.pop_front();

    save_queue(queue)?;

    // SAFETY: Already ensured that queue was not empty, so popped must be Some
    println!("Popped: {}", popped.unwrap());

    Ok(())
}
