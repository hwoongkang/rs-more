use crossterm::{style::Print, terminal, ExecutableCommand};
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Lines, Write};

pub struct Reader {
    pub line_cursor: usize,
    pub file_buffer: Lines<BufReader<File>>,
    pub line_buffer: Vec<String>,
    pub should_close: bool,
    size: (usize, usize),
    stdout: io::Stdout,
}

// TODO: Add error handling
pub fn setup_terminal() -> Option<()> {
    match terminal::enable_raw_mode() {
        Ok(_) => Some(()),
        Err(_) => None,
    }
}

impl Reader {
    // TODO: Add error handling
    pub fn new(filename: &str) -> Option<Self> {
        let file = File::open(filename).expect("Could not open file");
        let file_buffer = BufReader::new(file).lines();

        let Ok(size) = terminal::size() else {
            println!("TODO: Error! Could not get terminal size");
            return None
        };

        let size = (size.0 as usize, size.1 as usize);

        setup_terminal();

        let mut reader = Reader {
            line_cursor: 0,
            file_buffer,
            line_buffer: vec![],
            should_close: false,
            size,
            stdout: io::stdout(),
        };

        reader.draw();

        Some(reader)
    }

    //TODO: Add error handling
    pub fn draw(&mut self) -> Option<()> {
        while self.line_buffer.len() < self.line_cursor + (self.size.1 as usize) {
            if let Some(next_line) = self.file_buffer.next() {
                if let Ok(line) = next_line {
                    self.line_buffer.push(line);
                } else {
                    self.should_close = true;
                    break;
                }
            } else {
                self.should_close = true;
                break;
            }
        }
        let Ok(_) = self.stdout
            .execute(terminal::Clear(terminal::ClearType::CurrentLine)) else {
                return None
            };

        let Ok(_) = self.stdout.execute(Print(
            self.line_buffer[self.line_cursor..self.line_cursor + self.size.1]
                .iter()
                .map(|line| format!("\r{}\n", line))
                .collect::<Vec<String>>()
                .join(""),
        )) else {
            return None
        };

        let Ok(_) = self.stdout.execute(Print("\r:")) else {
            return None
        };

        Some(())
    }
}
