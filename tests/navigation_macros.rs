#[macro_export]
macro_rules! test_nav {
    ($test_name:ident $action:ident($($args:expr),*); $($line:expr),*) => {
        #[test]
        fn $test_name() {
            let test_case = parse_test_case(vec![$($line),*]);

            let cursor_command = test_case.editor_state.cursor.$action(&test_case.editor_state.content, $($args),*);
            let scroll_command = match cursor_command {
                Some(Cursor { row, col, .. }) => test_case.editor_state.viewport.scroll_into_view((row, col)),
                _ => None
            };

            assert_eq!(cursor_command.map(|cmd| cmd.pos()), test_case.expected_cursor.map(|cmd| cmd.pos()));
            assert_eq!(scroll_command, test_case.expected_scroll);
        }
    };
}
