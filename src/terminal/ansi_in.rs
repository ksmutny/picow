use nom::{
    branch::alt,
    bytes::{complete::take_until, streaming::tag},
    character::complete::{digit1, satisfy},
    combinator::{eof, map_res},
    error::Error,
    sequence::{delimited, preceded, tuple},
    IResult, Parser
};

use super::events::{Event::{self, *}, Key::{self, *}, Mouse::{self, *}, MouseButton, MouseEvent::*};


pub fn parse(input: &str) -> IResult<&str, Event> {
    alt((
        special_char.map(Key),
        bracketed_paste.map(Paste),
        preceded(
            tag("\x1B["),
            alt((
                special_key.map(Key),
                mouse.map(Mouse),
            ))
        ),
        unicode_char.map(|c| Key(Char(c))),
    ))(input)
}

pub const BRACKETED_PASTE_START: &str = "\x1B[200~";
pub const BRACKETED_PASTE_END: &str = "\x1B[201~";

pub fn bracketed_paste(input: &str) -> IResult<&str, String> {
    delimited(
        tag(BRACKETED_PASTE_START),
        take_until(BRACKETED_PASTE_END),
        tag(BRACKETED_PASTE_END),
    )(input).map(|(rest, paste)| (rest, paste.to_string()))
}

fn unicode_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| c >= ' ')(input)
}

fn special_char(input: &str) -> IResult<&str, Key> {
    alt((
        preceded(tag("\x1B"), eof).map(|_| Key::Esc),
        tag("\u{7F}").map(|_| Key::Backspace),
        tag("\t").map(|_| Key::Tab),
        tag("\n").map(|_| Key::Enter),
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

fn mouse(input: &str) -> IResult<&str, Mouse> {
    let (rest, (button, x, y, event)) = preceded(
        tag("<"),
        tuple((
            map_res(digit1, str::parse::<u8>),
            preceded(tag(";"), map_res(digit1, str::parse::<u16>)),
            preceded(tag(";"), map_res(digit1, str::parse::<u16>)),
            alt((tag("M"), tag("m"))),
        )),
    )(input)?;

    let mouse = match button {
        64 => WheelUp(x, y),
        65 => WheelDown(x, y),
        _ => Button(
            match button & 0b11 {
                0 => MouseButton::Left,
                1 => MouseButton::Middle,
                2 => MouseButton::Right,
                _ => return Err(nom::Err::Error(Error { input, code: nom::error::ErrorKind::Char })),
            },
            match (button & 0b00100000, event) {
                (0, "M") => Press,
                (0, "m") => Release,
                (32, "M") => Drag,
                (32, "m") => Drag,
                _ => return Err(nom::Err::Error(Error { input, code: nom::error::ErrorKind::Char })),
            },
            x,
            y,
        ),
    };

    Ok((rest, mouse))
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

    parse!(paste: "\x1B[200~Hello World!\x1B[201~" => Paste("Hello World!".to_string()));

    parse!(mouse_left_press: "\x1B[<0;128;43M" => Mouse(Button(MouseButton::Left, Press, 128, 43)));
    parse!(mouse_middle_press: "\x1B[<1;1;12M" => Mouse(Button(MouseButton::Middle, Press, 1, 12)));
    parse!(mouse_right_press: "\x1B[<2;98;443M" => Mouse(Button(MouseButton::Right, Press, 98, 443)));

    parse!(mouse_left_drag: "\x1B[<32;128;43M" => Mouse(Button(MouseButton::Left, Drag, 128, 43)));
    parse!(mouse_middle_drag: "\x1B[<33;1;12M" => Mouse(Button(MouseButton::Middle, Drag, 1, 12)));
    parse!(mouse_right_drag: "\x1B[<34;98;443M" => Mouse(Button(MouseButton::Right, Drag, 98, 443)));

    parse!(mouse_left_release: "\x1B[<0;128;43m" => Mouse(Button(MouseButton::Left, Release, 128, 43)));
    parse!(mouse_middle_release: "\x1B[<1;1;12m" => Mouse(Button(MouseButton::Middle, Release, 1, 12)));
    parse!(mouse_right_release: "\x1B[<2;98;443m" => Mouse(Button(MouseButton::Right, Release, 98, 443)));

    parse!(mouse_wheel_up: "\x1B[<64;128;43M" => Mouse(WheelUp(128, 43)));
    parse!(mouse_wheel_down: "\x1B[<65;1;12M" => Mouse(WheelDown(1, 12)));
}
