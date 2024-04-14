use std::cmp::min;

use super::content::{EditorContent, PosInDocument};

pub type PosOnScreen = (u16, u16);
pub type ViewportDimensions = (u16, u16);

pub type ScrollCommand = Option<PosInDocument>;


pub struct Viewport {
    pub left: usize,
    pub top: usize,
    pub width: u16,
    pub height: u16,
}

impl Viewport {
    pub fn new(left: usize, top: usize, width: u16, height: u16) -> Self {
        Self { left, top, width, height }
    }

    pub fn pos(&self) -> PosInDocument { (self.top, self.left) }
    pub fn size(&self) -> ViewportDimensions { (self.width, self.height) }

    pub fn cursor_within(&self, (cursor_x, cursor_y): PosInDocument) -> bool {
        cursor_x >= self.left && cursor_x < self.left + self.width as usize &&
        cursor_y >= self.top && cursor_y < self.top + self.height as usize
    }

    pub fn scroll(&mut self, top: usize, left: usize) {
        self.left = left;
        self.top = top;
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height - 1;
    }

    pub fn to_relative(&self, (x, y): PosInDocument) -> PosOnScreen {
        ((x - self.left + 1) as u16, (y - self.top + 1) as u16)
    }

    pub fn to_absolute(&self, (row, col): PosOnScreen) -> PosInDocument {
        (row as usize + self.top - 1, col as usize + self.left - 1)
    }

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
        if new_pos == self.pos() { None } else { Some((row, col)) }
    }
}
