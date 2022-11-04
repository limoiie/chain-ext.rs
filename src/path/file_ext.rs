use std::fs::{File, OpenOptions};
use std::path::Path;

pub trait FileExt {
    fn open(&self) -> std::io::Result<File>;
    fn open_with(&self, option: OpenOptions) -> std::io::Result<File>;
    fn create(&self) -> std::io::Result<File>;
    fn remove(&self) -> std::io::Result<()>;
}

impl<P: AsRef<Path>> FileExt for P {
    fn open(&self) -> std::io::Result<File> {
        File::open(self)
    }

    fn open_with(&self, option: OpenOptions) -> std::io::Result<File> {
        option.open(self)
    }

    fn create(&self) -> std::io::Result<File> {
        File::create(self)
    }

    fn remove(&self) -> std::io::Result<()> {
        std::fs::remove_file(self)
    }
}
