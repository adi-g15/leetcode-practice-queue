use std::{collections::VecDeque, io, path::PathBuf, fs::{self, File}};
use xdg::BaseDirectories;
use fs2::FileExt;
use io::{Read, Write};

fn get_store_filepath() -> PathBuf {
    let xdg_dirs = BaseDirectories::with_prefix("lc-practice-queue").unwrap();

    // SAFETY: If .unwrap() fails, we can assume something wrong with user's config files
    //         and we can safely panic.
    let data_dir = xdg_dirs.place_data_file("practice.txt").unwrap();

    data_dir
}

pub struct FileStore {
    file: File,
    locked: bool
}

impl FileStore {
    pub fn open() -> io::Result<Self> {
        let filepath = get_store_filepath();
        if !filepath.exists() {
            fs::create_dir_all(filepath.parent().unwrap())?;
            File::create(&filepath)?;
        }

        let file = File::open(&filepath)?;

        // Lock the file since we may be writing to it
        file.lock_exclusive()?;

        Ok(FileStore { file, locked: true })
    }

    pub fn get_queue(&mut self) -> io::Result<VecDeque<String>> {
        let mut queue = VecDeque::new();

        let mut lines = String::new();
        self.file.read_to_string(&mut lines)?;
    
        for line in lines.lines() {
            queue.push_back(line.to_string());
        }

        Ok(queue)
    }

    pub fn save_queue(&mut self, queue: VecDeque<String>) -> Result<(),io::Error> {
        let content = queue.iter().fold(String::new(), |acc, item| acc + "\n" + item);

        self.file.write_all(content.as_bytes())
    }

    pub fn close(&mut self) -> io::Result<()> {
        // Release the file lock
        self.file.unlock()?;
        self.locked = false;
        Ok(())
    }
}

impl Drop for FileStore {
    fn drop(&mut self) {
        if self.locked {
            self.close().unwrap();
        }
    }
}

// ex: shiftwidth=4 expandtab:
