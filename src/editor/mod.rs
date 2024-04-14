pub mod content;
pub mod cursor;
pub mod edit;
pub mod state;
// pub mod events;
pub mod viewport;
pub mod renderer;
pub mod macros;

use std::{collections::LinkedList, io};

use crate::terminal::{events::{Event::{self, *}, KeyCode::*, Mouse::*, MouseButton, MouseEvent::*, CTRL}, reader::read_event};

use self::{
    content::{EditorContent, PosInDocument}, edit::EditOp, renderer::EditorRenderer, state::EditorState, viewport::ViewportDimensions
};


pub struct Editor {
    state: EditorState,
    undo_stack: LinkedList<EditOp>,
    redo_stack: LinkedList<EditOp>,
    renderer: EditorRenderer,
}

impl Editor {
    pub fn new(content: EditorContent) -> Self {
        let viewport = EditorRenderer::create_viewport().unwrap();
        Self {
            state: EditorState::new(content, viewport, (0, 0)),
            undo_stack: LinkedList::new(),
            redo_stack: LinkedList::new(),
            renderer: EditorRenderer::new(),
        }
    }

    pub fn event_loop(&mut self) -> io::Result<()> {
        loop {
            self.refresh()?;

            match read_event()? {
                Key(Esc, 0) => break Ok(()),
                event => self.process_event(event)
            }
        }
    }

    fn process_event(&mut self, event: Event) {
        let EditorState { ref cursor, ref content, ref viewport, .. } = self.state;

        let cursor_command = match event {
            Key(ref key, modifiers) => match (key, modifiers) {
                (Home, 0) => cursor.move_line_start(content),
                (End, 0) => cursor.move_line_end(content),
                (Up, 0) => cursor.move_up(content, 1),
                (Down, 0) => cursor.move_down(content, 1),
                (Right, 0) => cursor.move_right(content),
                (Left, 0) => cursor.move_left(content),
                (PageDown, 0) => cursor.move_down(content, viewport.height as usize - 1),
                (PageUp, 0) => cursor.move_up(content, viewport.height as usize - 1),

                (Home, CTRL) => cursor.move_document_start(content),
                (End, CTRL) => cursor.move_document_end(content),

                _ => None
            },
            Mouse(Button(MouseButton::Left, Press, column, row)) => cursor.move_to(content, viewport.to_absolute((row, column))),
            _ => None
        };

        let scroll_command = match event {
            Key(Up, CTRL) | Mouse(WheelUp(_, _)) => viewport.scroll_up(1),
            Key(Down, CTRL) | Mouse(WheelDown(_, _)) => viewport.scroll_down(1, content.last_line_row()),
            _ => None
        };

        self.state.move_cursor(cursor_command);
        self.state.scroll(scroll_command);

        match event {
            Key(ref key, modifiers) => match (key, modifiers) {
                (Char(c), 0) => self.insert_char(*c),
                (Enter, 0) => self.insert_char('\n'),
                (Backspace, 0) => self.backspace(),
                (Delete, 0) => self.delete_char(),
                (Char('Y'), CTRL) => self.redo(),
                (Char('Z'), CTRL) => self.undo(),
                _ => {}
            },
            Paste(s) => self.insert(&s),
            _ => {}
        }
    }

    fn refresh(&mut self) -> io::Result<()> {
        if self.state.marked_for_refresh {
            self.renderer.refresh(&self.state);
            self.state.marked_for_refresh = false
        }
        self.renderer.refresh_status_bar(&self.state);
        self.renderer.flush()
    }


    fn resize(&mut self, (width, height): ViewportDimensions) {
        self.state.viewport.resize(width, height);
        self.state.mark_for_refresh()
    }

    fn insert_char(&mut self, c: char) {
        self.insert(&c.to_string());
    }

    fn delete_char(&mut self) {
        if let Some(cursor) = self.state.cursor.move_right(&self.state.content) {
            self.delete(self.state.cursor.pos(), cursor.pos())
        }
    }

    fn backspace(&mut self) {
        if let Some(cursor) = self.state.cursor.move_left(&self.state.content) {
            self.delete(cursor.pos(), self.state.cursor.pos())
        }
    }

    fn insert(&mut self, str: &str) {
        self.process(EditOp::insert(self.state.cursor.pos(), str))
    }

    fn delete(&mut self, from: PosInDocument, to: PosInDocument) {
        self.process(EditOp::delete(&self.state.content, from, to))
    }

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
