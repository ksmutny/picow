pub type PosInDocument = (usize, usize);

pub trait PosInDocumentExt {
    fn is_before(&self, other: &PosInDocument) -> bool;
}

impl PosInDocumentExt for PosInDocument {
    fn is_before(&self, other: &PosInDocument) -> bool {
        self.0 < other.0 || (self.0 == other.0 && self.1 < other.1)
    }
}
