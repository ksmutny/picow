use std::cmp::min;

use super::content::{EditorContent, PosInDocument};


#[derive(PartialEq, Debug)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
    furthest_col: Option<usize>
}

pub type NavigationCommand = Option<Cursor>;


impl Cursor {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col, furthest_col: None }
    }

    pub fn from((row, col): PosInDocument) -> Self {
        Self::new(row, col)
    }

    pub fn pos(&self) -> PosInDocument {
        (self.row, self.col)
    }

    pub fn is_at(&self, row: usize, col: usize) -> bool {
        self.row == row && self.col == col
    }

    pub fn move_to(&self, content: &EditorContent, pos: PosInDocument) -> NavigationCommand {
        self.move_to_from(content, pos, None)
    }

    fn move_to_from(&self, content: &EditorContent, pos: PosInDocument, furthest_col: Option<usize>) -> NavigationCommand {
        let new_cursor_pos = Self::within_text(content, pos);
        self.move_cmd(new_cursor_pos, furthest_col)
    }

    fn within_text(content: &EditorContent, (row, col): PosInDocument) -> PosInDocument {
        let new_row = min(row, content.last_line_row());
        let new_col = min(col, content.line_len(new_row));
        (new_row, new_col)
    }

    fn move_cmd(&self, (row, col): PosInDocument, furthest_col: Option<usize>) -> NavigationCommand {
        if self.is_at(row, col) { None } else { Some(Cursor { row, col, furthest_col } ) }
    }

    pub fn move_up(&self, content: &EditorContent, n: usize) -> NavigationCommand {
        self.move_vertical(content, |row| row - min(n, row))
    }

    pub fn move_down(&self, content: &EditorContent, n: usize) -> NavigationCommand {
        self.move_vertical(content, |row| row + min(n, content.last_line_row() - row))
    }

    fn move_vertical<F>(&self, content: &EditorContent, new: F) -> NavigationCommand
    where
        F: Fn(usize) -> usize,
    {
        let new_col = self.furthest_col.unwrap_or(self.col);
        self.move_to_from(content, (new(self.row), new_col), Some(new_col))
    }

    pub fn move_left(&self, content: &EditorContent) -> NavigationCommand {
        let move_to = match self.pos() {
            (0, 0) => (0, 0),
            (row, 0) => content.line_end(row - 1),
            (row, col) => (row, col - 1)
        };
        self.move_to(content, move_to)
    }

    pub fn move_right(&self, content: &EditorContent) -> NavigationCommand {
        let move_to = match self.pos() {
            (row, col) if col < content.line_len(row) => (row, col + 1),
            (row, _) if row < content.last_line_row() => (row + 1, 0),
            _ => self.pos()
        };
        self.move_to(content, move_to)
    }

    pub fn move_line_start(&self, content: &EditorContent) -> NavigationCommand {
        self.move_to(content, (self.row, 0))
    }

    pub fn move_line_end(&self, content: &EditorContent) -> NavigationCommand {
        self.move_to(content, content.line_end(self.row))
    }

    pub fn move_document_start(&self, content: &EditorContent) -> NavigationCommand {
        self.move_to(content, (0, 0))
    }

    pub fn move_document_end(&self, content: &EditorContent) -> NavigationCommand {
        self.move_to(content, content.last_line_end())
    }
}
