pub mod content;
pub mod state;
// pub mod events;
pub mod navigation;
pub mod renderer;

use std::io;

use crate::terminal::{events::{Event::*, KeyCode::*, Mouse::*, MouseButton, MouseEvent::*, CTRL}, reader::read_event};

use self::{
    content::EditorContent, navigation::{MoveCursorTo, NavigationCommand, ScrollCommand, ScrollViewportTo},
    renderer::EditorRenderer,
    state::{EditorState, ViewportDimensions}
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
        EditorRenderer::open("editor.rs".to_string())?;
        self.renderer.refresh(&self.state);
        self.renderer.refresh_cursor(&self.state);
        self.renderer.flush()?;

        self.event_loop()?;
        EditorRenderer::close()
    }

    fn event_loop(&mut self) -> io::Result<()> {
        loop {
            let event = read_event()?;

            let cursor_command = match event {
                Key(ref key, modifiers) => match (key, modifiers) {
                    (Esc, 0) => break Ok(()),
                    (Home, 0) => self.state.move_line_start(),
                    (End, 0) => self.state.move_line_end(),
                    (Up, 0) => self.state.move_up(1),
                    (Down, 0) => self.state.move_down(1),
                    (Right, 0) => self.state.move_right(),
                    (Left, 0) => self.state.move_left(),
                    (PageDown, 0) => self.state.move_page_down(),
                    (PageUp, 0) => self.state.move_page_up(),

                    (Home, CTRL) => self.state.move_document_start(),
                    (End, CTRL) => self.state.move_document_end(),

                    _ => None
                },
                Mouse(Button(MouseButton::Left, Press, column, row)) => self.state.click(self.state.viewport.to_absolute((column, row))),
                _ => None
            };

            let scroll_command = if let Some(MoveCursorTo(x, y, _)) = cursor_command {
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
        if let Some(MoveCursorTo(x, y, is_vertical)) = cursor_cmd {
            if is_vertical {
                self.state.start_or_keep_vertical_navigation()
            }
            else {
                self.state.end_vertical_navigation()
            }
            self.state.cursor_pos = (x, y);
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
        let new_cursor_pos = self.state.content.insert(self.state.cursor_pos, str);
        self.move_and_scroll(self.state.click(new_cursor_pos));
        self.renderer.refresh(&self.state);
    }

    fn delete_char(&mut self) {
         if let Some(MoveCursorTo(right_col, right_row, _)) = self.state.move_right() {
            let (left_col, left_row) = self.state.cursor_pos;
            self.state.content.delete((left_row, left_col), (right_row, right_col));
            self.renderer.refresh(&self.state);
        }
    }

    fn backspace(&mut self) {
        if self.state.cursor_pos == (0, 0) { return }
        self.move_and_scroll(self.state.move_left());
        self.delete_char();
    }

    fn move_and_scroll(&mut self, cursor_command: NavigationCommand) {
        self.queue((self.state.scroll_into_view(self.state.cursor_pos), cursor_command));
    }
}
