use super::{content::EditorContent, split::split, state::AbsPosition};


#[derive(PartialEq, Debug)]
pub struct Edit {
    pub op: EditOperation,
    pub from: AbsPosition,
    pub lines: Vec<String>,
}

#[derive(PartialEq, Debug)]
pub enum EditOperation {
    Insert,
    Delete
}

impl Edit {
    fn to(&self) -> AbsPosition {
        let (from_row, from_col) = self.from;
        let to_col_offset = if self.lines.len() == 1 { from_col } else { 0 };
        (from_row + self.lines.len() - 1, to_col_offset + self.lines[self.lines.len() - 1].len())
    }
}

fn op(op: EditOperation, from: AbsPosition, lines: Vec<String>) -> Edit {
    Edit { op, from, lines }
}

pub fn insert(cursor_pos: AbsPosition, to_insert: &str) -> Edit {
    let (lines, _) = split(to_insert);

    Edit {
        op: EditOperation::Insert,
        from: cursor_pos,
        lines
    }
}

pub fn delete(content: &EditorContent, from_pos @ (from_row, from_col): AbsPosition, (to_row, to_col): AbsPosition) -> Edit {
    let mut to_delete = Vec::new();
    let mut push = |line: &str| to_delete.push(line.to_owned());

    let lines = &content.lines;

    if from_row == to_row {
        push(&lines[from_row][from_col..=to_col]);
    } else {
        push(&lines[from_row][from_col..]);
        lines[from_row + 1..to_row].iter().for_each(|line| push(&line));
        push(&lines[to_row][..=to_col]);
    }

    Edit {
        op: EditOperation::Delete,
        from: from_pos,
        lines: to_delete
    }
}


fn process(content: &mut EditorContent, edit: &Edit) {
    let Edit { op, from: (from_row, from_col), lines } = edit;

    match op {
        EditOperation::Insert => {
            let mut to_insert = lines.clone();
            let (pre, post) = content.lines[*from_row].split_at(*from_col);
            to_insert[0] = pre.to_string() + &to_insert[0];
            to_insert[lines.len() - 1] = to_insert[lines.len() - 1].to_string() + post;

            content.lines.splice(from_row..=from_row, to_insert);
        },
        _ => ()
    }
}


#[cfg(test)]
mod test {
    macro_rules! s { ($x:expr) => ($x.to_string()); }
    macro_rules! vecs { ($($x:expr),*) => (vec![$(s!($x)),*]); }

    use crate::editor::{content::EditorContent, edit::{EditOperation::*, *}};

    #[test]
    fn to_single_line() {
        let edit = op(Insert, (12, 14), vecs!["line"]);
        assert_eq!(edit.to(), (12, 18));
    }

    #[test]
    fn to_multi_line() {
        let edit = op(Insert, (12, 14), vecs!["line 1", "line 23"]);
        assert_eq!(edit.to(), (13, 7));
    }

    #[test]
    fn insert_multi_line() {
        assert_eq!(
            insert((0, 5), "Hello\nWorld"),
            op(Insert, (0, 5), vecs!["Hello", "World"])
        )
    }

    #[test]
    fn delete_char() {
        let content = EditorContent::new(vecs![
            "Hello",
            "Amazing",
            "World"
        ], s!["\n"]);

        assert_eq!(
            delete(&content, (1, 0), (1, 0)),
            op(Delete, (1, 0), vecs!["A"])
        )
    }

    #[test]
    fn delete_line() {
        let content = EditorContent::new(vecs!["Hello"], s!["\n"]);

        assert_eq!(
            delete(&content, (0, 0), (0, 4)),
            op(Delete, (0, 0), vecs!["Hello"])
        )
    }

    #[test]
    fn delete_multi_line() {
        let content = EditorContent::new(vecs![
            "Hello",
            "Amazing",
            "World"
        ], s!["\n"]);

        assert_eq!(
            delete(&content, (0, 3), (2, 2)),
            op(Delete, (0, 3), vecs!["lo", "Amazing", "Wor"])
        )
    }

    #[test]
    fn delete_everything() {
        let content = EditorContent::new(vecs![
            "Hello",
            "World"
        ], s!["\n"]);

        assert_eq!(
            delete(&content, (0, 0), (1, 4)),
            op(Delete, (0, 0), vecs!["Hello", "World"])
        )
    }

    #[test]
    fn process_insert_single_line() {
        let mut content = EditorContent::new(vecs![
            "Hello",
            "World"
        ], s!["\n"]);

        let edit = op(Insert, (0, 4), vecs!["issim"]);

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

        let edit = op(Insert, (0, 4), vecs!["issimo", "Bell"]);

        process(&mut content, &edit);

        assert_eq!(content.lines, vecs![
            "Hellissimo",
            "Bello",
            "World"
        ])
    }
}
