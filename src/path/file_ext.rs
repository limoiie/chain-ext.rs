use std::fs::{File, OpenOptions};
use std::path::Path;

/// Extend path-like objects with methods to open, create or remove a file.
pub trait FileExt {
    fn open(&self) -> std::io::Result<File>;
    fn open_with(&self, option: OpenOptions) -> std::io::Result<File>;
    fn create(&self) -> std::io::Result<File>;
    fn remove(&self) -> std::io::Result<()>;
}

impl<P: AsRef<Path>> FileExt for P {
    /// Open file at the path.
    fn open(&self) -> std::io::Result<File> {
        File::open(self)
    }

    /// Open file with given options at the path.
    fn open_with(&self, option: OpenOptions) -> std::io::Result<File> {
        option.open(self)
    }

    /// Create file at the path.
    fn create(&self) -> std::io::Result<File> {
        File::create(self)
    }

    /// Delete file at the path.
    fn remove(&self) -> std::io::Result<()> {
        std::fs::remove_file(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

    #[test]
    fn test_path_buf() {
        let ntf = tempfile::NamedTempFile::new().unwrap();
        test_path(ntf.path().to_path_buf());
    }

    #[test]
    fn test_path_ref() {
        let ntf = tempfile::NamedTempFile::new().unwrap();
        test_path(ntf.path());
    }

    #[test]
    fn test_path_str() {
        let ntf = tempfile::NamedTempFile::new().unwrap();
        test_path(ntf.path().to_str().unwrap())
    }

    fn test_path<P: AsRef<Path>>(p: P) {
        {
            let mut f = p.create().unwrap();
            f.write(&[10, 20]).unwrap();
        }

        assert!(p.as_ref().exists());
        {
            let mut f = p.open().unwrap();
            let mut buf = [];
            f.read(&mut buf).unwrap();
        }

        p.remove().unwrap();
        assert!(!p.as_ref().exists());
    }
}
