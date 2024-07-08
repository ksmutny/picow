use std::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Row {
    bytes: String,
    byte_idx: Vec<usize>,
    char_width: Vec<usize>,
}

impl Row {
    pub fn new(str: &str) -> Self {
        let (byte_idx, char_width) = Self::idx(str);

        Self {
            bytes: str.to_string(),
            byte_idx,
            char_width,
        }
    }

    fn idx(str: &str) -> (Vec<usize>, Vec<usize>) {
        let mut byte_idx = Vec::new();
        let mut char_width = Vec::new();

        let mut byte = 0;
        let mut width = 0;

        for grapheme in UnicodeSegmentation::graphemes(str, true) {
            byte_idx.push(byte);
            char_width.push(width);

            byte += grapheme.len();
            width += UnicodeWidthStr::width(grapheme);
        }
        char_width.push(width);

        (byte_idx, char_width)
    }

    fn byte_idx(&self, char_idx: usize) -> usize {
        if char_idx < self.byte_idx.len() {
            return self.byte_idx[char_idx]
        }
        return self.bytes.len();
    }

    pub fn mono_col_at(&self, char_idx: usize) -> usize {
        if char_idx < self.char_width.len() {
            return self.char_width[char_idx]
        }
        return self.bytes.len();
    }

    pub fn char_idx_at(&self, mono_col: usize) -> usize {
        for (idx, &col) in self.char_width.iter().enumerate() {
            if col >= mono_col {
                return idx
            }
        }
        return self.char_width.len();
    }

    pub fn len(&self) -> usize {
        self.byte_idx.len()
    }

    pub fn split_at(&self, at: usize) -> (Row, Row) {
        let (left, right) = self.bytes.split_at(self.byte_idx(at));
        (Row::new(left), Row::new(right))
    }

    pub fn concat(&self, other: &Row) -> Row {
        let mut bytes = self.bytes.clone();
        bytes.push_str(&other.bytes);
        Row::new(&bytes)
    }
}

pub trait RowVecExt {
    fn join(&self, sep: &str) -> String;
}

impl RowVecExt for Vec<Row> {
    fn join(&self, sep: &str) -> String {
        self.iter()
            .map(|row| row.bytes.as_str())
            .collect::<Vec<&str>>()
            .join(sep)
    }
}

impl Index<Range<usize>> for Row {
    type Output = str;

    fn index(&self, range: Range<usize>) -> &Self::Output {
        let start = self.byte_idx(range.start);
        let end = self.byte_idx(range.end);
        &self.bytes[start..end]
    }
}

impl Index<RangeFrom<usize>> for Row {
    type Output = str;

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        let start = self.byte_idx(range.start);
        &self.bytes[start..]
    }
}

impl Index<RangeTo<usize>> for Row {
    type Output = str;

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        let end = self.byte_idx(range.end);
        &self.bytes[..end]
    }
}

impl Index<RangeFull> for Row {
    type Output = str;

    fn index(&self, _: RangeFull) -> &Self::Output {
        &self.bytes[..]
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn row(str: &str) -> Row {
        Row::new(str)
    }

    macro_rules! test {
        ($name:ident: $left:expr => $right:expr) => {
            #[test]
            fn $name() {
                assert_eq!($left, $right)
            }
        };
    }

    test! { len: row("Příliš").len() => 6 }
    test! { split_at: row("Příliš").split_at(3) => (row("Pří"), row("liš")) }
    test! { concat: row("žlu").concat(&row("ťoučký")) => row("žluťoučký") }
    test! { range: &row("žluťoučký")[1..5] => "luťo" }
    test! { range_from: &row("žluťoučký")[2..] => "uťoučký" }
    test! { range_to: &row("žluťoučký")[..7] => "žluťouč" }
    test! { range_full: &row("žluťoučký")[..] => "žluťoučký" }

    test! { char_width_start: row("Hello").mono_col_at(0) => 0 }
    test! { char_width_ascii: row("Hello").mono_col_at(3) => 3 }
    test! { char_width_accent: row("kůň").mono_col_at(2) => 2 }

    test! { char_width_emoji_0: row("I💖U").mono_col_at(0) => 0 }
    test! { char_width_emoji_1: row("I💖U").mono_col_at(1) => 1 }
    test! { char_width_emoji_2: row("I💖U").mono_col_at(2) => 3 }
    test! { char_width_emoji_3: row("I💖U").mono_col_at(3) => 4 }

    test! { chat_idx_at_0: row("I💖kůň").char_idx_at(0) => 0 }
    test! { chat_idx_at_1: row("I💖kůň").char_idx_at(1) => 1 }
    test! { chat_idx_at_3: row("I💖kůň").char_idx_at(3) => 2 }
    test! { chat_idx_at_5: row("I💖kůň").char_idx_at(5) => 4 }

    #[test]
    fn join() {
        let rows = vec![row("žlu"), row("ťoučký")];
        assert_eq!(rows.join(""), "žluťoučký")
    }
}
