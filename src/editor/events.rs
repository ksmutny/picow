use std::io;

use crate::terminal::{
    ansi_in::parse,
    events::{Event::*, Key::*},
    reader::{read_cmd, StdinReader}
};

use super::Editor;


impl Editor {
    pub fn event_loop(&mut self) -> io::Result<()> {
        let mut stdin = StdinReader::new();

        loop {
            let input = read_cmd(&mut stdin)?;
            println!("{:?}", input);

            match parse(&input) {
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
                    Paste(s) => {
                        let lines = s.split("\r");
                        for line in lines {
                            println!("Paste: {}", line);
                        }
                    },
                },
                Err(err) => eprintln!("Parse error: {}", err),
            }
        }
    }
}
