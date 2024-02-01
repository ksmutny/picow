#[path="./tuple_ops.rs"]
mod tuple_ops;

use std::cmp::{max,min};

use super::state::{CursorPosition, EditorState, ScrollPosition};
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

fn move_to_abs(editor: &EditorState, pos_abs: ScrollPosition) -> NavigationCommand {
    let new_cursor_abs = within_text(editor, pos_abs);
    let new_scroll_pos = scroll_into_view(editor, new_cursor_abs);
    let new_cursor_pos = to_relative(new_cursor_abs, new_scroll_pos);

    (scroll_cmd(editor, new_scroll_pos), move_cmd(editor, new_cursor_pos))
}

fn within_text(editor: &EditorState, (x_abs, y_abs): ScrollPosition) -> ScrollPosition {
    let new_y_abs = min(y_abs, editor.lines.len() - 1);
    let new_x_abs = min(editor.vertical_nav.x(x_abs), editor.lines[new_y_abs].len());
    (new_x_abs, new_y_abs)
}

fn scroll_into_view(editor: &EditorState, (x_abs, y_abs): ScrollPosition) -> ScrollPosition {
    let (scroll_left, scroll_top) = editor.scroll_pos;
    let (width, height) = editor.viewport_usize();

    (
        if x_abs < scroll_left { x_abs }
        else if x_abs >= scroll_left + width { x_abs - width + 1 }
        else { scroll_left },

        if y_abs < scroll_top { y_abs }
        else if y_abs >= scroll_top + height { y_abs - height + 1 }
        else { scroll_top }
    )
}

fn to_relative((x_abs, y_abs): ScrollPosition, (scroll_left, scroll_top): ScrollPosition) -> CursorPosition {
    ((x_abs - scroll_left + 1) as u16, (y_abs - scroll_top + 1) as u16)
}

fn to_absolute((x, y): CursorPosition, (scroll_left, scroll_top): ScrollPosition) -> ScrollPosition {
    (x as usize + scroll_left - 1, y as usize + scroll_top - 1)
}

fn move_cmd(editor: &EditorState, new_pos @ (x, y): CursorPosition) -> CursorCommand {
    if new_pos == editor.cursor_pos { NoMove } else { MoveTo(x, y) }
}

fn scroll_cmd(editor: &EditorState, new_pos @ (x, y): ScrollPosition) -> ScrollCommand {
    if new_pos == editor.scroll_pos { NoScroll } else { ScrollTo(x, y) }
}

pub fn click(editor: &EditorState, x: u16, y: u16) -> NavigationCommand {
    move_to_abs(editor, to_absolute((x, y), editor.scroll_pos))
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

pub fn move_left(editor: &EditorState) -> NavigationCommand {
    let move_to = match editor.cursor_pos_abs() {
        (0, 0) => (0, 0),
        (0, y) => line_end(editor, y - 1),
        (x, y) => (x - 1, y)
    };
    move_to_abs(editor, move_to)
}

pub fn move_right(editor: &EditorState) -> NavigationCommand {
    let (x, y) = editor.cursor_pos_abs();
    let move_to = if x < editor.lines[y].len() {
        (x + 1, y)
    } else if y < editor.lines.len() - 1 {
        (0, y + 1)
    } else {
        (x, y)
    };
    move_to_abs(editor, move_to)
}

pub fn move_line_start(editor: &EditorState) -> NavigationCommand {
    move_to_abs(editor, (0, editor.cursor_y_abs()))
}

pub fn move_line_end(editor: &EditorState) -> NavigationCommand {
    move_to_abs(editor, line_end(editor, editor.cursor_y_abs()))
}

pub fn move_document_start(editor: &EditorState) -> NavigationCommand {
    move_to_abs(editor, (0, 0))
}

pub fn move_document_end(editor: &EditorState) -> NavigationCommand {
    move_to_abs(editor, last_line_end(editor))
}

fn line_end(editor: &EditorState, y: usize) -> ScrollPosition {
    (editor.lines[y].len(), y)
}

fn last_line_end(editor: &EditorState) -> ScrollPosition {
    line_end(editor, editor.lines.len() - 1)
}

pub fn scroll_to(editor: &EditorState, (scroll_left, scroll_top): ScrollPosition) -> NavigationCommand {
    let new_scroll_top = min(scroll_top, editor.lines.len() - 1);
    (scroll_cmd(editor, (scroll_left, new_scroll_top)), NoMove)
}

pub fn scroll_up(editor: &EditorState, n: usize) -> NavigationCommand {
    scroll_vertical(editor, |y| y - min(n, y))
}

pub fn scroll_down(editor: &EditorState, n: usize) -> NavigationCommand {
    scroll_vertical(editor, |y| y + min(n, editor.lines.len() - 1 - y))
}

fn scroll_vertical<F>(editor: &EditorState, new: F) -> NavigationCommand
where
    F: Fn(usize) -> usize,
{
    let (x, y) = editor.scroll_pos;
    scroll_to(editor, (x, new(y)))
}
