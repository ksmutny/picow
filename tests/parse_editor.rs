#[macro_use]
#[path ="./editor_macros.rs"]
mod editor_macros;

use picow::editor::{
    content::{EditorContent, PosInDocument}, state::EditorState, viewport::{Viewport, ViewportDimensions}, Editor, row::Row
};

pub struct TestCase {
    pub editor: Editor,
    pub expected_cursor: PosInDocument,
    pub expected_scroll: PosInDocument,
    pub expected_selection: Option<PosInDocument>,
}

pub fn parse_test_case(input: Vec<&str>) -> TestCase {
    let mut lines = Vec::new();
    let mut has_selection = false;
    let mut eof_found = false;
    let mut eof_reached = false;
    let mut viewport_size: ViewportDimensions = (0, 0);
    let mut cursor_pos: PosInDocument = (0, 0);
    let mut scroll_pos: PosInDocument = (0, 0);
    let mut scroll_pos_identified = false;
    let mut expected_cursor: Option<PosInDocument> = None;
    let mut expected_scroll: Option<PosInDocument> = None;

    for (i, line) in input.iter().enumerate() {
        if line.contains("┌") {
            let scroll_left = pos(line, '┌');
            scroll_pos = (i, scroll_left);
            scroll_pos_identified = true;
        }

        if line.starts_with("▮─") && !scroll_pos_identified {
            scroll_pos = (i, 0);
            scroll_pos_identified = true;
        }

        if line.contains("└") {
            let scroll_top = scroll_pos.0 as u16;
            let scroll_left = scroll_pos.1 as u16;
            let viewport_width = pos(line, '┘') as u16 - scroll_left + 1;
            let viewport_height = i as u16 - scroll_top + 1;
            viewport_size = (viewport_width, viewport_height);
        }

        if line.contains('▮') {
            let cursor_col = pos(line, '▮');
            let cursor_row = i;
            cursor_pos = (cursor_row, cursor_col);
        }

        if line.contains('╔') {
            let exp_scroll_top = i;
            let exp_scroll_left = pos(line, '╔');
            expected_scroll = Some((exp_scroll_top, exp_scroll_left));
        }

        if line.contains('▯') || line.contains('▯') {
            let exp_cursor_col = if line.contains('▯') { pos(line, '▯') } else { pos(line, '▯') };
            let exp_cursor_row = i;
            expected_cursor = Some((exp_cursor_row, exp_cursor_col));

            if expected_scroll == None && !scroll_pos_identified {
                expected_scroll = expected_cursor;
            }
        }

        if line.contains('☼') {
            eof_found = true;
        }

        if line.contains('▬') {
            has_selection = true;
        }

        if !eof_reached {
            let processed_line = line
                .replace(['▮', '┘'], if line.contains('.') || line.contains('☼') { " " } else { "_" } )
                .replace(['│', '▯', '▯'], " ")
                .replace(['┌', '─', '┐', '└', '╔', '▮', '.', '☼'], "_")
                .trim_end()
                .to_string();

            lines.push(Row::new(&processed_line));

            if eof_found { eof_reached = true; }
        }
    }

    if expected_cursor == None {
        expected_cursor = Some(cursor_pos);
    }
    if expected_scroll == None {
        expected_scroll = Some(scroll_pos);
    }

    let (top, left) = scroll_pos;
    let (width, height) = viewport_size;

    TestCase {
        editor: Editor::new(
            EditorState::new(
                EditorContent::new(lines, "\n".to_string()),
                Viewport::new(left, top, width, height),
                cursor_pos
            )
        ),
        expected_cursor: expected_cursor.unwrap(),
        expected_scroll: expected_scroll.unwrap(),
        expected_selection: if has_selection { Some(cursor_pos) } else { None }
    }
}

fn pos(str: &str, ch: char) -> usize {
    str.chars().position(|c| c == ch).unwrap()
}

