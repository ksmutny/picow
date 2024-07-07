use std::{io, cmp::min};

use crate::terminal::{buffer::CommandBuffer, commands::Command::*};
use super::{row::Row, state::EditorState, viewport::Viewport};


pub fn render(state: &EditorState) -> io::Result<()> {
    exec(|commands| {
        hide_cursor(commands);
        if state.marked_for_render {
            render_content(state, commands);
        }
        render_status_bar(state, commands);
        render_cursor(state, commands);
    })
}

fn exec<F>(mut action: F) -> io::Result<()>
where
    F: FnMut(&mut CommandBuffer) -> (),
{
    let mut commands = CommandBuffer::new();
    action(&mut commands);
    commands.execute()
}

fn render_content(state: &EditorState, commands: &mut CommandBuffer) {
    for (i, row) in visible_rows(state).iter().enumerate() {
        render_row(i, row, &state.viewport, commands)
    }
}

fn visible_rows(state: &EditorState) -> &[Row] {
    let Viewport { top, height, .. } = state.viewport;
    let bottom = min(top + height as usize, state.content.lines.len());

    &state.content.lines[top..bottom]
}

fn render_row(i: usize, row: &Row, viewport: &Viewport, commands: &mut CommandBuffer) {
    let part = visible_row_part(row, viewport);

    commands.queue(MoveTo(1, 1 + i as u16));
    commands.queue(Print(part));
    commands.queue(ClearToEndOfLine);
}

fn visible_row_part(row: &Row, viewport: &Viewport) -> String {
    let start = viewport.left;
    if row.len() <= start { return "".to_string() }

    let width = viewport.width as usize;
    let len = min(row.len() - start, width);

    row[start..start + len].to_string()
}

fn render_status_bar(state: &EditorState, commands: &mut CommandBuffer) {
    let Viewport { top, width, height, .. } = state.viewport;
    let (row, col) = state.cursor.pos();

    let status = format!("{}x{} | {} {} | {} | {} | {}", width, height, row + 1, col + 1, top + 1, delimiter_label(&state.content.delimiter), render_selection(&state));

    commands.queue(MoveTo(1, state.viewport.height + 1));
    commands.queue(ClearLine);
    commands.queue(Print(status));
}

fn render_selection(state: &EditorState) -> String {
    match state.selection_pos {
        Some((col, row)) => format!("{} {}", col, row),
        None => "-".to_string()
    }
}

fn hide_cursor(commands: &mut CommandBuffer) {
    commands.queue(HideCursor)
}

fn render_cursor(state: &EditorState, commands: &mut CommandBuffer) {
    let (row, col) = state.cursor.pos();

    if state.viewport.cursor_within((row, col)) {
        let col_2 = state.content.lines[row].mono_col_at(col);
        let (row_rel, col_rel) = state.viewport.to_relative((row, col_2));
        commands.queue(MoveTo(col_rel, row_rel));
        commands.queue(ShowCursor);
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
