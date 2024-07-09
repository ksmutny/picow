pub mod macros;

pub mod clipboard;
pub mod content;
pub mod cursor;
pub mod edit;
pub mod events;
pub mod pos;
pub mod renderer;
pub mod row;
pub mod state;
pub mod viewport;

use std::io;

use crate::terminal::{events::{Event::Key, KeyCode::Esc}, reader::read_event};


pub fn event_loop(state: &mut state::EditorState) -> io::Result<()> {
    let mut rerender_content = true;
    loop {
        renderer::render(&state, rerender_content)?;

        match read_event()? {
            Key(Esc, 0) => break Ok(()),
            event => rerender_content = events::process_event(&event, state)
        }
    }
}
