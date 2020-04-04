use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind, Read, Stdin};
use std::path::Path;

pub struct WrappedBufReader<T> {
    reader: BufReader<T>,
}

impl<T: Read> Read for WrappedBufReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<T: Read> BufRead for WrappedBufReader<T> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt);
    }
}

pub type FileReader = WrappedBufReader<File>;
pub type StdinReader = WrappedBufReader<Stdin>;

impl FileReader {
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let meta = file.metadata()?;

        if meta.is_dir() {
            return Err(Error::new(ErrorKind::Other, "Is a directory"));
        }

        let returned_reader = FileReader {
            reader: BufReader::new(file),
        };

        Ok(returned_reader)
    }
}

impl StdinReader {
    pub fn new() -> Self {
        StdinReader {
            reader: BufReader::new(io::stdin()),
        }
    }
}
