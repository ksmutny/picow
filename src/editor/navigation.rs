#[path="./tuple_ops.rs"]
mod tuple_ops;

use std::cmp::{max,min};

use super::state::{EditorState, ScrollPosition};
use CursorCommand::*;
use ScrollCommand::*;


#[derive(PartialEq, Debug)]
pub enum CursorCommand {
    NoMove,
    MoveTo(u16, u16)
}

#[derive(PartialEq, Debug)]
pub enum ScrollCommand {
    NoScroll,
    ScrollTo(usize, usize)
}

pub type NavigationCommand = (ScrollCommand, CursorCommand);

fn move_to_abs(editor: &EditorState, new_cursor_pos_abs: ScrollPosition) -> NavigationCommand {
    let (x_abs, y_abs) = new_cursor_pos_abs;
    let line_len = editor.lines[y_abs].len();

    let scroll_top = editor.scroll_top();
    let height = editor.viewport_height() as usize;

    let new_scroll_top = if y_abs < scroll_top {
        y_abs
    } else if y_abs >= scroll_top + height {
        y_abs - height + 1
    } else {
        scroll_top
    };

    let (new_x, new_y) = (
        (min(editor.vertical_nav.x(x_abs), line_len) + 1) as u16,
        (y_abs - new_scroll_top + 1) as u16
    );
    let scroll_cmd = if new_scroll_top != scroll_top {
        ScrollTo(0, new_scroll_top)
    } else {
        NoScroll
    };

    (scroll_cmd, MoveTo(new_x, new_y))
}

pub fn move_up(editor: &EditorState, n: usize) -> NavigationCommand {
    move_vertical(editor, |y| y - min(n, y))
}

pub fn move_down(editor: &EditorState, n: usize) -> NavigationCommand {
    move_vertical(editor, |y| y + min(n, editor.lines.len() - 1 - y))
}

fn move_vertical<F>(editor: &EditorState, new: F) -> NavigationCommand
where
    F: Fn(usize) -> usize,
{
    let (x, y) = editor.cursor_pos_abs();
    move_to_abs(editor, (x, new(y)))
}

pub fn move_document_start(editor: &EditorState) -> NavigationCommand {
    move_to_abs(editor, (0, 0))
}

pub fn move_document_end(editor: &EditorState) -> NavigationCommand {
    let lines = editor.lines.len();
    let last_line_len = editor.lines[lines - 1].len();

    move_to_abs(editor, (last_line_len, lines - 1))
}
