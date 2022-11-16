use std::fs::File;
use std::io::{self, prelude::*, BufReader, Write};

pub struct Reader {
    line_cursor: usize,
    file_buffer: BufReader<File>,
    line_buffer: Vec<String>,
}

pub enum ReaderResult<T> {
    Ok(T),
    Err(ReaderError),
}

pub enum ReaderError {
    InitError(InitError),
}

pub enum InitError {
    FileNotFound,
}

impl Reader {
    pub fn new(filename: &str) -> ReaderResult<Self> {
        let Ok(file) = File::open(filename) else {
            return ReaderResult::Err(ReaderError::InitError(InitError::FileNotFound));
        };
        let file_buffer = BufReader::new(file);

        ReaderResult::Ok(Self {
            line_cursor: 0,
            file_buffer,
            line_buffer: vec![],
        })
    }
}
