#[macro_use]
#[path ="./edit_test_parse.rs"]
mod edit_test_parse;


mod edit_test {
    use super::edit_test_parse::{assert, state};

    use picow::editor::events::process_event;
    use picow::terminal::events::{Event::{Key, Paste}, KeyCode::Char};

    edit_test!(
        test_insert_char:
        "H▮llo";
        Key(Char('e'), 0);
        "He▮llo"
    );

    edit_test!(
        test_type:
        "▮";
        Key(Char('H'), 0),
        Key(Char('e'), 0),
        Key(Char('y'), 0);
        "Hey▮"
    );

    edit_test!(
        test_insert_lf:
        "Hello▮World";
        Key(Char('\n'), 0);
        "Hello",
        "▮World"
    );

    edit_test!(
        test_paste:
        "Hello",
        "▮World";
        Paste(s!["Wonderful\n"]);
        "Hello",
        "Wonderful",
        "▮World"
    );
}
