use std::io;

use crate::terminal::{
    ansi_in::parse,
    events::{Event::*, KeyCode::*, Mouse::*},
    reader::{read_cmd, StdinReader}
};

use super::Editor;


impl Editor {
    pub fn event_loop_dummy(&mut self) -> io::Result<()> {
        let mut stdin = StdinReader::new();

        loop {
            let input = read_cmd(&mut stdin)?;
            println!("{:?}", input);

            match parse(&input) {
                Ok((_, event)) => match event {
                    Key(key, m) => {
                        match key {
                            Esc => break Ok(()),
                            Enter => print!("Enter"),
                            Backspace => print!("Backspace"),
                            Tab => print!("Tab"),
                            Insert => print!("Insert"),
                            Delete => print!("Delete"),
                            Home => print!("Home"),
                            End => print!("End"),
                            PageUp => print!("Page up"),
                            PageDown => print!("Page down"),
                            Up => print!("Arrow up"),
                            Down => print!("Arrow down"),
                            Right => print!("Arrow right"),
                            Left => print!("Arrow left"),
                            Char(c) => print!("Char: {} {}", c, c as u32),
                        }
                        println!(", Modifiers: {}", m)
                    },
                    Mouse(mouse) => match mouse {
                        Button(button, event, x, y) => println!("{:?}: {:?} at ({}, {})", event, button, x, y),
                        WheelUp(x, y) => println!("Wheel up at ({}, {})", x, y),
                        WheelDown(x, y) => println!("Wheel down at ({}, {})", x, y),
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
