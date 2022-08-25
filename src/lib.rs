use std::{collections::VecDeque, io, path::PathBuf, fs};
use xdg::BaseDirectories;

fn get_store_filepath() -> PathBuf {
    let xdg_dirs = BaseDirectories::with_prefix("lc-todo-queue").unwrap();

    // SAFETY: If .unwrap() fails, we can assume something wrong with user's config files
    //         and we can safely panic.
    let data_dir = xdg_dirs.place_data_file("todo.txt").unwrap();

    data_dir
}


pub fn get_queue() -> VecDeque<String> {
    let mut queue = VecDeque::new();

    let store_filepath = get_store_filepath();

    let lines = fs::read_to_string(&store_filepath)
        .expect(&format!("Unable to read {:?}!", &store_filepath));
    
    for line in lines.lines() {
        queue.push_back(line.to_string());
    }

    queue
}

pub fn save_queue(queue: VecDeque<String>) -> Result<(),io::Error> {
    let store_filepath = get_store_filepath();

    let content = queue.iter().fold(String::new(), |acc, item| acc + "\n" + item);

    fs::write(store_filepath, content)
}

