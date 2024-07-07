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
        loop {
            self.render()?;

            match read_event()? {
                Key(Esc, 0) => break Ok(()),
                event => process_event(event, &mut self.state)
            }
        }
    }

    fn render(&mut self) -> io::Result<()> {
        renderer::render(&self.state)?;
        self.state.mark_rendered();
        Ok(())
    }
}
