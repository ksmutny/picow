use std::{fs, io};

use crate::{
    editor::{content::EditorContent, event_loop, state::EditorState, viewport::Viewport},
    terminal::{self, buffer::CommandExecutor, commands::Command::*}
};


pub fn start(file_name: &str) -> io::Result<()> {
    let content = read_content(file_name)?;

    let console_mode = init(file_name)?;

    run_editor(content)?;

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

fn run_editor(content: EditorContent) -> io::Result<()> {
    let mut state = EditorState::new(content, create_viewport().unwrap(), (0, 0));
    event_loop(&mut state)
}

fn create_viewport() -> io::Result<Viewport> {
    let (width, height) = terminal::terminal_size()?;
    Ok(Viewport::new(0, 0, width, height - 1))
}

fn close(orig_console_mode: u32) -> io::Result<()> {
    vec!(
        DisableBracketedPaste,
        DisableMouseCapture,
        LeaveAlternateScreen
    ).execute()?;

    terminal::restore_console_mode(orig_console_mode)
}
