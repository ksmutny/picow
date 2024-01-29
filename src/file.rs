use std::{fs, io::Error};

pub fn read_file(path: &str) -> Result<(Vec<String>, String), Error> {
    let file_content = fs::read_to_string(path)?;
    let line_delimiter = detect_line_delimiter(&file_content);
    let lines = file_content.split(&line_delimiter).map(|s| s.to_string()).collect();

    Ok((lines, line_delimiter))
}

const CRFL: &str = "\r\n";
const LF: &str = "\n";
const CR: &str = "\r";

fn detect_line_delimiter(file_content: &str) -> String {
    let delimiter = if file_content.contains(CRFL) {
        CRFL
    } else if file_content.contains(LF) {
        LF
    } else if file_content.contains(CR) {
        CR
    } else {
        LF
    };

    delimiter.to_string()
}
