use crossterm::event;

use crossterm::{style::Print, terminal, ExecutableCommand};
use std::fs;
use std::io::{self, prelude::*, BufReader, Write};

fn main() {
    let file = fs::File::open("./test_dir/holy_bible.txt").expect("Failed to open file");

    println!("{:?}", terminal::size());
    println!("{:?}", terminal::is_raw_mode_enabled());
    terminal::enable_raw_mode().expect("Could not enable raw mode");

    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    writeln!(io::stdout(), "{:?}", lines).expect("Something wrong with stdout");

    let mut buffer: Vec<String> = vec![];

    let (_width, height) = terminal::size().expect("Could not get terminal size");

    for _ in 0..height {
        if let Some(Ok(line)) = lines.next() {
            buffer.push(line);
        } else {
            break;
        }
    }

    let mut index: usize = 0;

    let mut stdout = io::stdout();

    loop {
        match event::read().expect("Failed to read event") {
            event::Event::Key(event) => match event.code {
                event::KeyCode::Char('c') => {
                    if event.modifiers == event::KeyModifiers::CONTROL {
                        break;
                    }
                }
                event::KeyCode::Up => {
                    if index > 0 {
                        index -= 1;
                    }
                }
                event::KeyCode::Down => {
                    index += 1;
                    while buffer.len() < index + (height as usize) {
                        if let Some(Ok(line)) = lines.next() {
                            buffer.push(line);
                        } else {
                            break;
                        }
                    }
                    stdout
                        .execute(terminal::Clear(terminal::ClearType::All))
                        .unwrap();
                    stdout
                        .execute(Print(
                            buffer[index..(index + (height as usize))]
                                .iter()
                                .map(|line| format!("\r{}\n", line))
                                .collect::<Vec<String>>()
                                .join(""),
                        ))
                        .expect("sth wrong");
                }
                _ => {}
            },
            _ => {}
        }
    }
    terminal::disable_raw_mode().expect("Could not disable raw mode");
    println!("disabled raw mode");
}
