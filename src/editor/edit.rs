use super::{content::{split, EditorContent, PosInDocument}, row::Row};
use EditOpKind::*;


#[derive(PartialEq, Debug)]
pub struct EditOp {
    pub kind: EditOpKind,
    pub from: PosInDocument,
    pub lines: Vec<Row>,
}

#[derive(PartialEq, Debug)]
pub enum EditOpKind {
    Insert,
    Delete
}

pub type EditCommand = Option<EditOp>;

impl EditOp {
    pub fn new(kind: EditOpKind, from: PosInDocument, lines: Vec<Row>) -> Self {
        Self { kind, from, lines }
    }

    pub fn insert(from: PosInDocument, str: &str) -> Self {
        Self::new(Insert, from, Self::lines_to_insert(str))
    }

    pub fn delete(content: &EditorContent, from: PosInDocument, to: PosInDocument) -> Self {
        Self::new(Delete, from, Self::lines_to_delete(&content, from, to))
    }

    pub fn inverse(&self) -> Self {
        Self::new(self.kind.inverse(), self.from, self.lines.clone())
    }

    fn lines_to_insert(str: &str) -> Vec<Row> {
        split(str).0
    }

    fn lines_to_delete(content: &EditorContent, (from_row, from_col): PosInDocument, (to_row, to_col): PosInDocument) -> Vec<Row> {
        let mut lines_to_delete = Vec::new();
        let mut push = |line: &str| lines_to_delete.push(Row::new(line));

        if from_row == to_row {
            push(&content.lines[from_row][from_col..to_col]);
        } else {
            push(&content.lines[from_row][from_col..]);
            content.lines[from_row + 1..to_row].iter().for_each(|line| push(&line[..]));
            push(&content.lines[to_row][..to_col]);
        };

        lines_to_delete
    }

    pub fn to(&self) -> PosInDocument {
        let (from_row, from_col) = self.from;

        let to_row = from_row + self.lines.len() - 1;

        let to_col_offset = if self.lines.len() == 1 { from_col } else { 0 };
        let to_col = to_col_offset + self.lines[self.lines.len() - 1].len();

        (to_row, to_col)
    }
}

impl EditOpKind {
    fn inverse(&self) -> EditOpKind {
        match self {
            Insert => Delete,
            Delete => Insert,
        }
    }
}


pub fn process(content: &mut EditorContent, edit_op: &EditOp) {
    let EditOp { kind, lines, .. } = edit_op;
    let (from_row, from_col) = edit_op.from;

    match kind {
        EditOpKind::Insert => {
            let mut to_insert = lines.clone();
            let (pre, post) = content.lines[from_row].split_at(from_col);
            to_insert[0] = pre.concat(&to_insert[0]);
            to_insert[lines.len() - 1] = to_insert[lines.len() - 1].concat(&post);

            content.lines.splice(from_row..=from_row, to_insert);
        },
        EditOpKind::Delete => {
            let (to_row, to_col) = edit_op.to();
            let pre = Row::new(&content.lines[from_row][..from_col]);
            let post = Row::new(&content.lines[to_row][to_col..]);
            let after_delete = pre.concat(&post);

            content.lines.splice(from_row..=to_row, vec![after_delete]);
        }
    }
}


#[cfg(test)]
mod test {
    use crate::{s, vecr};
    use crate::editor::{content::EditorContent, edit::*, row::Row};

    #[test]
    fn to_single_line() {
        let edit_op = EditOp::new(Insert, (12, 14), vecr!["line"]);
        assert_eq!(edit_op.to(), (12, 18));
    }

    #[test]
    fn to_multi_line() {
        let edit_op = EditOp::new(Insert, (12, 14), vecr!["line 1", "line 23"]);
        assert_eq!(edit_op.to(), (13, 7));
    }

    #[test]
    fn insert_op_multi_line() {
        assert_eq!(
            EditOp::insert((0, 5), "Hello\nWorld"),
            EditOp::new(Insert, (0, 5), vecr!["Hello", "World"])
        )
    }

    #[test]
    fn delete_op_char() {
        let content = EditorContent::new(vecr![
            "Hello",
            "Amazing",
            "World"
        ], s!["\n"]);

        assert_eq!(
            EditOp::delete(&content, (1, 0), (1, 1)),
            EditOp::new(Delete, (1, 0), vecr!["A"])
        )
    }

    #[test]
    fn delete_op_eol() {
        let content = EditorContent::new(vecr!["Hello", "A"], s!["\n"]);

        assert_eq!(
            EditOp::delete(&content, (0, 5), (1, 0)),
            EditOp::new(Delete, (0, 5), vecr!["", ""])
        )
    }

    #[test]
    fn delete_op_line() {
        let content = EditorContent::new(vecr!["Hello"], s!["\n"]);

        assert_eq!(
            EditOp::delete(&content, (0, 0), (0, 5)),
            EditOp::new(Delete, (0, 0), vecr!["Hello"])
        )
    }

    #[test]
    fn delete_op_multi_line() {
        let content = EditorContent::new(vecr![
            "Hello",
            "Amazing",
            "World"
        ], s!["\n"]);

        assert_eq!(
            EditOp::delete(&content, (0, 3), (2, 3)),
            EditOp::new(Delete, (0, 3), vecr!["lo", "Amazing", "Wor"])
        )
    }

    #[test]
    fn delete_op_everything() {
        let content = EditorContent::new(vecr![
            "Hello",
            "World"
        ], s!["\n"]);

        assert_eq!(
            EditOp::delete(&content, (0, 0), (1, 5)),
            EditOp::new(Delete, (0, 0), vecr!["Hello", "World"])
        )
    }

    #[test]
    fn process_insert_single_line() {
        let mut content = EditorContent::new(vecr![
            "Hello",
            "World"
        ], s!["\n"]);

        let edit_op = EditOp::new(Insert, (0, 4), vecr!["issim"]);

        process(&mut content, &edit_op);

        assert_eq!(content.lines, vecr![
            "Hellissimo",
            "World"
        ])
    }

    #[test]
    fn process_insert_multi_line() {
        let mut content = EditorContent::new(vecr![
            "Hello",
            "World"
        ], s!["\n"]);

        let edit_op = EditOp::new(Insert, (0, 4), vecr!["issimo", "Bell"]);

        process(&mut content, &edit_op);

        assert_eq!(content.lines, vecr![
            "Hellissimo",
            "Bello",
            "World"
        ])
    }

    #[test]
    fn process_delete_single_line() {
        let mut content = EditorContent::new(vecr!["Hello", "World"], s!("\n"));

        let edit_op = EditOp::delete(&content, (0, 3), (0, 5));

        process(&mut content, &edit_op);
        assert_eq!(content.lines, vecr!["Hel", "World"]);
    }

    #[test]
    fn delete_eol() {
        let mut content = EditorContent::new(vecr!["Hello", "World"], s!("\n"));
        let edit_op = EditOp::delete(&content, (0, 5), (1, 0));

        process(&mut content, &edit_op);
        assert_eq!(content.lines, vecr!["HelloWorld"]);
    }

    #[test]
    fn delete_multiple_lines() {
        let mut content = EditorContent::new(vecr!["Hello", "World", "How are you?"], s!("\n"));
        let edit_op = EditOp::delete(&content, (0, 2), (2, 8));

        process(&mut content, &edit_op);
        assert_eq!(content.lines, vecr!["Heyou?"]);
    }
}
