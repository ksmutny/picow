use std::{fs, io::Error};

pub fn read_file(path: &str) -> Result<(Vec<String>, String), Error> {
    let file_content = fs::read_to_string(path)?;
    let line_delimiter = detect_line_delimiter(&file_content);
    let lines = file_content.split(&line_delimiter).map(|s| s.to_string()).collect();

    Ok((lines, line_delimiter))
}

pub const CRLF: &str = "\r\n";
pub const LF: &str = "\n";
pub const CR: &str = "\r";

fn detect_line_delimiter(file_content: &str) -> String {
    let delimiter = if file_content.contains(CRLF) {
        CRLF
    } else if file_content.contains(LF) {
        LF
    } else if file_content.contains(CR) {
        CR
    } else {
        LF
    };

    delimiter.to_string()
}
