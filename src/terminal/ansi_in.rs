use nom::{
    branch::alt,
    bytes::{complete::take_until, streaming::tag},
    character::complete::satisfy,
    combinator::eof,
    sequence::{delimited, preceded},
    IResult, Parser
};

use super::events::{Event::{self, *}, Key::{self, *}};

pub fn parse(input: &str) -> IResult<&str, Event> {
    alt((
        special_char.map(Key),
        bracketed_paste.map(Paste),
        preceded(
            tag("\x1B["),
            special_key.map(Key),
            // alt((
                // parse_mouse_press.map(Event::MousePress),
            // ))
        ),
        unicode_char.map(|c| Key(Char(c))),
    ))(input)
}

pub const BRACKETED_PASTE_START: &str = "\x1B[200~";
pub const BRACKETED_PASTE_END: &str = "\x1B[201~";

pub fn bracketed_paste(input: &str) -> IResult<&str, &str> {
    delimited(
        tag(BRACKETED_PASTE_START),
        take_until(BRACKETED_PASTE_END),
        tag(BRACKETED_PASTE_END),
    )(input)
}

fn unicode_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| c >= ' ')(input)
}

fn special_char(input: &str) -> IResult<&str, Key> {
    alt((
        preceded(tag("\x1B"), eof).map(|_| Key::Esc),
        tag("\u{7F}").map(|_| Key::Backspace),
        tag("\t").map(|_| Key::Tab),
    ))(input)
}

fn special_key(input: &str) -> IResult<&str, Key> {
    alt((
        tag("2~").map(|_| Key::Insert),
        tag("3~").map(|_| Key::Delete),
        tag("H").map(|_| Key::Home),
        tag("F").map(|_| Key::End),
        tag("5~").map(|_| Key::PageUp),
        tag("6~").map(|_| Key::PageDown),
        tag("A").map(|_| Key::Up),
        tag("B").map(|_| Key::Down),
        tag("C").map(|_| Key::Right),
        tag("D").map(|_| Key::Left),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parse {
        ($name:ident: $input:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(parse($input), Ok(("", $expected)));
            }
        };
    }

    parse!(key_x: "x" => Key(Char('x')));
    // parse!(key_esc: "\x1B" => Key(Esc));
    parse!(key_backspace: "\u{7F}" => Key(Backspace));
    parse!(key_tab: "\t" => Key(Tab));
    parse!(key_up: "\x1B[A" => Key(Up));
    parse!(key_down:"\x1B[B" => Key(Down));
    parse!(key_right: "\x1B[C" => Key(Right));
    parse!(key_left: "\x1B[D" => Key(Left));
    parse!(key_insert: "\x1B[2~" => Key(Insert));
    parse!(key_delete: "\x1B[3~" => Key(Delete));
    parse!(key_home: "\x1B[H" => Key(Home));
    parse!(key_end: "\x1B[F" => Key(End));
    parse!(key_page_up: "\x1B[5~" => Key(PageUp));
    parse!(key_page_down: "\x1B[6~" => Key(PageDown));

    parse!(paste: "\x1B[200~Hello World!\x1B[201~" => Paste("Hello World!"));
}
