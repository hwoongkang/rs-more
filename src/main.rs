use crossterm::event;
use crossterm::terminal;
use std::fs;

fn main() {
    let bible = fs::File::open("./test_dir/holy_bible.txt").expect("Failed to open file");

    println!("{:?}", terminal::size());
    println!("{:?}", terminal::is_raw_mode_enabled());
    terminal::enable_raw_mode().expect("Could not enable raw mode");

    loop {
        match event::read().expect("Failed to read event") {
            event::Event::Key(event) => match event.code {
                event::KeyCode::Char('c') => {
                    if event.modifiers == event::KeyModifiers::CONTROL {
                        break;
                    }
                }
                event::KeyCode::Up => {
                    println!("Up");
                }
                _ => {
                    println!("Don't care");
                }
            },
            _ => {
                println!("don't care")
            }
        }
    }
    terminal::disable_raw_mode().expect("Could not disable raw mode");
}
