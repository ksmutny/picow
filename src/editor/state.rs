use super::{content::{EditorContent, PosInDocument}, cursor::Cursor, viewport::Viewport};


pub struct EditorState {
    pub content: EditorContent,
    pub viewport: Viewport,
    pub cursor: Cursor,
}

impl EditorState {
    pub fn new(content: EditorContent, viewport: Viewport, cursor_pos: PosInDocument) -> Self {
        let (col, row) = cursor_pos;
        let cursor = Cursor::new(row, col);
        Self { content, viewport, cursor }
    }
}
