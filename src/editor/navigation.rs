#[path="./tuple_ops.rs"]
mod tuple_ops;

use std::cmp;

use tuple_ops::*;

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

fn move_to_abs(editor: &EditorState, new_cursor_pos: ScrollPosition) -> NavigationCommand {
    let (x, y) = new_cursor_pos;

    let scroll_top = editor.scroll_top();
    let new_scroll_top = cmp::min(scroll_top, y);

    let (new_x, new_y) = (
        (x + 1) as u16,
        (y - new_scroll_top + 1) as u16
    );
    let scroll_cmd = if new_scroll_top != scroll_top {
        ScrollTo(0, new_scroll_top)
    } else {
        NoScroll
    };

    (scroll_cmd, MoveTo(new_x, new_y))
}

pub fn move_up(editor: &EditorState, n: usize) -> NavigationCommand {
    let (x, y) = editor.cursor_pos_abs();
    let T(new_pos) = t(x, y) - t(0, n);

    move_to_abs(editor, new_pos)
}
