pub const CRLF: &str = "\r\n";
pub const LF: &str = "\n";
pub const CR: &str = "\r";


pub fn split(content: &str) -> (Vec<String>, String) {
    let delimiter = detect_line_delimiter(content);
    let lines = content.split(delimiter).map(String::from).collect();
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
    use crate::{s, vecs};
    use super::*;

    #[test]
    fn split_crlf() {
        let text = "Hello\r\nWorld\r\n";
        let (lines, delimiter) = split(text);
        assert_eq!(lines, vecs!["Hello", "World", ""]);
        assert_eq!(delimiter, s!(CRLF));
    }

    #[test]
    fn split_cr() {
        let text = "Hello\rWorld\r";
        let (lines, delimiter) = split(text);
        assert_eq!(lines, vecs!["Hello", "World", ""]);
        assert_eq!(delimiter, s!(CR));
    }

    #[test]
    fn split_lf() {
        let text = "Hello\nWorld\n";
        let (lines, delimiter) = split(text);
        assert_eq!(lines, vecs!["Hello", "World", ""]);
        assert_eq!(delimiter, s!(LF));
    }
}
