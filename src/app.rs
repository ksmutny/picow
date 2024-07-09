use std::{fs, io};

use crate::{
    editor::{content::EditorContent, event_loop, state::EditorState, viewport::Viewport},
    terminal
};


pub fn start(file_name: &str) -> io::Result<()> {
    let mut state = create_editor_state(file_name)?;

    terminal::on_alternate_screen(file_name, ||
        event_loop(&mut state)
    )
}


fn create_editor_state(file_name: &str) -> io::Result<EditorState> {
    let content = read_content(file_name)?;
    let viewport = create_viewport()?;

    Ok(EditorState::new(content, viewport, (0, 0), None))
}

fn read_content(file_name: &str) -> io::Result<EditorContent> {
    let file_content = fs::read_to_string(file_name)?;

    Ok(EditorContent::parse(&file_content))
}

fn create_viewport() -> io::Result<Viewport> {
    let (width, height) = terminal::terminal_size()?;

    Ok(Viewport::new(0, 0, width, height - 1))
}
