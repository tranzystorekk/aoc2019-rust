use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Stdin};
use std::path::Path;

pub type FileReader = BufReader<File>;
pub type StdinReader = BufReader<Stdin>;

pub fn file_reader<P: AsRef<Path>>(path: P) -> std::io::Result<FileReader> {
    let file = File::open(path)?;
    let meta = file.metadata()?;

    if meta.is_dir() {
        return Err(Error::new(ErrorKind::Other, "Is a directory"));
    }

    let returned_reader = BufReader::new(file);

    Ok(returned_reader)
}

pub fn stdin_reader() -> StdinReader {
    BufReader::new(std::io::stdin())
}
