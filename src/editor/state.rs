use super::{content::{EditorContent, PosInDocument}, cursor::Cursor, viewport::Viewport};


pub struct EditorState {
    pub content: EditorContent,
    pub viewport: Viewport,
    pub cursor: Cursor,
    pub marked_for_refresh: bool,
}

impl EditorState {
    pub fn new(content: EditorContent, viewport: Viewport, cursor_pos: PosInDocument) -> Self {
        let (col, row) = cursor_pos;
        let cursor = Cursor::new(row, col);
        Self { content, viewport, cursor, marked_for_refresh: true }
    }

    pub fn mark_for_refresh(&mut self) {
        self.marked_for_refresh = true
    }
}
