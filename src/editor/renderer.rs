use std::cmp::min;

use crate::{s, terminal::commands::Command::{self, *}};
use super::{row::Row, state::{EditorState, Selection}, viewport::Viewport};


pub fn render(state: &EditorState, rerender_content: bool) -> Vec<Command> {
    buffer(|commands| {
        hide_cursor(commands);
        if rerender_content {
            render_content(state, commands);
        }
        render_status_bar(state, commands);
        render_cursor(state, commands);
    })
}

fn buffer<F>(mut action: F) -> Vec<Command>
where
    F: FnMut(&mut Vec<Command>) -> (),
{
    let mut commands = Vec::<Command>::new();
    action(&mut commands);
    commands
}

fn render_content(state: &EditorState, commands: &mut Vec<Command>) {
    let selection = state.selection();
    let visible_rows = visible_rows(state);
    let visible_rows_count = visible_rows.len();

    for (i, row) in visible_rows.iter().enumerate() {
        render_row(i, row, &state.viewport, selection, commands)
    }

    for i in visible_rows_count..state.viewport.height as usize {
        clear_row(i as u16 + 1, commands)
    }
}

fn visible_rows(state: &EditorState) -> &[Row] {
    let Viewport { top, height, .. } = state.viewport;
    let bottom = min(top + height as usize, state.content.lines.len());

    &state.content.lines[top..bottom]
}

fn render_row(i: usize, row: &Row, viewport: &Viewport, selection: Selection, commands: &mut Vec<Command>) {
    let (pre_selection, selected, post_selection) = visible_row_part(i, row, viewport, selection);

    commands.push(MoveTo(1, 1 + i as u16));
    commands.push(Print(pre_selection));

    commands.push(SetBackgroundColor(100));
    commands.push(Print(selected));
    commands.push(SetBackgroundColor(0));

    commands.push(Print(post_selection));
    commands.push(ClearToEndOfLine);
}

fn visible_row_part(i: usize, row: &Row, viewport: &Viewport, selection: Selection) -> (String, String, String) {
    let start = viewport.left;
    if row.len() < start { return (s![""], s![""], s![""]) }

    let width = viewport.width as usize;
    let len = min(row.len() - start, width);

    let row_idx = viewport.top + i;
    match selection {
        Some(((sel_start_row, sel_start_col), (sel_end_row, sel_end_col)))
            if row_idx >= sel_start_row && row_idx <= sel_end_row => {
                let sel_start = if row_idx == sel_start_row { sel_start_col + start } else { start };
                let sel_end = if row_idx == sel_end_row { sel_end_col + start } else { row.len() + start };

                let pre_selection = if sel_start > start { s![row[start..sel_start]] } else { s![""] };
                let selected = s![row[sel_start..sel_end]] +
                    if row_idx < sel_end_row && sel_end >= start + len { " " } else { "" };
                let post_selection = if sel_end < start + len { s![row[sel_end..start + len]] } else { s![""] };

                (pre_selection, selected, post_selection)
            },
        _ => (s![row[start..start + len]], s![""], s![""])
    }
}

fn clear_row(row: u16, commands: &mut Vec<Command>) {
    commands.push(MoveTo(1, row));
    commands.push(ClearLine);
}

fn render_status_bar(state: &EditorState, commands: &mut Vec<Command>) {
    let Viewport { top, width, height, .. } = state.viewport;
    let (row, col) = state.cursor.pos();

    let status = format!("{}x{} | {} {} | {} | {}", width, height, row + 1, col + 1, top + 1, delimiter_label(&state.content.delimiter));

    clear_row(state.viewport.height + 1, commands);
    commands.push(Print(status));
}

fn hide_cursor(commands: &mut Vec<Command>) {
    commands.push(HideCursor)
}

fn render_cursor(state: &EditorState, commands: &mut Vec<Command>) {
    let (row, col) = state.cursor.pos();

    if state.viewport.cursor_within((row, col)) {
        let col_2 = state.content.lines[row].mono_col_at(col);
        let (row_rel, col_rel) = state.viewport.to_relative((row, col_2));
        commands.push(MoveTo(col_rel, row_rel));
        commands.push(ShowCursor);
    }
}

fn delimiter_label(delimiter: &str) -> &str {
    use super::content::{CRLF, CR, LF};

    match delimiter {
        CRLF => "CRLF",
        CR => "CR",
        LF => "LF",
        _ => "?"
    }
}
