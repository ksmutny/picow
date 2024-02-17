use nom::{
    branch::alt,
    bytes::{complete::take_until, streaming::tag},
    character::complete::{digit1, satisfy},
    combinator::{eof, map_res, opt},
    error::Error,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser
};

use super::events::{*, Event::{self, *}, KeyCode::*, Mouse::{self, *}, MouseButton, MouseEvent::*};


pub fn parse(input: &str) -> IResult<&str, Event> {
    alt((
        special_char,
        bracketed_paste.map(Paste),
        preceded(
            tag("\x1B["),
            alt((
                cursor_key,
                special_key,
                mouse.map(Mouse),
            ))
        ),
        unicode_char.map(|c| Key(Char(c), 0)),
    ))(input)
}

pub const BRACKETED_PASTE_START: &str = "\x1B[200~";
pub const BRACKETED_PASTE_END: &str = "\x1B[201~";

pub fn bracketed_paste(input: &str) -> IResult<&str, String> {
    delimited(
        tag(BRACKETED_PASTE_START),
        take_until(BRACKETED_PASTE_END).map(str::to_string),
        tag(BRACKETED_PASTE_END),
    )(input)
}

fn unicode_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| c >= ' ')(input)
}

fn special_char(input: &str) -> IResult<&str, Event> {
    alt((
        preceded(tag("\x1B"), eof).map(|_| Key(Esc, 0)),
        tag("\u{7F}").map(|_| Key(Backspace, 0)),
        tag("\t").map(|_| Key(Tab, 0)),
        tag("\n").map(|_| Key(Enter, 0)),
        tag("\r").map(|_| Key(Enter, 0)),
    ))(input)
}

fn cursor_key(input: &str) -> IResult<&str, Event> {
    tuple((
        opt(preceded(tag("1;"), digit1)).map(key_modifiers),
        alt((
            tag("A").map(|_| Up),
            tag("B").map(|_| Down),
            tag("C").map(|_| Right),
            tag("D").map(|_| Left),
            tag("H").map(|_| Home),
            tag("F").map(|_| End),
        ))
    ))
    .map(|(modifiers, key)| Key(key, modifiers))
    .parse(input)
}

fn key_modifiers(s: Option<&str>) -> u8 {
    match s {
        Some("2") => SHIFT,
        Some("3") => ALT,
        Some("4") => ALT | SHIFT,
        Some("5") => CTRL,
        Some("6") => CTRL | SHIFT,
        Some("7") => CTRL | ALT,
        Some("8") => CTRL | ALT | SHIFT,
        _ => 0,
    }
}

fn special_key(input: &str) -> IResult<&str, Event> {
    terminated(
        tuple((
            alt((
                tag("2").map(|_| Insert),
                tag("3").map(|_| Delete),
                tag("5").map(|_| PageUp),
                tag("6").map(|_| PageDown),
            )),
            opt(preceded(tag(";"), digit1)).map(key_modifiers),
        )),
        tag("~")
    )
    .map(|(key, modifiers)| Key(key, modifiers))
    .parse(input)
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

    parse!(key_x: "x" => Key(Char('x'), 0));
    // parse!(key_esc: "\x1B" => Key(Esc));
    parse!(key_backspace: "\u{7F}" => Key(Backspace, 0));
    parse!(key_tab: "\t" => Key(Tab, 0));
    parse!(key_up: "\x1B[A" => Key(Up, 0));
    parse!(key_down:"\x1B[B" => Key(Down, 0));
    parse!(key_right: "\x1B[C" => Key(Right, 0));
    parse!(key_left: "\x1B[D" => Key(Left, 0));
    parse!(key_home: "\x1B[H" => Key(Home, 0));
    parse!(key_end: "\x1B[F" => Key(End, 0));

    parse!(key_ctrl_up: "\x1B[1;5A" => Key(Up, CTRL));
    parse!(key_alt_down: "\x1B[1;3B" => Key(Down, ALT));
    parse!(key_shift_right: "\x1B[1;2C" => Key(Right, SHIFT));
    parse!(key_ctrl_shift_left: "\x1B[1;6D" => Key(Left, CTRL | SHIFT));
    parse!(key_ctrl_alt_home: "\x1B[1;7H" => Key(Home, CTRL | ALT));
    parse!(key_alt_shift_end: "\x1B[1;4F" => Key(End, ALT | SHIFT));
    parse!(key_control_alt_shift_up: "\x1B[1;8A" => Key(Up, CTRL | ALT | SHIFT));

    parse!(key_insert: "\x1B[2~" => Key(Insert, 0));
    parse!(key_delete: "\x1B[3~" => Key(Delete, 0));
    parse!(key_page_up: "\x1B[5~" => Key(PageUp, 0));
    parse!(key_page_down: "\x1B[6~" => Key(PageDown, 0));

    parse!(key_alt_insert: "\x1B[2;3~" => Key(Insert, ALT));
    parse!(key_shift_delete: "\x1B[3;2~" => Key(Delete, SHIFT));
    parse!(key_ctrl_page_up: "\x1B[5;5~" => Key(PageUp, CTRL));
    parse!(key_ctrl_alt_page_down: "\x1B[6;7~" => Key(PageDown, CTRL | ALT));

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
