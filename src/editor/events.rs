use crate::terminal::events::{Event::{self, *}, KeyCode::*, Mouse::*, MouseButton, MouseEvent::*, CTRL, SHIFT};

use super::{Editor, cursor::NavigationCommand, state::EditorState, viewport::ScrollCommand};


impl Editor {
    pub fn cursor_command(event: &Event, state: &EditorState) -> (NavigationCommand, bool) {
        let EditorState { ref cursor, ref content, ref viewport, .. } = state;

        let cursor_command = match *event {
            Key(ref key, modifiers) => match (key, modifiers) {
                (Home, 0 | SHIFT) => cursor.move_line_start(content),
                (End, 0 | SHIFT) => cursor.move_line_end(content),
                (Up, 0 | SHIFT) => cursor.move_up(content, 1),
                (Down, 0 | SHIFT) => cursor.move_down(content, 1),
                (Right, 0 | SHIFT) => cursor.move_right(content),
                (Left, 0 | SHIFT) => cursor.move_left(content),
                (PageDown, 0 | SHIFT) => cursor.move_down(content, viewport.height as usize - 1),
                (PageUp, 0 | SHIFT) => cursor.move_up(content, viewport.height as usize - 1),

                (Home, CTRL) => cursor.move_document_start(content),
                (End, CTRL) => cursor.move_document_end(content),

                _ => None
            },
            Mouse(Button(MouseButton::Left, Press, column, row)) => cursor.move_to(content, viewport.to_absolute((row, column))),
            _ => None
        };

        let is_selection = match event {
            Key(_, SHIFT) if cursor_command.is_some() => true,
            _ => false
        };

        (cursor_command, is_selection)
    }

    pub fn scroll_command(event: &Event, state: &EditorState) -> ScrollCommand {
        let EditorState { ref content, ref viewport, .. } = state;

        match event {
            Key(Up, CTRL) | Mouse(WheelUp(_, _)) => viewport.scroll_up(1),
            Key(Down, CTRL) | Mouse(WheelDown(_, _)) => viewport.scroll_down(1, content.last_line_row()),
            _ => None
        }
    }
}
