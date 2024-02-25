#[derive(PartialEq, Debug)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
    // TODO make private after moving navigation methods to Cursor
    pub moved_vertically: bool,
    pub last_col: usize
}


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
}
