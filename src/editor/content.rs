pub const CRLF: &str = "\r\n";
pub const LF: &str = "\n";
pub const CR: &str = "\r";

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


pub fn split(content: &str) -> (Vec<String>, String) {
    let delimiter = detect_line_delimiter(content);
    let lines = content.split(delimiter).map(String::from).collect();
    (lines, delimiter.to_string())
}

fn detect_line_delimiter(file_content: &str) -> &str {
    if file_content.contains(CRLF) { CRLF }
    else if file_content.contains(LF) { LF }
    else if file_content.contains(CR) { CR }
    else { LF }
}


#[cfg(test)]
mod test {
    use crate::{s, vecs};
    use super::*;

    #[test]
    fn split_crlf() {
        let text = "Hello\r\nWorld\r\n";
        let (lines, delimiter) = split(text);
        assert_eq!(lines, vecs!["Hello", "World", ""]);
        assert_eq!(delimiter, s!(CRLF));
    }

    #[test]
    fn split_cr() {
        let text = "Hello\rWorld\r";
        let (lines, delimiter) = split(text);
        assert_eq!(lines, vecs!["Hello", "World", ""]);
        assert_eq!(delimiter, s!(CR));
    }

    #[test]
    fn split_lf() {
        let text = "Hello\nWorld\n";
        let (lines, delimiter) = split(text);
        assert_eq!(lines, vecs!["Hello", "World", ""]);
        assert_eq!(delimiter, s!(LF));
    }
}
