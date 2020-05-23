use std::io::{BufRead, Read};

use crate::utils::bufwrap::{file_reader, stdin_reader, FileReader, StdinReader};

use clap::{App, Arg};

pub enum BufferedInput {
    File(FileReader),
    Stdin(StdinReader),
}

impl BufferedInput {
    pub fn parse_args(description: &str) -> std::io::Result<Self> {
        let app = App::new("").about(description).arg(
            Arg::with_name("input")
                .value_name("FILE")
                .help("Input file (defaults to STDIN if not provided)"),
        );

        let matches = app.get_matches();

        let result = match matches.value_of("input") {
            Some(path) => BufferedInput::File(file_reader(path)?),
            _ => BufferedInput::Stdin(stdin_reader()),
        };

        Ok(result)
    }
}

impl Read for BufferedInput {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            BufferedInput::File(file_reader) => file_reader.read(buf),
            BufferedInput::Stdin(stdin_reader) => stdin_reader.read(buf),
        }
    }
}

impl BufRead for BufferedInput {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        match self {
            BufferedInput::File(file_reader) => file_reader.fill_buf(),
            BufferedInput::Stdin(stdin_reader) => stdin_reader.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            BufferedInput::File(file_reader) => file_reader.consume(amt),
            BufferedInput::Stdin(stdin_reader) => stdin_reader.consume(amt),
        };
    }
}
