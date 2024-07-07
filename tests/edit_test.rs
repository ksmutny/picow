#[cfg(test)]
mod edit_test {
    use picow::editor::{content::{EditorContent, LF}, events::process_event, row::Row, state::EditorState, viewport::Viewport};
    use picow::terminal::events::{Event::{Key, Paste}, KeyCode::Char};

    macro_rules! s { ($x:expr) => ($x.to_string()); }
    macro_rules! vecr { ($($x:expr),*) => (vec![$(Row::new($x)),*]); }

    fn state(lines: Vec<&str>) -> EditorState {
        let (rows, cursor) = parse_rows(lines);
        let content = EditorContent::new(rows, s![LF]);
        let viewport = Viewport::new(0, 0, 80, 25);
        EditorState::new(content, viewport, cursor)
    }

    fn parse_rows(input: Vec<&str>) -> (Vec<Row>, (usize, usize)) {
        let mut rows = Vec::new();
        let mut cursor = None;

        for (i, line) in input.iter().enumerate() {
            let (row, pos) = parse_row(line);
            rows.push(row);

            if let Some(pos) = pos {
                cursor = Some((i, pos));
            }
        }

        (rows, cursor.unwrap())
    }

    fn parse_row(input: &str) -> (Row, Option<usize>) {
        match input.find('▮') {
            Some(pos) => (Row::new(&input.replace("▮", "")), Some(pos)),
            None => (Row::new(input), None),
        }
    }

    #[test]
    fn test_parse_row() {
        let input = "Hel▮lo";
        assert_eq!((Row::new("Hello"), Some(3)), parse_row(input));
    }

    #[test]
    fn test_parse_rows() {
        let input = vec!["Hello", "W▮orld"];
        assert_eq!((vecr!["Hello", "World"], (1, 1)), parse_rows(input));
    }

    fn assert(state: &EditorState, lines: Vec<&str>) {
        let (rows, cursor) = parse_rows(lines);
        assert_eq!(state.content.lines, rows);
        assert_eq!(state.cursor.pos(), cursor);
    }

    macro_rules! edit_test {
        ($name:ident: $($line_in:expr),*; $($event:expr),*; $($line_exp:expr),*) => {
            #[test]
            fn $name() {
                let mut state = state(vec![$($line_in),*]);
                let events = vec![$($event),*];

                events.into_iter().for_each(|event| {
                    process_event(&event, &mut state);
                });

                assert(&state, vec![$($line_exp),*]);
            }
        };
    }

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
