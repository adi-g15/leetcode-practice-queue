use std::io;

use leetcode_todo_queue::get_queue;

fn main() {
    let mut queue = get_queue();

    if queue.is_empty() {
        println!("No TODOs :D");
    } else {
        for (i, todo) in queue.iter().enumerate() {
            println!("{}. {}", i + 1, todo);
        }
    }
}