#[test]
fn move_cursor_no_scroll() {
    let tc = parse_test_case(vec![
    //   0123456789012
        "┌─────▯─────┐", // 0
        "│_____▮     │", // 1
        "│______     │", // 2
        "└───────────┘"  // 3
    ]);

    let state = tc.editor.state;
    assert_eq!(state.viewport.size(), (13, 4));
    assert_eq!(state.cursor.pos(), (1, 6));
    assert_eq!(state.viewport.pos(), (0, 0));
    assert_eq!(state.content.lines, vecr![
        "______ ______",
        " ______",
        " ______",
        "_____________"
    ]);

    assert_eq!(tc.expected_cursor, (0, 6));
    assert_eq!(tc.expected_scroll, (0, 0));
}

#[test]
fn no_move_cursor() {
    let tc = parse_test_case(vec![
    //   0123456789012
        "┌───────────┐", // 0
        "│_____▮     │", // 1
        "│______     │", // 2
        "└───────────┘"  // 3
    ]);

    let state = tc.editor.state;
    assert_eq!(state.cursor.pos(), (1, 6));
    assert_eq!(tc.expected_cursor, (1, 6));
}

#[test]
fn cursor_top_left() {
    let tc = parse_test_case(vec![
    //   01234567890123
        "________     ", // 0
        "▮───────────┐", // 1
        "│_____      │", // 2
        "│______     │", // 3
        "└───────────┘"  // 4
    ]);

    let state = tc.editor.state;
    assert_eq!(state.cursor.pos(), (1, 0));
}

#[test]
fn move_cursor_and_scroll() {
    let tc = parse_test_case(vec![
    //   012345678901234567890123
        "_________ ╔_____▯___   ",          // 0
        "_________ __________________",     // 1
        "_________ ┌───────────┐",          // 2
        "_________ │_____▮     │",          // 3
        "_________ │___________│_________", // 4
        "_________ └───────────┘"           // 5
    ]);

    let state = tc.editor.state;
    assert_eq!(state.viewport.size(), (13, 4));
    assert_eq!(state.cursor.pos(), (3, 16));
    assert_eq!(state.viewport.pos(), (2, 10));
    assert_eq!(state.content.lines, vecr![
        "_________ ______ ___",
        "_________ __________________",
        "_________ _____________", // 1
        "_________  ______", // 2
        "_________  ___________ _________",
        "_________ _____________"  // 4
    ]);

    assert_eq!(tc.expected_cursor, (0, 16));
    assert_eq!(tc.expected_scroll, (0, 10));
}

#[test]
fn document_start() {
    let tc = parse_test_case(vec![
    //   01234567890123
        "▯_____________", // 0
        "_┌───────────┐", // 1
        "_│_____      │", // 2
        "_│______▮    │", // 3
        "_└───────────┘"  // 4
    ]);

    let state = tc.editor.state;
    assert_eq!(state.viewport.size(), (13, 4));
    assert_eq!(state.cursor.pos(), (3, 8));
    assert_eq!(state.viewport.pos(), (1, 1));
    assert_eq!(state.content.lines, vecr![
        " _____________",
        "______________",
        "_ _____",
        "_ _______",
        "______________"
    ]);

    assert_eq!(tc.expected_cursor, (0, 0));
    assert_eq!(tc.expected_scroll, (0, 0));
}

#[test]
fn eol() {
    let tc = parse_test_case(vec![
    //   01234567890123
        "┌───────────┐",
        "│_____.▮    │",
        "└───────────┘"
    ]);

    let state = tc.editor.state;
    assert_eq!(state.content.lines, vecr![
    //   01234567890123
        "_____________",
        " ______",
        "_____________"
    ]);
    assert_eq!(state.cursor.pos(), (1, 7));
}

#[test]
fn eof() {
    let tc = parse_test_case(vec![
        "┌───────────┐",
        "│_____▯     │",
        "│______▮    │",
        "│__☼        │",
        "│           │",
        "└───────────┘"
    ]);

    let state = tc.editor.state;
    assert_eq!(state.content.lines, vecr![
        "_____________",
        " _____",
        " _______",
        " ___"
    ]);
}
