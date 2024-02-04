use super::state::{AbsPosition, EditorState};

impl EditorState {
    pub fn insert_char(&mut self, (x, y): AbsPosition, c: char) {
        if c == '\n' {
            let (left, right) = (self.line(y)[..x].to_string(), self.line(y)[x..].to_string());
            self.lines[y] = left;
            self.lines.insert(y + 1, right);
        } else {
            self.lines[y] = format!("{}{}{}", &self.line(y)[..x], c, &self.line(y)[x..]);
        }
    }

    pub fn delete_char(&mut self, (x, y): AbsPosition) {
        if x < self.line(y).len() {
            self.lines[y] = format!("{}{}", &self.line(y)[..x], &self.line(y)[x + 1..]);
        } else if y < self.lines.len() - 1 {
            let next_line = self.lines.remove(y + 1);
            self.lines[y] = format!("{}{}", &self.line(y), next_line);
        }
    }

    fn line(&self, y: usize) -> &str {
        self.lines.get(y).unwrap()
    }
}
