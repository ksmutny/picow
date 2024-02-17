use std::{io::{self, Read}, str};

use super::{ansi_in::{self, parse}, events::Event};


pub trait Reader {
    fn read(&mut self) -> io::Result<&str>;
}

pub fn read_event() -> io::Result<Event> {
    let mut stdin = StdinReader::new();

    loop {
        let input = read_cmd(&mut stdin)?;
        match parse(&input) {
            Ok((_, event)) => return Ok(event),
            Err(_) => continue,
        }
    }
}

pub fn read_cmd<'a, R: Reader>(reader: &'a mut R) -> io::Result<String> {
    let input = reader.read()?;
    let mut buffer = String::from(input);

    if input == ansi_in::BRACKETED_PASTE_START {
        read_bracketed_paste(reader, &mut buffer)?;
    }

    Ok(buffer)
}

fn read_bracketed_paste<'a, R: Reader>(reader: &mut R, buffer: &'a mut String) -> io::Result<&'a mut String> {
    loop {
        let input = reader.read()?;
        buffer.push_str(input);

        if input.ends_with(ansi_in::BRACKETED_PASTE_END) {
            break;
        }
    }
    Ok(buffer)
}


pub struct StdinReader {
    buffer: [u8; 1024],
}

impl StdinReader {
    pub fn new() -> Self {
        Self { buffer: [0; 1024] }
    }
}

impl Reader for StdinReader {
    fn read(&mut self) -> io::Result<&str> {
        let n = io::stdin().read(&mut self.buffer)?;
        Ok(str::from_utf8(&self.buffer[..n]).unwrap())
    }
}


#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    pub struct MockReader<'a> {
        pub data: VecDeque<&'a str>,
    }

    impl<'a> MockReader<'a> {
        pub fn new(data: Vec<&'a str>) -> Self {
            Self { data: VecDeque::from(data) }
        }
    }

    impl<'a> Reader for MockReader<'a> {
        fn read(&mut self) -> io::Result<&'a str> {
            Ok(self.data.pop_front().unwrap())
        }
    }

    #[test]
    fn parse_event() {
        let mut reader = MockReader::new(vec!["\x1B[A"]);

        let event = read_cmd(&mut reader).unwrap();
        assert_eq!(event, "\x1B[A");
    }

    #[test]
    fn bracketed_paste() {
        let mut reader = MockReader::new(vec![
            "\x1B[200~",
            "Hello, world!",
            "How are you?\x1B[201~",
        ]);

        let event = read_cmd(&mut reader).unwrap();
        assert_eq!(event, "\x1B[200~Hello, world!How are you?\x1B[201~");
    }
}
