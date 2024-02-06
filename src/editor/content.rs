use super::state::AbsPosition;

pub const CRLF: &str = "\r\n";
pub const LF: &str = "\n";
pub const CR: &str = "\r";

pub struct EditorContent {
    pub lines: Vec<String>,
    pub delimiter: String,
}

impl EditorContent {
    pub fn new(lines: Vec<String>, delimiter: String) -> Self {
        Self { lines, delimiter }
    }

    pub fn parse(content: &str) -> Self {
        let (lines, delimiter) = Self::split(content);
        Self::new(lines, delimiter)
    }

    fn split(content: &str) -> (Vec<String>, String) {
        let delimiter = Self::detect_line_delimiter(content);
        let lines = content.split(delimiter).map(String::from).collect();
        (lines, delimiter.to_string())
    }

    fn detect_line_delimiter(file_content: &str) -> &str {
        if file_content.contains(CRLF) { CRLF }
        else if file_content.contains(LF) { LF }
        else if file_content.contains(CR) { CR }
        else { LF }
    }

    pub fn insert(&mut self, (row, col): AbsPosition, str: &str) -> AbsPosition {
        let mut to_insert = self.lines[row].clone();
        to_insert.insert_str(col, str);

        let (inserted_lines, _) = Self::split(&to_insert);

        let cursor_pos = (
            row + inserted_lines.len() - 1,
            if inserted_lines.len() == 1 {
                col + str.len()
            } else {
                inserted_lines[inserted_lines.len() - 1].len() - (self.lines[row].len() - col)
            }
        );

        self.lines.splice(row..=row, inserted_lines);

        cursor_pos
    }

    pub fn delete(&mut self, (from_row, from_col): AbsPosition, (to_row, to_col): AbsPosition) {
        let after_delete = format!("{}{}", &self.lines[from_row][..from_col], &self.lines[to_row][to_col..]);
        self.lines.splice(from_row..=to_row, vec![after_delete]);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! s { ($x:expr) => ($x.to_string()); }

    macro_rules! vecs { ($($x:expr),*) => (vec![$(s!($x)),*]); }


    #[test]
    fn parse() {
        let text = "Hello\r\nWorld\r\n";
        let content = EditorContent::parse(text);
        assert_eq!(content.lines, vecs!["Hello", "World", ""]);
        assert_eq!(content.delimiter, "\r\n".to_string());
    }


    #[test]
    fn insert_single_line() {
        let mut content = EditorContent::new(vecs!["Hello", "World"], s!("\n"));
        let cursor_pos = content.insert((0, 4), "issim");
        assert_eq!(cursor_pos, (0, 9));
        assert_eq!(content.lines, vecs!["Hellissimo", "World"]);
    }

    #[test]
    fn insert_single_line_eol() {
        let mut content = EditorContent::new(vecs!["Hello", "World"], s!("\n"));
        let cursor_pos = content.insert((0, 5), ",");
        assert_eq!(cursor_pos, (0, 6));
        assert_eq!(content.lines, vecs!["Hello,", "World"]);
    }

    #[test]
    fn insert_single_line_eof() {
        let mut content = EditorContent::new(vecs!["Hello", "World"], s!("\n"));
        let cursor_pos = content.insert((1, 5), "!");
        assert_eq!(cursor_pos, (1, 6));
        assert_eq!(content.lines, vecs!["Hello", "World!"]);
    }

    #[test]
    fn insert_multi_line() {
        let mut content = EditorContent::new(vecs!["Hello", "World"], s!("\n"));
        let cursor_pos = content.insert((0, 3), "lissimo,\r\nWonderful\r\nBig ");
        assert_eq!(cursor_pos, (2, 4));
        assert_eq!(content.lines, vecs!["Hellissimo,", "Wonderful", "Big lo", "World"]);
    }

    #[test]
    fn insert_multi_line_eol() {
        let mut content = EditorContent::new(vecs!["Hello", "World"], s!("\n"));
        let cursor_pos = content.insert((0, 5), ",\r\nWonderful\r\nBig");
        assert_eq!(cursor_pos, (2, 3));
        assert_eq!(content.lines, vecs!["Hello,", "Wonderful", "Big", "World"]);
    }

    #[test]
    fn insert_multi_line_eof() {
        let mut content = EditorContent::new(vecs!["Hello", "World"], s!("\n"));
        let cursor_pos = content.insert((1, 5), "!\nHow are you?");
        assert_eq!(cursor_pos, (2, 12));
        assert_eq!(content.lines, vecs!["Hello", "World!", "How are you?"]);
    }


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
