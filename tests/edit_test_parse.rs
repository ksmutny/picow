#[macro_use]
#[path ="./edit_test_macros.rs"]
mod edit_test_macros;

use picow::editor::{content::{EditorContent, LF}, row::Row, state::EditorState, viewport::Viewport};

pub fn state(lines: Vec<&str>) -> EditorState {
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

pub fn assert(state: &EditorState, lines: Vec<&str>) {
    let (rows, cursor) = parse_rows(lines);
    assert_eq!(state.content.lines, rows);
    assert_eq!(state.cursor.pos(), cursor);
}


#[cfg(test)]
mod test {
    use picow::editor::row::Row;

    use super::{parse_row, parse_rows};

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
}
