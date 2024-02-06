pub mod content;
pub mod state;
pub mod navigation;
pub mod renderer;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

use self::{
    content::EditorContent, navigation::{MoveCursorTo, NavigationCommand, ScrollViewportTo},
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

        self.event_loop()?;
        EditorRenderer::close()
    }

    fn event_loop(&mut self) -> io::Result<()> {
        loop {
            match event::read()? {
                Event::Key(KeyEvent { kind: KeyEventKind::Press, code, modifiers, .. }) => {
                    use KeyCode::*;
                    const CTRL: KeyModifiers = KeyModifiers::CONTROL;

                    match (code, modifiers) {
                        (Esc, _) => break Ok(()),

                        (Char(c), _) => self.insert_char(c),
                        (Enter, _) => self.insert_char('\n'),
                        (Backspace, _) => self.backspace(),
                        (Delete, _) => self.delete_char(),

                        (Up, CTRL) => self.queue(self.state.scroll_up(1)),
                        (Down, CTRL) => self.queue(self.state.scroll_down(1)),
                        (Home, CTRL) =>  self.queue(self.state.move_document_start()),
                        (End, CTRL) =>  self.queue(self.state.move_document_end()),

                        (Right, _) => self.queue(self.state.move_right()),
                        (Left, _) => self.queue(self.state.move_left()),
                        (Up, _) => self.queue(self.state.move_up(1)),
                        (Down, _) => self.queue(self.state.move_down(1)),
                        (Home, _) => self.queue(self.state.move_line_start()),
                        (End, _) => self.queue(self.state.move_line_end()),
                        (PageUp, _) => self.queue(self.state.move_page_up()),
                        (PageDown, _) =>  self.queue(self.state.move_page_down()),
                        _ => {}
                    }
                },
                Event::Mouse(MouseEvent { kind, column, row, .. }) => {
                    use MouseButton::*;

                    match kind {
                        MouseEventKind::Down(Left) => self.queue(self.state.click(self.state.viewport.to_absolute((column + 1, row + 1)))),
                        MouseEventKind::ScrollDown => self.queue(self.state.scroll_down(1)),
                        MouseEventKind::ScrollUp => self.queue(self.state.scroll_up(1)),
                        _ => {}
                    }
                },
                Event::Resize(width, height) => self.resize((width, height)),
                _ => {}
            }
            self.renderer.refresh_status_bar(&self.state);
            self.renderer.flush()?;
        }
    }

    fn queue(&mut self, (scroll_cmd, cursor_cmd): NavigationCommand) {
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
        let (col, row) = self.state.cursor_pos;
        self.state.content.insert((row, col), &c.to_string());

        self.queue(self.state.move_right());
        self.renderer.refresh(&self.state);
    }

    fn delete_char(&mut self) {
        if let (_, Some(MoveCursorTo(right_col, right_row, _))) = self.state.move_right() {
            let (left_col, left_row) = self.state.cursor_pos;
            self.state.content.delete((left_row, left_col), (right_row, right_col));
            self.renderer.refresh(&self.state);
        }
    }

    fn backspace(&mut self) {
        if self.state.cursor_pos == (0, 0) { return }
        self.queue(self.state.move_left());
        self.delete_char();
    }
}
