use std::cmp::min;

use super::state::{AbsPosition, EditorState, Viewport};


#[derive(PartialEq, Debug)]
pub struct ScrollViewportTo(pub usize, pub usize);
pub type ScrollCommand = Option<ScrollViewportTo>;

impl EditorState {

    pub fn scroll_into_view(&self, (x, y): AbsPosition) -> ScrollCommand {
        let Viewport { left, top, width, height } = self.viewport;

        let scroll_into = |cursor_pos, viewport_start, viewport_size| {
            if cursor_pos < viewport_start { cursor_pos }
            else if cursor_pos >= viewport_start + viewport_size as usize { cursor_pos - viewport_size as usize + 1 }
            else { viewport_start }
        };

        self.scroll_cmd((scroll_into(x, left, width), scroll_into(y, top, height)))
    }

    pub fn scroll_to(&self, (scroll_left, scroll_top): AbsPosition) -> ScrollCommand {
        let new_scroll_top = min(scroll_top, self.content.last_line_row());
        self.scroll_cmd((scroll_left, new_scroll_top))
    }

    fn scroll_cmd(&self, new_pos @ (x, y): AbsPosition) -> ScrollCommand {
        if new_pos == self.viewport.pos() { None } else { Some(ScrollViewportTo(x, y)) }
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
        self.scroll_to((left, new(top)))
    }
}
