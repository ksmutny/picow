use super::split::split;

pub type PosInDocument = (usize, usize);

pub struct EditorContent {
    pub lines: Vec<String>,
    pub delimiter: String,
}

impl EditorContent {
    pub fn new(lines: Vec<String>, delimiter: String) -> Self {
        Self { lines, delimiter }
    }

    pub fn parse(content: &str) -> Self {
        let (lines, delimiter) = split(content);
        Self::new(lines, delimiter)
    }

    pub fn line_end(&self, row: usize) -> PosInDocument {
        (row, self.line_len(row))
    }

    pub fn line_len(&self, row: usize) -> usize {
        self.lines[row].len()
    }

    pub fn last_line_end(&self) -> PosInDocument {
        self.line_end(self.last_line_row())
    }

    pub fn last_line_row(&self) -> usize {
        self.lines.len() - 1
    }
}
