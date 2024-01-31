use picow::editor::{
    navigation::{CursorCommand::{self, *}, ScrollCommand::{self, *}},
    state::{CursorPosition, EditorState, ScrollPosition, ViewportDimensions}
};

pub struct TestCase {
    pub editor_state: EditorState,
    pub expected_cursor: CursorCommand,
    pub expected_scroll: ScrollCommand,
}

pub fn parse_test_case(input: Vec<&str>) -> TestCase {
    let mut lines = Vec::new();
    let mut viewport_size: ViewportDimensions = (0, 0);
    let mut cursor_pos: CursorPosition = (0, 0);
    let mut scroll_pos: ScrollPosition = (0, 0);
    let mut scroll_pos_identified = false;
    let mut expected_cursor = None;
    let mut expected_scroll = None;

    for (i, line) in input.iter().enumerate() {
        if line.contains("┌") {
            let scroll_top = pos(line, '┌');
            scroll_pos = (scroll_top, i);
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
            let scroll_top = scroll_pos.1 as u16;
            let scroll_left = scroll_pos.0 as u16;
            let cursor_x = pos(line, '▮') as u16 + 1 - scroll_left;
            let cursor_y = i as u16 + 1 - scroll_top;
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

            if let Some((exp_scroll_left, exp_scroll_top)) = expected_scroll {
                let exp_cursor_x = (exp_cursor_x_abs - exp_scroll_left + 1) as u16;
                let exp_cursor_y = (exp_cursor_y_abs - exp_scroll_top + 1) as u16;
                expected_cursor = Some((exp_cursor_x, exp_cursor_y));
            } else if !scroll_pos_identified {
                expected_scroll = Some((exp_cursor_x_abs, exp_cursor_y_abs));
                expected_cursor = Some((1, 1));
            } else {
                let scroll_top = scroll_pos.1;
                let scroll_left = scroll_pos.0;
                let exp_cursor_x = (exp_cursor_x_abs - scroll_left + 1) as u16;
                let exp_cursor_y = (exp_cursor_y_abs - scroll_top + 1) as u16;
                expected_cursor = Some((exp_cursor_x, exp_cursor_y));
            }
        }

        let processed_line = line.replace(['│'], " ").replace(['┌', '─', '┐', '└', '┘', '╔', '▮', '▯'], "_").trim_end().to_string();
        lines.push(processed_line);
    }

    TestCase {
        editor_state: EditorState {
            viewport_size,
            scroll_pos,
            cursor_pos,
            lines,
        },
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
    //   1234567890123
        "┌─────▯─────┐", // 1
        "│_____▮     │", // 2
        "│______     │", // 3
        "└───────────┘"  // 4
    ]);

    let state = tc.editor_state;
    assert_eq!(state.viewport_size, (13, 4));
    assert_eq!(state.cursor_pos, (7, 2));
    assert_eq!(state.scroll_pos, (0, 0));
    assert_eq!(state.lines, vec![
        "_____________",
        " ______",
        " ______",
        "_____________"
    ]);

    assert_eq!(tc.expected_cursor, MoveTo(7, 1));
    assert_eq!(tc.expected_scroll, NoScroll);
}


#[test]
fn move_cursor_and_scroll() {
    let tc = parse_test_case(vec![
    //             1234567890123
        "_________ ╔_____▯___   ",
        "_________ __________________",
        "_________ ┌───────────┐", // 1
        "_________ │_____▮     │", // 2
        "_________ │___________│_________",
        "_________ └───────────┘"  // 4
    ]);

    let state = tc.editor_state;
    assert_eq!(state.viewport_size, (13, 4));
    assert_eq!(state.cursor_pos, (7, 2));
    assert_eq!(state.scroll_pos, (10, 2));
    assert_eq!(state.lines, vec![
        "_________ __________",
        "_________ __________________",
        "_________ _____________", // 1
        "_________  ______", // 2
        "_________  ___________ _________",
        "_________ _____________"  // 4
    ]);

    assert_eq!(tc.expected_cursor, MoveTo(7, 1));
    assert_eq!(tc.expected_scroll, ScrollTo(10, 0));
}

#[test]
fn document_start() {
    let tc = parse_test_case(vec![
    //   1234567890123
        "▯_____________",
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
        "______________",
        "______________",
        "_ _____",
        "_ _______",
        "______________"
    ]);

    assert_eq!(tc.expected_cursor, MoveTo(1, 1));
    assert_eq!(tc.expected_scroll, ScrollTo(0, 0));
}
