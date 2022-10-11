use std::fs::{File, OpenOptions};
use std::path::PathBuf;

pub trait FileExt {
    fn open(&self) -> std::io::Result<File>;
    fn open_with(&self, option: OpenOptions) -> std::io::Result<File>;
    fn create(&self) -> std::io::Result<File>;
}

impl FileExt for PathBuf {
    fn open(&self) -> std::io::Result<File> {
        File::open(self)
    }

    fn open_with(&self, option: OpenOptions) -> std::io::Result<File> {
        option.open(self)
    }

    fn create(&self) -> std::io::Result<File> {
        File::create(self)
    }
}
