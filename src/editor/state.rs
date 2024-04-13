use super::{content::{EditorContent, PosInDocument}, cursor::Cursor};

pub type PosOnScreen = (u16, u16);
pub type ViewportDimensions = (u16, u16);


pub struct EditorState {
    pub content: EditorContent,
    pub viewport: Viewport,
    pub cursor: Cursor,
}

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
}

impl EditorState {

    pub fn new(content: EditorContent, viewport: Viewport, cursor_pos: PosInDocument) -> Self {
        let (col, row) = cursor_pos;
        let cursor = Cursor::new(row, col);
        Self { content, viewport, cursor }
    }
}
