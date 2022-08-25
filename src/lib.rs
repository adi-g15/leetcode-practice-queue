use std::{collections::VecDeque, io::{self, Seek, BufReader}, path::PathBuf, fs::{self, File, OpenOptions}};
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
            fs::File::create(&filepath)?;
        }

        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(false)        // Don't truncate the file yet, since it contains the queue
            .open(filepath)?;

        // Lock the file since we may be writing to it
        file.lock_exclusive()?;

        Ok(FileStore { file, locked: true })
    }

    pub fn get_queue(&mut self) -> io::Result<VecDeque<String>> {
        let mut queue = VecDeque::new();

        let mut lines = String::new();
        let mut buf_reader = BufReader::new(&mut self.file);
        buf_reader.read_to_string(&mut lines)?;
 
        for line in lines.lines() {
            if line.is_empty() { continue; }

            queue.push_back(line.to_string());
        }

        Ok(queue)
    }

    pub fn save_queue(&mut self, queue: VecDeque<String>) -> Result<(),io::Error> {
        let content = queue.iter().fold(String::new(), |acc, item| acc + item + "\n");

        // .set_len(0) to clear out the earlier content
        self.file.set_len(0)?;

        // .set_len doesn't reset the file's cursors, so using seek
        self.file.seek(io::SeekFrom::Start(0))?;

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
