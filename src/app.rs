use std::{fs, io};

use crate::{editor::{content::EditorContent, Editor}, terminal};


pub fn start(file_name: &str) -> io::Result<()> {
    let editor_content = read_content(file_name)?;

    let console_mode = init()?;
    Editor::new(editor_content).run()?;
    close(console_mode)
}

fn read_content(file_name: &str) -> io::Result<EditorContent> {
    let file_content = fs::read_to_string(file_name)?;
    let editor_content = EditorContent::parse(&file_content);
    Ok(editor_content)
}

fn init() -> io::Result<u32> {
    let console_mode = terminal::init()?;
    Ok(console_mode)
}

fn close(console_mode: u32) -> io::Result<()> {
    terminal::restore_console_mode(console_mode)
}
