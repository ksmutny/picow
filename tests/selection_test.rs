#[macro_use]
#[path ="./edit_test_parse.rs"]
mod edit_test_parse;


mod selection_test {
    use super::edit_test_parse::{assert, state};

    use picow::editor::events::process_event;
    use picow::terminal::events::{Event::Key, KeyCode::*};

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
}
