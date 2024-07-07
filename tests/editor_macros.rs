#[macro_export]
macro_rules! test_editor {
    ($test_name:ident $($event:expr),*; $($line:expr),*) => {
        #[test]
        fn $test_name() {
            let mut test_case = parse_test_case(vec![$($line),*]);

            let events = vec![$($event),*];

            events.into_iter().for_each(|event| {
                process_event(event, &mut test_case.editor.state);
            });

            assert_eq!(test_case.editor.state.cursor.pos(), test_case.expected_cursor);
            assert_eq!(test_case.editor.state.viewport.pos(), test_case.expected_scroll);
            assert_eq!(test_case.editor.state.selection_pos, test_case.expected_selection);
        }
    };
}

#[macro_export]
macro_rules! vecr {
    ($($x:expr),*) => (vec![$(Row::new($x)),*]);
}
