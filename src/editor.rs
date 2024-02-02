pub mod state;
pub mod navigation;
pub mod renderer;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

use crate::terminal::{self, *};

use self::{navigation::{CursorCommand, NavigationCommand, ScrollCommand}, renderer::EditorRenderer, state::EditorState};


pub struct Editor {
    state: EditorState,
    renderer: EditorRenderer,
    delimiter: String,
}

impl Editor {
    pub fn new(rows: Vec<String>, delimiter: String) -> Self {
        let terminal_size = terminal::terminal_size().unwrap();
        Self {
            state: EditorState::new(rows, terminal_size, (0, 0), (0, 0)),
            renderer: EditorRenderer::new(),
            delimiter,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        EditorRenderer::open()?;
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

                    match code {
                        Up | Down | PageUp | PageDown => self.state.keep_vertical_navigation(),
                        _ => self.state.end_vertical_navigation(),
                    }

                    match (code, modifiers) {
                        (Esc, _) => break Ok(()),

                        // (Char(c), _) => self.commands.queue(Command::Print(c.to_string())),

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
                        (PageUp, _) => self.queue(self.state.move_up(self.state.viewport_height() as usize - 1)),
                        (PageDown, _) =>  self.queue(self.state.move_down(self.state.viewport_height() as usize - 1)),
                        _ => {}
                    }
                },
                Event::Mouse(MouseEvent { kind, column, row, .. }) => {
                    use MouseButton::*;

                    match kind {
                        MouseEventKind::Down(Left) => self.queue(self.state.click(column + 1, row + 1)),
                        MouseEventKind::ScrollDown => self.queue(self.state.scroll_down(1)),
                        MouseEventKind::ScrollUp => self.queue(self.state.scroll_up(1)),
                        _ => {}
                    }
                },
                Event::Resize(width, height) => self.resize((width, height)),
                _ => {}
            }
            self.renderer.refresh_status_bar(&self.state, &self.delimiter);
            self.renderer.flush()?;
        }
    }

    fn queue(&mut self, (scroll_cmd, cursor_cmd): NavigationCommand) {
        if let ScrollCommand::ScrollTo(x, y) = scroll_cmd {
            self.state.scroll_pos = (x, y);
            self.renderer.refresh(&self.state);
        }
        if let CursorCommand::MoveTo(x, y) = cursor_cmd {
            self.state.cursor_pos = (x, y);
            self.renderer.refresh_cursor(&self.state);
        }
    }

    fn resize(&mut self, terminal_size: Coordinates) {
        self.state.viewport_size = (terminal_size.0, terminal_size.1 - 1);
        self.renderer.refresh(&self.state)
    }
}
