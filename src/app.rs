use std::{fs, io};

use crate::{
    editor::{content::EditorContent, Editor},
    terminal::{self, buffer::CommandExecutor, commands::Command::*}
};


pub fn start(file_name: &str) -> io::Result<()> {
    let editor_content = read_content(file_name)?;

    let console_mode = init(file_name)?;
    Editor::new(editor_content).run()?;
    close(console_mode)
}

fn read_content(file_name: &str) -> io::Result<EditorContent> {
    let file_content = fs::read_to_string(file_name)?;
    let editor_content = EditorContent::parse(&file_content);

    Ok(editor_content)
}

fn init(file_name: &str) -> io::Result<u32> {
    let orig_console_mode = terminal::init()?;
    vec!(
        EnterAlternateScreen,
        EnableMouseCapture,
        EnableBracketedPaste,
        SetWindowTitle(file_name.to_owned())
    ).execute()?;

    Ok(orig_console_mode)
}

fn close(orig_console_mode: u32) -> io::Result<()> {
    vec!(
        DisableBracketedPaste,
        DisableMouseCapture,
        LeaveAlternateScreen
    ).execute()?;

    terminal::restore_console_mode(orig_console_mode)
}
