use picow::editor::{
    navigation::{CursorCommand::{self, *}, ScrollCommand::{self, *}},
    state::{EditorState, AbsPosition, ViewportDimensions}
};

pub struct TestCase {
    pub editor_state: EditorState,
    pub expected_cursor: CursorCommand,
    pub expected_scroll: ScrollCommand,
}

pub fn parse_test_case(input: Vec<&str>) -> TestCase {
    let mut lines = Vec::new();
    let mut eof_found = false;
    let mut eof_reached = false;
    let mut viewport_size: ViewportDimensions = (0, 0);
    let mut cursor_pos: AbsPosition = (0, 0);
    let mut scroll_pos: AbsPosition = (0, 0);
    let mut scroll_pos_identified = false;
    let mut expected_cursor = None;
    let mut expected_scroll = None;

    for (i, line) in input.iter().enumerate() {
        if line.contains("┌") {
            let scroll_left = pos(line, '┌');
            scroll_pos = (scroll_left, i);
            scroll_pos_identified = true;
        }

        if line.starts_with("▮─") && !scroll_pos_identified {
            scroll_pos = (0, i);
            scroll_pos_identified = true;
        }

        if line.contains("└") {
            let scroll_top = scroll_pos.1 as u16;
            let scroll_left = scroll_pos.0 as u16;
            let viewport_width = pos(line, '┘') as u16 - scroll_left + 1;
            let viewport_height = i as u16 - scroll_top + 1;
            viewport_size = (viewport_width, viewport_height);
        }

        if line.contains('▮') {
            let cursor_x = pos(line, '▮');
            let cursor_y = i;
            cursor_pos = (cursor_x, cursor_y);
        }

        if line.contains('╔') {
            let exp_scroll_top = i;
            let exp_scroll_left = pos(line, '╔');
            expected_scroll = Some((exp_scroll_left, exp_scroll_top));
        }

        if line.contains('▯') {
            let exp_cursor_x_abs = pos(line, '▯');
            let exp_cursor_y_abs = i;
            expected_cursor = Some((exp_cursor_x_abs, exp_cursor_y_abs));

            if expected_scroll == None && !scroll_pos_identified {
                expected_scroll = Some((exp_cursor_x_abs, exp_cursor_y_abs));
            }
        }

        if line.contains('☼') {
            eof_found = true;
        }

        if !eof_reached {
            let processed_line = line
                .replace(['▮', '┘'], if line.contains('.') || line.contains('☼') { " " } else { "_" } )
                .replace(['│', '▯'], " ")
                .replace(['┌', '─', '┐', '└', '╔', '▮', '.', '☼'], "_")
                .trim_end()
                .to_string();

            lines.push(processed_line);

            if eof_found { eof_reached = true; }
        }
    }

    if expected_cursor == Some(cursor_pos) {
        expected_cursor = None;
    }

    TestCase {
        editor_state: EditorState::new(lines, viewport_size, scroll_pos, cursor_pos),
        expected_cursor: expected_cursor.map(|pos| MoveTo(pos.0, pos.1)).unwrap_or(NoMove),
        expected_scroll: expected_scroll.map(|pos| ScrollTo(pos.0, pos.1)).unwrap_or(NoScroll),
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

    let state = tc.editor_state;
    assert_eq!(state.viewport_size, (13, 4));
    assert_eq!(state.cursor_pos, (6, 1));
    assert_eq!(state.scroll_pos, (0, 0));
    assert_eq!(state.lines, vec![
        "______ ______",
        " ______",
        " ______",
        "_____________"
    ]);

    assert_eq!(tc.expected_cursor, MoveTo(6, 0));
    assert_eq!(tc.expected_scroll, NoScroll);
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

    let state = tc.editor_state;
    assert_eq!(state.cursor_pos, (6, 1));
    assert_eq!(tc.expected_cursor, NoMove);
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

    let state = tc.editor_state;
    assert_eq!(state.cursor_pos, (0, 1));
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

    let state = tc.editor_state;
    assert_eq!(state.viewport_size, (13, 4));
    assert_eq!(state.cursor_pos, (16, 3));
    assert_eq!(state.scroll_pos, (10, 2));
    assert_eq!(state.lines, vec![
        "_________ ______ ___",
        "_________ __________________",
        "_________ _____________", // 1
        "_________  ______", // 2
        "_________  ___________ _________",
        "_________ _____________"  // 4
    ]);

    assert_eq!(tc.expected_cursor, MoveTo(16, 0));
    assert_eq!(tc.expected_scroll, ScrollTo(10, 0));
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

    let state = tc.editor_state;
    assert_eq!(state.viewport_size, (13, 4));
    assert_eq!(state.cursor_pos, (8, 3));
    assert_eq!(state.scroll_pos, (1, 1));
    assert_eq!(state.lines, vec![
        " _____________",
        "______________",
        "_ _____",
        "_ _______",
        "______________"
    ]);

    assert_eq!(tc.expected_cursor, MoveTo(0, 0));
    assert_eq!(tc.expected_scroll, ScrollTo(0, 0));
}

#[test]
fn eol() {
    let tc = parse_test_case(vec![
    //   01234567890123
        "┌───────────┐",
        "│_____.▮    │",
        "└───────────┘"
    ]);

    let state = tc.editor_state;
    assert_eq!(state.lines, vec![
    //   01234567890123
        "_____________",
        " ______",
        "_____________",
    ]);
    assert_eq!(state.cursor_pos, (7, 1));
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

    let state = tc.editor_state;
    assert_eq!(state.lines, vec![
        "_____________",
        " _____",
        " _______",
        " ___",
    ]);
}
