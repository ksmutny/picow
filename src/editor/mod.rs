pub mod content;
pub mod cursor;
pub mod edit;
pub mod state;
// pub mod events;
pub mod scroll;
pub mod split;
pub mod renderer;
pub mod macros;

use std::io;

use crate::terminal::{events::{Event::*, KeyCode::*, Mouse::*, MouseButton, MouseEvent::*, CTRL}, reader::read_event};

use self::{
    content::EditorContent, cursor::{Cursor, NavigationCommand}, renderer::EditorRenderer, scroll::{ScrollCommand, ScrollViewportTo}, state::{EditorState, ViewportDimensions}
};


pub struct Editor {
    state: EditorState,
    renderer: EditorRenderer,
}

impl Editor {
    pub fn new(content: EditorContent) -> Self {
        let viewport = EditorRenderer::create_viewport().unwrap();
        Self {
            state: EditorState::new(content, viewport, (0, 0)),
            renderer: EditorRenderer::new()
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.renderer.refresh(&self.state);
        self.renderer.refresh_cursor(&self.state);
        self.renderer.flush()?;

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
                Mouse(Button(MouseButton::Left, Press, column, row)) => cursor.click(content, viewport.to_absolute((column, row))),
                _ => None
            };

            let scroll_command = if let Some(Cursor { col: x, row: y, .. }) = cursor_command {
                self.state.scroll_into_view((x, y))
            } else {
                match event {
                    Key(Up, CTRL) | Mouse(WheelUp(_, _)) => self.state.scroll_up(1),
                    Key(Down, CTRL) | Mouse(WheelDown(_, _)) => self.state.scroll_down(1),
                    _ => None
                }
            };

            self.queue((scroll_command, cursor_command));

            match event {
                Key(ref key, _) => match key {
                    Char(c) => self.insert_char(*c),
                    Enter => self.insert_char('\n'),
                    Backspace => self.backspace(),
                    Delete => self.delete_char(),
                    _ => {}
                },
                Paste(s) => self.insert(&s),
                _ => {}
            }

            self.renderer.refresh_status_bar(&self.state);
            self.renderer.flush()?;
        }
    }


    fn queue(&mut self, (scroll_cmd, cursor_cmd): (ScrollCommand, NavigationCommand)) {
        if let Some(ScrollViewportTo(left, top)) = scroll_cmd {
            self.state.scroll_viewport(left, top);
            self.renderer.refresh(&self.state);
        }
        if let Some(cursor) = cursor_cmd {
            self.state.cursor = cursor;
        }
    }

    fn resize(&mut self, (width, height): ViewportDimensions) {
        self.state.resize_viewport(width, height);
        self.renderer.refresh(&self.state)
    }

    fn insert_char(&mut self, c: char) {
        self.insert(&c.to_string());
    }

    fn insert(&mut self, str: &str) {
        let (col, row) = self.state.cursor.pos();
        let op = edit::insert_op((row, col), str);
        let (new_row, new_col) = op.to();
        edit::process(&mut self.state.content, &op);
        self.move_and_scroll(self.state.cursor.click(&self.state.content, (new_col, new_row)));
        self.renderer.refresh(&self.state);
    }

    fn delete_char(&mut self) {
         if let Some(Cursor { col: right_col, row: right_row, .. }) = self.state.cursor.move_right(&self.state.content) {
            let (left_col, left_row) = self.state.cursor.pos();
            self.state.content.delete((left_row, left_col), (right_row, right_col));
            self.renderer.refresh(&self.state);
        }
    }

    fn backspace(&mut self) {
        if self.state.cursor.is_at(0, 0) { return }
        self.move_and_scroll(self.state.cursor.move_left(&self.state.content));
        self.delete_char();
    }

    fn move_and_scroll(&mut self, cursor_command: NavigationCommand) {
        self.queue((self.state.scroll_into_view(self.state.cursor.pos()), cursor_command));
    }
}
