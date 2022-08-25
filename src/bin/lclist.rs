use std::io;

use leetcode_practice_queue::get_queue;

fn main() {
    let mut queue = get_queue();

    if queue.is_empty() {
        println!("No practices :D");
    } else {
        for (i, practice) in queue.iter().enumerate() {
            println!("{}. {}", i + 1, practice);
        }
    }
}
