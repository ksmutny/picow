use std::cmp::min;

use super::{content::EditorContent, state::AbsPosition};


#[derive(PartialEq, Debug)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
    moved_vertically: bool,
    last_col: usize
}

pub type NavigationCommand = Option<Cursor>;


impl Cursor {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col, moved_vertically: false, last_col: 0 }
    }

    // TODO temporary method
    pub fn pos(&self) -> (usize, usize) {
        (self.col, self.row)
    }

    pub fn is_at(&self, row: usize, col: usize) -> bool {
        self.row == row && self.col == col
    }

    fn move_to(&self, content: &EditorContent, (x, y): AbsPosition, is_vertical: bool) -> NavigationCommand {
        let (new_x, new_last_x) = match (is_vertical, self.moved_vertically) {
            (true, true) => (self.last_col, self.last_col),
            (true, false) => (self.col, self.last_col),
            (false, _) => (x, x)
        };

        let new_cursor_pos = self.within_text(content, (new_x, y));
        self.move_cmd(new_cursor_pos, is_vertical, new_last_x)
    }

    fn within_text(&self, content: &EditorContent, (x, y): AbsPosition) -> AbsPosition {
        let new_y = min(y, content.last_line_y());
        let new_x = min(x, content.line_len(new_y));
        (new_x, new_y)
    }

    fn move_cmd(&self, new_pos @ (x, y): AbsPosition, is_vertical: bool, last_x: usize) -> NavigationCommand {
        if new_pos == self.pos() { None } else { Some(Cursor { row: y, col: x, moved_vertically: is_vertical, last_col: last_x } ) }
    }

    pub fn move_up(&self, content: &EditorContent, n: usize) -> NavigationCommand {
        self.move_vertical(content, |y| y - min(n, y))
    }

    pub fn move_down(&self, content: &EditorContent, n: usize) -> NavigationCommand {
        self.move_vertical(content, |y| y + min(n, content.last_line_y() - y))
    }

    fn move_vertical<F>(&self, content: &EditorContent, new: F) -> NavigationCommand
    where
        F: Fn(usize) -> usize,
    {
        let (x, y) = self.pos();
        self.move_to(content, (x, new(y)), true)
    }

    pub fn move_left(&self, content: &EditorContent) -> NavigationCommand {
        let move_to = match self.pos() {
            (0, 0) => (0, 0),
            (0, y) => content.line_end(y - 1),
            (x, y) => (x - 1, y)
        };
        self.move_to(content, move_to, false)
    }

    pub fn move_right(&self, content: &EditorContent) -> NavigationCommand {
        let move_to = match self.pos() {
            (x, y) if x < content.line_len(y) => (x + 1, y),
            (_, y) if y < content.last_line_y() => (0, y + 1),
            _ => self.pos()
        };
        self.move_to(content, move_to, false)
    }

    pub fn move_line_start(&self, content: &EditorContent) -> NavigationCommand {
        self.move_to(content, (0, self.row), false)
    }

    pub fn move_line_end(&self, content: &EditorContent) -> NavigationCommand {
        self.move_to(content, content.line_end(self.row), false)
    }

    pub fn move_document_start(&self, content: &EditorContent) -> NavigationCommand {
        self.move_to(content, (0, 0), false)
    }

    pub fn move_document_end(&self, content: &EditorContent) -> NavigationCommand {
        self.move_to(content, content.last_line_end(), false)
    }

    pub fn click(&self, content: &EditorContent, new_pos: AbsPosition) -> NavigationCommand {
        self.move_to(content, new_pos, false)
    }
}
