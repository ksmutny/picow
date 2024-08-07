use std::{fs, io};

use crate::{
    editor::{content::EditorContent, events, renderer, state::EditorState, viewport::Viewport},
    terminal::{self, events::{Event::Key, KeyCode::{Char, Esc}, CTRL}}
};


pub fn start(file_name: &str) -> io::Result<()> {
    let mut state = create_editor_state(file_name)?;

    terminal::on_alternate_screen(file_name, ||
        event_loop(file_name, &mut state)
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


fn event_loop(file_name: &str, state: &mut EditorState) -> io::Result<()> {
    let mut rerender_content = true;
    loop {
        terminal::output(
            renderer::render(&state, rerender_content)
        )?;

        match terminal::read_event()? {
            Key(Esc, 0) => break Ok(()),
            Key(Char('S'), CTRL) => save_file(file_name, &state.content)?,
            event => rerender_content = events::process_event(&event, state)
        }
    }
}

fn save_file(file_name: &str, content: &EditorContent) -> io::Result<()> {
    let content = content.to_string();
    fs::write(file_name, content)
}
