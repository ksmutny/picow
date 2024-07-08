#[macro_use]
#[path ="./edit_test_parse.rs"]
mod edit_test_parse;


mod selection_test {
    use super::edit_test_parse::{assert, state};

    use picow::editor::events::process_event;
    use picow::terminal::events::{Event::{Key, Paste}, KeyCode::*, CTRL};

    edit_test!(
        test_selection_delete:
        "He▯llo Kitt▮y";
        Key(Delete, 0);
        "He▮y"
    );

    edit_test!(
        test_selection_backspace:
        "He▯llo Kitt▮y";
        Key(Backspace, 0);
        "He▮y"
    );

    edit_test!(
        test_selection_replace_char:
        "He▯llo Kitt▮y";
        Key(Char('i'), 0);
        "Hei▮y"
    );

    edit_test!(
        test_selection_replace_lf:
        "He▯llo Kitt▮y";
        Key(Enter, 0);
        "He",
        "▮y"
    );

    edit_test!(
        test_selection_replace_char_undo:
        "He▯llo Kitt▮y";
        Key(Char('i'), 0),
        Key(Char('Z'), CTRL);
        "Hello Kitt▮y"
    );

    edit_test!(
        test_selection_replace_paste:
        "He▯llo Kitt▮y";
        Paste("llo World".to_string());
        "Hello World▮y"
    );

    edit_test!(
        test_selection_replace_paste_undo:
        "He▯llo Kitt▮y";
        Paste("llo World".to_string()),
        Key(Char('Z'), CTRL);
        "Hello Kitt▮y"
    );
}
