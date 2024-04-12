use std::cmp::min;

use super::{content::EditorContent, state::AbsPosition};


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

    // TODO temporary method
    pub fn pos(&self) -> (usize, usize) {
        (self.col, self.row)
    }

    pub fn is_at(&self, row: usize, col: usize) -> bool {
        self.row == row && self.col == col
    }

    fn move_to(&self, content: &EditorContent, pos: AbsPosition) -> NavigationCommand {
        self.move_to_from(content, pos, None)
    }

    fn move_to_from(&self, content: &EditorContent, (col, row): AbsPosition, furthest_col: Option<usize>) -> NavigationCommand {
        let new_cursor_pos = Self::within_text(content, (col, row));
        self.move_cmd(new_cursor_pos, furthest_col)
    }

    fn within_text(content: &EditorContent, (col, row): AbsPosition) -> AbsPosition {
        let new_row = min(row, content.last_line_row());
        let new_col = min(col, content.line_len(new_row));
        (new_col, new_row)
    }

    fn move_cmd(&self, (col, row): AbsPosition, furthest_col: Option<usize>) -> NavigationCommand {
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
        self.move_to_from(content, (new_col, new(self.row)), Some(new_col))
    }

    pub fn move_left(&self, content: &EditorContent) -> NavigationCommand {
        let move_to = match self.pos() {
            (0, 0) => (0, 0),
            (0, row) => content.line_end(row - 1),
            (col, row) => (col - 1, row)
        };
        self.move_to(content, move_to)
    }

    pub fn move_right(&self, content: &EditorContent) -> NavigationCommand {
        let move_to = match self.pos() {
            (col, row) if col < content.line_len(row) => (col + 1, row),
            (_, row) if row < content.last_line_row() => (0, row + 1),
            _ => self.pos()
        };
        self.move_to(content, move_to)
    }

    pub fn move_line_start(&self, content: &EditorContent) -> NavigationCommand {
        self.move_to(content, (0, self.row))
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

    pub fn click(&self, content: &EditorContent, new_pos: AbsPosition) -> NavigationCommand {
        self.move_to(content, new_pos)
    }
}
