use super::{content::{EditorContent, PosInDocument}, split::split};
use EditOperation::*;


#[derive(PartialEq, Debug)]
pub struct Edit {
    pub op: EditOperation,
    pub from: PosInDocument,
    pub lines: Vec<String>,
}

#[derive(PartialEq, Debug)]
pub enum EditOperation {
    Insert,
    Delete
}

impl Edit {
    pub fn new(op: EditOperation, from: PosInDocument, lines: Vec<String>) -> Self {
        Self { op, from, lines }
    }

    pub fn to(&self) -> PosInDocument {
        let (from_row, from_col) = self.from;
        let to_col_offset = if self.lines.len() == 1 { from_col } else { 0 };
        (from_row + self.lines.len() - 1, to_col_offset + self.lines[self.lines.len() - 1].len())
    }
}

pub fn insert_op(cursor_pos: PosInDocument, to_insert: &str) -> Edit {
    let (lines, _) = split(to_insert);

    Edit::new(Insert, cursor_pos, lines)
}

pub fn delete_op(content: &EditorContent, from_pos @ (from_row, from_col): PosInDocument, (to_row, to_col): PosInDocument) -> Edit {
    let mut to_delete = Vec::new();
    let mut push = |line: &str| to_delete.push(line.to_owned());

    let lines = &content.lines;

    if from_row == to_row {
        push(&lines[from_row][from_col..to_col]);
    } else {
        push(&lines[from_row][from_col..]);
        lines[from_row + 1..to_row].iter().for_each(|line| push(&line));
        push(&lines[to_row][..to_col]);
    }

    Edit::new(Delete, from_pos, to_delete)
}

pub fn inverse_op(edit: &Edit) -> Edit {
    let op = match edit.op {
        Insert => Delete,
        Delete => Insert,
    };

    Edit::new(op, edit.from, edit.lines.clone())
}


pub fn process(content: &mut EditorContent, edit: &Edit) {
    let Edit { op, lines, .. } = edit;
    let (from_row, from_col) = edit.from;

    match op {
        EditOperation::Insert => {
            let mut to_insert = lines.clone();
            let (pre, post) = content.lines[from_row].split_at(from_col);
            to_insert[0] = pre.to_string() + &to_insert[0];
            to_insert[lines.len() - 1] = to_insert[lines.len() - 1].to_string() + post;

            content.lines.splice(from_row..=from_row, to_insert);
        },
        EditOperation::Delete => {
            let (to_row, to_col) = edit.to();
            let after_delete = format!("{}{}", &content.lines[from_row][..from_col], &content.lines[to_row][to_col..]);

            content.lines.splice(from_row..=to_row, vec![after_delete]);
        }
    }
}


#[cfg(test)]
mod test {
    use crate::{s, vecs};
    use crate::editor::{content::EditorContent, edit::{EditOperation::*, *}};

    #[test]
    fn to_single_line() {
        let edit = Edit::new(Insert, (12, 14), vecs!["line"]);
        assert_eq!(edit.to(), (12, 18));
    }

    #[test]
    fn to_multi_line() {
        let edit = Edit::new(Insert, (12, 14), vecs!["line 1", "line 23"]);
        assert_eq!(edit.to(), (13, 7));
    }

    #[test]
    fn insert_op_multi_line() {
        assert_eq!(
            insert_op((0, 5), "Hello\nWorld"),
            Edit::new(Insert, (0, 5), vecs!["Hello", "World"])
        )
    }

    #[test]
    fn delete_op_char() {
        let content = EditorContent::new(vecs![
            "Hello",
            "Amazing",
            "World"
        ], s!["\n"]);

        assert_eq!(
            delete_op(&content, (1, 0), (1, 1)),
            Edit::new(Delete, (1, 0), vecs!["A"])
        )
    }

    #[test]
    fn delete_op_eol() {
        let content = EditorContent::new(vecs!["Hello", "A"], s!["\n"]);

        assert_eq!(
            delete_op(&content, (0, 5), (1, 0)),
            Edit::new(Delete, (0, 5), vecs!["", ""])
        )
    }

    #[test]
    fn delete_op_line() {
        let content = EditorContent::new(vecs!["Hello"], s!["\n"]);

        assert_eq!(
            delete_op(&content, (0, 0), (0, 5)),
            Edit::new(Delete, (0, 0), vecs!["Hello"])
        )
    }

    #[test]
    fn delete_op_multi_line() {
        let content = EditorContent::new(vecs![
            "Hello",
            "Amazing",
            "World"
        ], s!["\n"]);

        assert_eq!(
            delete_op(&content, (0, 3), (2, 3)),
            Edit::new(Delete, (0, 3), vecs!["lo", "Amazing", "Wor"])
        )
    }

    #[test]
    fn delete_op_everything() {
        let content = EditorContent::new(vecs![
            "Hello",
            "World"
        ], s!["\n"]);

        assert_eq!(
            delete_op(&content, (0, 0), (1, 5)),
            Edit::new(Delete, (0, 0), vecs!["Hello", "World"])
        )
    }

    #[test]
    fn process_insert_single_line() {
        let mut content = EditorContent::new(vecs![
            "Hello",
            "World"
        ], s!["\n"]);

        let edit = Edit::new(Insert, (0, 4), vecs!["issim"]);

        process(&mut content, &edit);

        assert_eq!(content.lines, vecs![
            "Hellissimo",
            "World"
        ])
    }

    #[test]
    fn process_insert_multi_line() {
        let mut content = EditorContent::new(vecs![
            "Hello",
            "World"
        ], s!["\n"]);

        let edit = Edit::new(Insert, (0, 4), vecs!["issimo", "Bell"]);

        process(&mut content, &edit);

        assert_eq!(content.lines, vecs![
            "Hellissimo",
            "Bello",
            "World"
        ])
    }

    #[test]
    fn process_delete_single_line() {
        let mut content = EditorContent::new(vecs!["Hello", "World"], s!("\n"));

        let edit = delete_op(&content, (0, 3), (0, 5));

        process(&mut content, &edit);
        assert_eq!(content.lines, vecs!["Hel", "World"]);
    }

    #[test]
    fn delete_eol() {
        let mut content = EditorContent::new(vecs!["Hello", "World"], s!("\n"));
        let edit = delete_op(&content, (0, 5), (1, 0));

        process(&mut content, &edit);
        assert_eq!(content.lines, vecs!["HelloWorld"]);
    }

    #[test]
    fn delete_multiple_lines() {
        let mut content = EditorContent::new(vecs!["Hello", "World", "How are you?"], s!("\n"));
        let edit = delete_op(&content, (0, 2), (2, 8));

        process(&mut content, &edit);
        assert_eq!(content.lines, vecs!["Heyou?"]);
    }
}
