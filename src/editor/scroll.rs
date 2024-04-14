use std::cmp::min;

use super::{content::{EditorContent, PosInDocument}, state::Viewport};


#[derive(PartialEq, Debug)]
pub struct ScrollViewportTo(pub usize, pub usize);
pub type ScrollCommand = Option<ScrollViewportTo>;

impl Viewport {

    pub fn scroll_into_view(&self, (row, col): PosInDocument) -> ScrollCommand {
        let scroll_into = |cursor_pos, viewport_start, viewport_size| {
            if cursor_pos < viewport_start { cursor_pos }
            else if cursor_pos >= viewport_start + viewport_size as usize { cursor_pos - viewport_size as usize + 1 }
            else { viewport_start }
        };

        self.scroll_cmd((scroll_into(row, self.top, self.height), scroll_into(col, self.left, self.width)))
    }

    pub fn scroll_up(&self, content: &EditorContent, n: usize) -> ScrollCommand {
        self.scroll_vertical(content, |y| y - min(n, y))
    }

    pub fn scroll_down(&self, content: &EditorContent, n: usize) -> ScrollCommand {
        self.scroll_vertical(content, |y| y + min(n, content.last_line_row() - y))
    }

    fn scroll_vertical<F>(&self, content: &EditorContent, new: F) -> ScrollCommand
    where
        F: Fn(usize) -> usize,
    {
        self.scroll_to(content, (new(self.top), self.left))
    }

    fn scroll_to(&self, content: &EditorContent, (scroll_top, scroll_left): PosInDocument) -> ScrollCommand {
        let new_scroll_top = min(scroll_top, content.last_line_row());
        self.scroll_cmd((new_scroll_top, scroll_left))
    }

    fn scroll_cmd(&self, new_pos @ (row, col): PosInDocument) -> ScrollCommand {
        if new_pos == self.pos() { None } else { Some(ScrollViewportTo(row, col)) }
    }
}
