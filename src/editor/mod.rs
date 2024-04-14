pub mod content;
pub mod cursor;
pub mod edit;
pub mod state;
// pub mod events;
pub mod viewport;
pub mod renderer;
pub mod macros;

use std::{collections::LinkedList, io};

use crate::terminal::{events::{Event::*, KeyCode::*, Mouse::*, MouseButton, MouseEvent::*, CTRL}, reader::read_event};

use self::{
    content::EditorContent, cursor::{Cursor, NavigationCommand}, edit::EditOp, renderer::EditorRenderer, state::EditorState, viewport::{ScrollCommand, ViewportDimensions}
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

    pub fn run(&mut self) -> io::Result<()> {
        self.refresh()?;
        self.event_loop()
    }

    fn event_loop(&mut self) -> io::Result<()> {
        loop {
            let event = read_event()?;
            let EditorState { ref cursor, ref content, ref viewport, .. } = self.state;

            let cursor_command = match event {
                Key(ref key, modifiers) => match (key, modifiers) {
                    (Esc, 0) => break Ok(()),
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

            self.move_and_scroll(cursor_command);
            self.scroll(scroll_command);

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

            self.refresh()?;
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

    fn insert(&mut self, str: &str) {
        let op = EditOp::insert(self.state.cursor.pos(), str);
        self.process(&op);
        self.move_and_scroll(self.state.cursor.move_to(&self.state.content, op.to()));
        self.push_to_undo_stack(op);
    }

    fn delete_char(&mut self) {
        if let Some(Cursor { col: right_col, row: right_row, .. }) = self.state.cursor.move_right(&self.state.content) {
            let op = EditOp::delete(&self.state.content, self.state.cursor.pos(), (right_row, right_col));
            self.process(&op);
            self.push_to_undo_stack(op);
        }
    }

    fn backspace(&mut self) {
        if self.state.cursor.is_at(0, 0) { return }
        self.move_and_scroll(self.state.cursor.move_left(&self.state.content));
        self.delete_char();
    }

    fn push_to_undo_stack(&mut self, op: EditOp) {
        self.undo_stack.push_front(op);
        self.redo_stack.clear()
    }

    fn undo(&mut self) {
        if let Some(edit_op) = self.undo_stack.pop_front() {
            let inverse_op = edit_op.inverse();
            self.process(&inverse_op);
            self.move_and_scroll(Some(Cursor::from(edit_op.from)));
            self.redo_stack.push_front(edit_op);
        }
    }

    fn redo(&mut self) {
        if let Some(edit_op) = self.redo_stack.pop_front() {
            self.process(&edit_op);
            self.move_and_scroll(Some(Cursor::from(edit_op.to())));
            self.undo_stack.push_front(edit_op);
        }
    }

    fn process(&mut self, op: &EditOp) {
        edit::process(&mut self.state.content, &op);
        self.state.mark_for_refresh()
    }

    fn move_and_scroll(&mut self, cursor_cmd: NavigationCommand) {
        if let Some(cursor) = cursor_cmd {
            self.state.cursor = cursor;
            let scroll_cmd = self.state.viewport.scroll_into_view(self.state.cursor.pos());
            self.scroll(scroll_cmd)
        }
    }

    fn scroll(&mut self, scroll_cmd: ScrollCommand) {
        if let Some((top, left)) = scroll_cmd {
            self.state.viewport.scroll(top, left);
            self.state.mark_for_refresh()
        }
    }
}
