use std::io::{self, Read};

use crate::terminal::{ansi_in::parse, events::Event::*, events::Key::*};

use super::Editor;


impl Editor {
    pub fn event_loop(&mut self) -> io::Result<()> {
        let mut stdin = std::io::stdin();
        let mut buffer = [0; 1024];

        loop {
            let n = stdin.read(&mut buffer).unwrap();
            let input = std::str::from_utf8(&buffer[..n]).unwrap();

            println!("{:?}", input);
            match parse(input) {
                Ok((_, event)) => match event {
                    Key(key) => match key {
                        Esc => break Ok(()),
                        Backspace => println!("Backspace"),
                        Tab => println!("Tab"),
                        Insert => println!("Insert"),
                        Delete => println!("Delete"),
                        Home => println!("Home"),
                        End => println!("End"),
                        PageUp => println!("Page up"),
                        PageDown => println!("Page down"),
                        Up => println!("Arrow up"),
                        Down => println!("Arrow down"),
                        Right => println!("Arrow right"),
                        Left => println!("Arrow left"),
                        Char(c) => println!("Char: {} {}", c, c as u8),
                    },
                },
                Err(err) => eprintln!("Parse error: {}", err),
            }
        }
    }
}
