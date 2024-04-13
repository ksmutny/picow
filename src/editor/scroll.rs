use std::cmp::min;

use super::{content::PosInDocument, state::{EditorState, Viewport}};


#[derive(PartialEq, Debug)]
pub struct ScrollViewportTo(pub usize, pub usize);
pub type ScrollCommand = Option<ScrollViewportTo>;

impl EditorState {

    pub fn scroll_into_view(&self, (row, col): PosInDocument) -> ScrollCommand {
        let Viewport { left, top, width, height } = self.viewport;

        let scroll_into = |cursor_pos, viewport_start, viewport_size| {
            if cursor_pos < viewport_start { cursor_pos }
            else if cursor_pos >= viewport_start + viewport_size as usize { cursor_pos - viewport_size as usize + 1 }
            else { viewport_start }
        };

        self.scroll_cmd((scroll_into(row, top, height), scroll_into(col, left, width)))
    }

    pub fn scroll_up(&self, n: usize) -> ScrollCommand {
        self.scroll_vertical(|y| y - min(n, y))
    }

    pub fn scroll_down(&self, n: usize) -> ScrollCommand {
        self.scroll_vertical(|y| y + min(n, self.content.last_line_row() - y))
    }

    fn scroll_vertical<F>(&self, new: F) -> ScrollCommand
    where
        F: Fn(usize) -> usize,
    {
        let Viewport { left, top, .. } = self.viewport;
        self.scroll_to((new(top), left))
    }

    fn scroll_to(&self, (scroll_top, scroll_left): PosInDocument) -> ScrollCommand {
        let new_scroll_top = min(scroll_top, self.content.last_line_row());
        self.scroll_cmd((new_scroll_top, scroll_left))
    }

    fn scroll_cmd(&self, new_pos @ (row, col): PosInDocument) -> ScrollCommand {
        if new_pos == self.viewport.pos() { None } else { Some(ScrollViewportTo(row, col)) }
    }
}
