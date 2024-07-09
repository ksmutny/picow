use std::cmp::min;

use super::{content::EditorContent, pos::PosInDocument};


#[derive(PartialEq, Debug)]
pub struct Cursor {
    row: usize,
    col: usize,
    furthest_col: Option<usize>
}

type NavigationCommand = Option<Cursor>;


impl Cursor {
    pub fn from((row, col): PosInDocument) -> Self {
        Self { row, col, furthest_col: None }
    }

    pub fn pos(&self) -> PosInDocument {
        (self.row, self.col)
    }

    pub fn move_to(&self, content: &EditorContent, pos: PosInDocument) -> NavigationCommand {
        self.move_to_set_furthest_col(content, pos, None)
    }

    fn move_to_set_furthest_col(&self, content: &EditorContent, pos: PosInDocument, furthest_col: Option<usize>) -> NavigationCommand {
        let new_cursor_pos = Self::within_text(content, pos);
        self.move_cmd(new_cursor_pos, furthest_col)
    }

    fn within_text(content: &EditorContent, (row, col): PosInDocument) -> PosInDocument {
        let new_row = min(row, content.last_line_row());
        let new_col = min(col, content.line_len(new_row));
        (new_row, new_col)
    }

    fn move_cmd(&self, (row, col): PosInDocument, furthest_col: Option<usize>) -> NavigationCommand {
        if self.pos() == (row, col) { None } else { Some(Cursor { row, col, furthest_col } ) }
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
        let get_line = |idx: usize| { content.lines.get(idx).unwrap() };

        let current_line = get_line(self.row);
        let new_line = get_line(new(self.row));

        let mono_col = self.furthest_col.unwrap_or(current_line.mono_col_at(self.col));
        let new_col = new_line.char_idx_at(mono_col);

        self.move_to_set_furthest_col(content, (new(self.row), new_col), Some(mono_col))
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


#[cfg(test)]
mod test {
    use crate::editor::{content::EditorContent, cursor::Cursor, row::Row};

    fn curs(row: usize, col: usize, furthest_col: Option<usize>) -> Cursor {
        Cursor { row, col, furthest_col }
    }

    fn content(rows: Vec<&str>) -> EditorContent {
        EditorContent::new(rows.iter().map(|&s| Row::new(s)).collect(), "\n".to_string())
    }

    #[test]
    fn unicode_down_2_to_1() {
        let cursor = curs(0, 2, None);
        let content = content(vec![
            "he",
            "😎😎"
        ]);
        assert_eq!(cursor.move_down(&content, 1), Some(curs(1, 1, Some(2))));
    }

    #[test]
    fn unicode_down_5_to_2() {
        let cursor = curs(0, 5, None);
        let content = content(vec![
            "hello",
            "😎😎"
        ]);
        assert_eq!(cursor.move_down(&content, 1), Some(curs(1, 2, Some(5))));
    }

    #[test]
    fn unicode_down_2_to_5() {
        let cursor = curs(1, 2, Some(5));
        let content = content(vec![
            "hello",
            "😎😎",
            "world!"
        ]);
        assert_eq!(cursor.move_down(&content, 1), Some(curs(2, 5, Some(5))));
    }
}
