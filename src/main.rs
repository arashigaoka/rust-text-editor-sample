use crossterm::event::{Event, KeyCode, KeyEvent}; /* modify */
use crossterm::{event, terminal};
use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::NONE,
                    } => break,
                    _ => {}
                }
                println!("{:?}\r", event)
            }
        } else {
            println!("No input yet \r");
        }
    }
        Ok(())
    // let mut buf = [0; 1];
    // while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {
    //     let character = buf[0] as char;
    //     if character.is_control() {
    //         println!("{}", character as u8)
    //     } else {
    //         println!("{}", character)
    //     }
    // }
}
