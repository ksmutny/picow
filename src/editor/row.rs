use std::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Row {
    bytes: String
}

impl Row {
    pub fn new(str: &str) -> Self {
        Self { bytes: str.to_string() }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn split_at(&self, at: usize) -> (Row, Row) {
        let (left, right) = self.bytes.split_at(at);
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
        &self.bytes[range.start..range.end]
    }
}

impl Index<RangeFrom<usize>> for Row {
    type Output = str;

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.bytes[range.start..]
    }
}

impl Index<RangeTo<usize>> for Row {
    type Output = str;

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.bytes[..range.end]
    }
}

impl Index<RangeFull> for Row {
    type Output = str;

    fn index(&self, _: RangeFull) -> &Self::Output {
        &self.bytes[..]
    }
}
