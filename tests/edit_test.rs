#[cfg(test)]
mod edit_test {
    use picow::editor::{content::{EditorContent, LF}, events::process_event, pos::PosInDocument, row::Row, state::EditorState, viewport::Viewport};
    use picow::terminal::events::{Event::Key, KeyCode::Char};

    macro_rules! s { ($x:expr) => ($x.to_string()); }
    macro_rules! vecr { ($($x:expr),*) => (vec![$(Row::new($x)),*]); }

    fn state(lines: Vec<Row>, cursor: PosInDocument) -> EditorState {
        let content = EditorContent::new(lines, s![LF]);
        let viewport = Viewport::new(0, 0, 80, 25);
        let cursor = cursor;
        EditorState::new(content, viewport, cursor)
    }

    #[test]
    fn test_insert_char() {
        let mut state = state(vecr!["Hllo"], (0, 1));

        process_event(&Key(Char('e'), 0), &mut state);

        assert_eq!(state.content.lines, vecr!["Hello"]);
        assert_eq!(state.cursor.pos(), (0, 2));
    }

    #[test]
    fn test_insert_lf() {
        let mut state = state(vecr!["HelloWorld"], (0, 5));

        process_event(&Key(Char('\n'), 0), &mut state);

        assert_eq!(state.content.lines, vecr!["Hello", "World"]);
        assert_eq!(state.cursor.pos(), (1, 0));
    }
}
