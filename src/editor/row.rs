use std::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};

use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Row {
    bytes: String,
    byte_idx: Vec<usize>,
}

impl Row {
    pub fn new(str: &str) -> Self {
        let byte_idx = Self::idx(str);

        Self {
            bytes: str.to_string(),
            byte_idx,
        }
    }

    fn idx(str: &str) -> Vec<usize> {
        let mut byte_idx = Vec::new();

        for (idx, _) in str.grapheme_indices(true) {
            byte_idx.push(idx);
        }

        byte_idx
    }

    fn byte_idx(&self, char_idx: usize) -> usize {
        if char_idx < self.byte_idx.len() {
            return self.byte_idx[char_idx]
        }
        return self.bytes.len();
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
}
