use super::{split::split, state::AbsPosition};

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

    pub fn line_end(&self, row: usize) -> AbsPosition {
        (self.line_len(row), row)
    }

    pub fn line_len(&self, row: usize) -> usize {
        self.lines[row].len()
    }

    pub fn last_line_end(&self) -> AbsPosition {
        self.line_end(self.last_line_y())
    }

    pub fn last_line_y(&self) -> usize {
        self.lines.len() - 1
    }

    pub fn delete(&mut self, (from_row, from_col): AbsPosition, (to_row, to_col): AbsPosition) {
        let after_delete = format!("{}{}", &self.lines[from_row][..from_col], &self.lines[to_row][to_col..]);
        self.lines.splice(from_row..=to_row, vec![after_delete]);
    }
}


#[cfg(test)]
mod tests {
    use crate::{s, vecs};
    use super::*;

    #[test]
    fn delete_single_line() {
        let mut content = EditorContent::new(vecs!["Hello", "World"], s!("\n"));
        content.delete((0, 3), (0, 5));
        assert_eq!(content.lines, vecs!["Hel", "World"]);
    }

    #[test]
    fn delete_eol() {
        let mut content = EditorContent::new(vecs!["Hello", "World"], s!("\n"));
        content.delete((0, 5), (1, 0));
        assert_eq!(content.lines, vecs!["HelloWorld"]);
    }

    #[test]
    fn delete_multi_line() {
        let mut content = EditorContent::new(vecs!["Hello", "World", "How are you?"], s!("\n"));
        content.delete((0, 2), (2, 8));
        assert_eq!(content.lines, vecs!["Heyou?"]);
    }
}
