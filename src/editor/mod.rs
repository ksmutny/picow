pub mod content;
pub mod cursor;
pub mod edit;
pub mod state;
pub mod events;
pub mod viewport;
pub mod renderer;
pub mod row;
pub mod macros;

use std::io;

use crate::terminal::{events::{Event::Key, KeyCode::Esc}, reader::read_event};

use events::process_event;
use state::EditorState;


pub struct Editor {
    pub state: EditorState,
}

impl Editor {
    pub fn new(state: EditorState) -> Self {
        Self { state }
    }

    pub fn event_loop(&mut self) -> io::Result<()> {
        let mut rerender_content = true;
        loop {
            renderer::render(&self.state, rerender_content)?;

            match read_event()? {
                Key(Esc, 0) => break Ok(()),
                event => rerender_content = process_event(event, &mut self.state)
            }
        }
    }
}
