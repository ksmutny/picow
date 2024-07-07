use super::row::Row;

pub const CRLF: &str = "\r\n";
pub const LF: &str = "\n";
pub const CR: &str = "\r";

pub type PosInDocument = (usize, usize);

pub struct EditorContent {
    pub lines: Vec<Row>,
    pub delimiter: String,
}

impl EditorContent {
    pub fn new(lines: Vec<Row>, delimiter: String) -> Self {
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


pub fn split(content: &str) -> (Vec<Row>, String) {
    let delimiter = detect_line_delimiter(content);
    let lines = content.split(delimiter).map(Row::new).collect();
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
    use crate::vecr;
    use super::{*, super::row::Row};

    macro_rules! test_split {
        ($name:ident, $text:expr, $lines:expr, $delimiter:expr) => {
            #[test]
            fn $name() {
                let (lines, delimiter) = split($text);
                assert_eq!(lines, $lines);
                assert_eq!(delimiter, $delimiter);
            }
        };
    }

    test_split! { split_empty, "", vecr![""], LF }
    test_split! { split_crlf, "Hello\r\nWorld\r\n", vecr!["Hello", "World", ""], CRLF }
    test_split! { split_cr, "Hello\rWonderful\rWorld", vecr!["Hello", "Wonderful", "World"], CR }
    test_split! { split_lf, "Hello\nWorld\n", vecr!["Hello", "World", ""], LF }
}
