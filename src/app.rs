use std::{fs, io};

use crate::{
    editor::{content::EditorContent, event_loop, state::EditorState, viewport::Viewport},
    terminal
};


pub fn start(file_name: &str) -> io::Result<()> {
    let content = read_content(file_name)?;

    terminal::on_alternate_screen(file_name, ||
        run_editor(content)
    )
}

fn read_content(file_name: &str) -> io::Result<EditorContent> {
    let file_content = fs::read_to_string(file_name)?;
    let editor_content = EditorContent::parse(&file_content);

    Ok(editor_content)
}

fn run_editor(content: EditorContent) -> io::Result<()> {
    let mut state = EditorState::new(content, create_viewport().unwrap(), (0, 0), None);
    event_loop(&mut state)
}

fn create_viewport() -> io::Result<Viewport> {
    let (width, height) = terminal::terminal_size()?;
    Ok(Viewport::new(0, 0, width, height - 1))
}
