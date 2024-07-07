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
                event => Self::process_event(event, &mut self.state)
            }
        }
    }

    fn render(&mut self) -> io::Result<()> {
        renderer::render(&self.state)?;
        self.state.mark_rendered();
        Ok(())
    }

    pub fn process_event(event: Event, state: &mut EditorState) {
        cursor_command(&event, state).map(|(cursor, is_selection)|
            state.move_cursor(cursor, is_selection)
        );

        scroll_command(&event, state).map(|scroll_to|
            state.scroll(scroll_to)
        );

        if let Some(cmd) = undo_redo_command(&event) {
            match cmd {
                Undo => state.undo(),
                Redo => state.redo(),
            }
        }

        edit_command(&event, state).map(|edit_op| state.edit(edit_op));
    }

    // fn resize(&mut self, (width, height): ViewportDimensions) {
    //     self.state.viewport.resize(width, height);
    //     self.state.mark_for_refresh()
    // }
}
