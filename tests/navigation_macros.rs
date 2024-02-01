#[macro_export]
macro_rules! test_nav {
    ($test_name:ident $action:ident($($args:expr),*); $($line:expr),*) => {
        #[test]
        fn $test_name() {
            let test_case = parse_test_case(vec![$($line),*]);

            let (scroll_command, cursor_command) = test_case.editor_state.$action($($args),*);

            assert_eq!(cursor_command, test_case.expected_cursor);
            assert_eq!(scroll_command, test_case.expected_scroll);
        }
    };
}
