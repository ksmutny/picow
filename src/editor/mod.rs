pub mod content;
pub mod cursor;
pub mod edit;
pub mod state;
pub mod events;
pub mod viewport;
pub mod renderer;
pub mod row;
pub mod macros;

use std::{collections::LinkedList, io};

use crate::terminal::{events::{Event::{self, *}, KeyCode::*, CTRL}, reader::read_event};

use self::{edit::EditOp, state::EditorState};


enum UndoRedo {
    Undo,
    Redo,
}
type UndoRedoCommand = Option<UndoRedo>;

pub struct Editor {
    pub state: EditorState,
    undo_stack: LinkedList<EditOp>,
    redo_stack: LinkedList<EditOp>,
}

impl Editor {
    pub fn new(state: EditorState) -> Self {
        Self {
            state,
            undo_stack: LinkedList::new(),
            redo_stack: LinkedList::new(),
        }
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
        Self::cursor_command(&event, &self.state).map(|(cursor, is_selection)|
            self.state.move_cursor(cursor, is_selection)
        );

        Self::scroll_command(&event, &self.state).map(|scroll_to|
            self.state.scroll(scroll_to)
        );

        if let Some(cmd) = Self::undo_redo_command(&event) {
            match cmd {
                UndoRedo::Undo => self.undo(),
                UndoRedo::Redo => self.redo(),
            }
        }

        Self::edit_command(&event, &self.state).map(|edit_op| self.process(edit_op));
    }

    fn undo_redo_command(event: &Event) -> UndoRedoCommand {
        match event {
            Key(Char('Y'), CTRL) => Some(UndoRedo::Redo),
            Key(Char('Z'), CTRL) => Some(UndoRedo::Undo),
            _ => None
        }
    }

    // fn resize(&mut self, (width, height): ViewportDimensions) {
    //     self.state.viewport.resize(width, height);
    //     self.state.mark_for_refresh()
    // }

    fn process(&mut self, op: EditOp) {
        self.state.process(&op);
        self.undo_stack.push_front(op);
        self.redo_stack.clear()
    }

    fn undo(&mut self) {
        if let Some(edit_op) = self.undo_stack.pop_front() {
            self.state.process(&edit_op.inverse());
            self.redo_stack.push_front(edit_op);
        }
    }

    fn redo(&mut self) {
        if let Some(edit_op) = self.redo_stack.pop_front() {
            self.state.process(&edit_op);
            self.undo_stack.push_front(edit_op);
        }
    }
}
