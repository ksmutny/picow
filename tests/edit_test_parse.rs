#[macro_use]
#[path ="./edit_test_macros.rs"]
mod edit_test_macros;

use picow::editor::{content::{EditorContent, LF}, row::Row, state::EditorState, viewport::Viewport};

pub fn state(lines: Vec<&str>) -> EditorState {
    let (rows, cursor, _) = parse_rows(lines);
    let content = EditorContent::new(rows, s![LF]);
    let viewport = Viewport::new(0, 0, 80, 25);
    EditorState::new(content, viewport, cursor)
}

fn parse_rows(input: Vec<&str>) -> (Vec<Row>, (usize, usize), Option<(usize, usize)>) {
    let mut rows = Vec::new();
    let mut cursor = None;
    let mut selection = None;

    for (i, line) in input.iter().enumerate() {
        let (row, cursor_pos, selection_pos) = parse_row(line);
        rows.push(row);

        if let Some(pos) = cursor_pos {
            cursor = Some((i, pos));
        }
        if let Some(pos) = selection_pos {
            selection = Some((i, pos));
        }
    }

    (rows, cursor.unwrap(), selection)
}

fn parse_row(input: &str) -> (Row, Option<usize>, Option<usize>) {
    let mut row = String::with_capacity(input.len());
    let mut cursor = None;
    let mut selection = None;

    let idx = |i: usize, other: Option<usize>| i - if other.is_some() { 1 } else { 0 };

    for (i, c) in input.chars().enumerate() {
        match c {
            '▮' => cursor = Some(idx(i, selection)),
            '▯' => selection = Some(idx(i, cursor)),
            _ => row.push(c),
        }
    }

    (Row::new(&row), cursor, selection)
}

pub fn assert(state: &EditorState, lines: Vec<&str>) {
    let (rows, cursor, _) = parse_rows(lines);
    assert_eq!(state.content.lines, rows);
    assert_eq!(state.cursor.pos(), cursor);
}


#[cfg(test)]
mod test {
    use picow::editor::row::Row;

    use super::{parse_row, parse_rows};

    #[test]
    fn test_parse_row() {
        let input = "Hello";
        assert_eq!((Row::new("Hello"), None, None), parse_row(input));
    }

    #[test]
    fn test_parse_row_cursor() {
        let input = "Hel▮lo";
        assert_eq!((Row::new("Hello"), Some(3), None), parse_row(input));
    }

    #[test]
    fn test_parse_row_selection() {
        let input = "Hell▯o";
        assert_eq!((Row::new("Hello"), None, Some(4)), parse_row(input));
    }

    #[test]
    fn test_parse_row_cursor_selection() {
        let input = "▮Hell▯o";
        assert_eq!((Row::new("Hello"), Some(0), Some(4)), parse_row(input));
    }

    #[test]
    fn test_parse_row_selection_cursor() {
        let input = "He▯l▮lo";
        assert_eq!((Row::new("Hello"), Some(3), Some(2)), parse_row(input));
    }

    #[test]
    fn test_parse_rows_cursor() {
        let input = vec!["Hello", "W▮orld"];
        assert_eq!((vecr!["Hello", "World"], (1, 1), None), parse_rows(input));
    }

    #[test]
    fn test_parse_rows_selection() {
        let input = vec!["H▯ello", "W▮orld"];
        assert_eq!((vecr!["Hello", "World"], (1, 1), Some((0, 1))), parse_rows(input));
    }
}
