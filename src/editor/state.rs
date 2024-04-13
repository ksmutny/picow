use super::{content::EditorContent, cursor::Cursor};

pub type PosOnScreen = (u16, u16);
pub type ViewportDimensions = (u16, u16);
pub type PosInDocument = (usize, usize);


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

    pub fn pos(&self) -> PosInDocument { (self.left, self.top) }
    pub fn size(&self) -> ViewportDimensions { (self.width, self.height) }

    pub fn cursor_within(&self, (cursor_x, cursor_y): PosInDocument) -> bool {
        cursor_x >= self.left && cursor_x < self.left + self.width as usize &&
        cursor_y >= self.top && cursor_y < self.top + self.height as usize
    }

    pub fn to_relative(&self, (x, y): PosInDocument) -> PosOnScreen {
        ((x - self.left + 1) as u16, (y - self.top + 1) as u16)
    }

    pub fn to_absolute(&self, (x, y): PosOnScreen) -> PosInDocument {
        (x as usize + self.left - 1, y as usize + self.top - 1)
    }
}

impl EditorState {

    pub fn new(content: EditorContent, viewport: Viewport, cursor_pos: PosInDocument) -> Self {
        let (col, row) = cursor_pos;
        let cursor = Cursor::new(row, col);
        Self { content, viewport, cursor }
    }

    pub fn scroll_viewport(&mut self, x: usize, y: usize) {
        self.viewport.left = x;
        self.viewport.top = y;
    }

    pub fn resize_viewport(&mut self, width: u16, height: u16) {
        self.viewport.width = width;
        self.viewport.height = height - 1;
    }

    pub fn cursor_x(&self) -> usize { self.cursor.col }
    pub fn cursor_y(&self) -> usize { self.cursor.row }
}
