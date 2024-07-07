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

use crate::terminal::{events::{Event::{self, Key}, KeyCode::Esc}, reader::read_event};

use events::{UndoRedo::*, cursor_command, edit_command, scroll_command, undo_redo_command};
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
                event => self.process_event(event)
            }
        }
    }

    fn render(&mut self) -> io::Result<()> {
        renderer::render(&self.state)?;
        self.state.mark_rendered();
        Ok(())
    }

    pub fn process_event(&mut self, event: Event) {
        cursor_command(&event, &self.state).map(|(cursor, is_selection)|
            self.state.move_cursor(cursor, is_selection)
        );

        scroll_command(&event, &self.state).map(|scroll_to|
            self.state.scroll(scroll_to)
        );

        if let Some(cmd) = undo_redo_command(&event) {
            match cmd {
                Undo => self.state.undo(),
                Redo => self.state.redo(),
            }
        }

        edit_command(&event, &self.state).map(|edit_op| self.state.edit(edit_op));
    }

    // fn resize(&mut self, (width, height): ViewportDimensions) {
    //     self.state.viewport.resize(width, height);
    //     self.state.mark_for_refresh()
    // }
}
