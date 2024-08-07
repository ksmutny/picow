use crate::terminal::events::{Event::{self, *}, KeyCode::*, Mouse::*, MouseButton, MouseEvent::*, CTRL, SHIFT};

use super::{clipboard::copy_to_clipboard, content::EditorContent, cursor::Cursor, edit::EditOp, pos::PosInDocument, state::{EditorState, ReRenderContent}, viewport::ScrollCommand};


pub fn process_event(event: &Event, state: &mut EditorState) -> ReRenderContent {
    if let Some((cursor, is_selection)) = cursor_command(event, state) {
        state.move_cursor(cursor, is_selection)
    } else if is_select_all(event) {
        state.select_all()
    } else if let Some(scroll_to) = scroll_command(event, state) {
        state.scroll(scroll_to)
    } else if is_undo(event) {
        state.undo()
    } else if is_redo(event) {
        state.redo()
    } else if let Some(edit_op) = edit_command(event, state) {
        state.edit(edit_op)
    } else if let Some(edit_op) = clipboard_command(event, state) {
        state.edit(edit_op)
    } else {
        false
    }
}


type CursorCommand = Option<(Cursor, bool)>;

fn cursor_command(event: &Event, state: &EditorState) -> CursorCommand {
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
        Mouse(Button(MouseButton::Left, Press | Drag, column, row)) => cursor.move_to(content, viewport.to_absolute((row, column))),
        _ => None
    };

    cursor_command.map(|cursor| (
        cursor,
        matches!(event, Key(_, SHIFT)) || matches!(event, Mouse(Button(_, Drag, _, _)))
    ))
}


fn is_select_all(event: &Event) -> bool {
    matches!(event, Key(Char('A'), CTRL))
}

fn clipboard_command(event: &Event, state: &EditorState) -> EditCommand {
    state.selection().and_then(|(from, to)|
        match event {
            Key(Char('C'), CTRL) => {
                copy_selection_to_clipboard(&state.content, from, to);
                None
            },
            Key(Char('X'), CTRL) => {
                copy_selection_to_clipboard(&state.content, from, to);
                delete(from, to, &state.content)
            }
        _ => None
        }
    )
}

fn copy_selection_to_clipboard(content: &EditorContent, from: PosInDocument, to: PosInDocument) {
    copy_to_clipboard(content.selected_text(from, to));
}


fn scroll_command(event: &Event, state: &EditorState) -> ScrollCommand {
    let EditorState { ref content, ref viewport, .. } = state;

    match event {
        Key(Up, CTRL) | Mouse(WheelUp(_, _)) => viewport.scroll_up(1),
        Key(Down, CTRL) | Mouse(WheelDown(_, _)) => viewport.scroll_down(1, content.last_line_row()),
        _ => None
    }
}


fn is_undo(event: &Event) -> bool {
    matches!(event, Key(Char('Z'), CTRL))
}

fn is_redo(event: &Event) -> bool {
    matches!(event, Key(Char('Y'), CTRL))
}


type EditCommand = Option<EditOp>;

fn edit_command(event: &Event, state: &EditorState) -> EditCommand {
    let EditorState { ref content, ref cursor, .. } = state;
    let selection = state.selection();

    match selection {
        Some((from, to)) => match event {
            Key(ref key, modifiers) => match (key, modifiers) {
                (Char(c), 0) => replace(content, from, to, &c.to_string()),
                (Enter, 0) => replace(content, from, to, "\n"),
                (Backspace, 0) => delete(from, to, content),
                (Delete, 0) => delete(from, to, content),
                _ => None
            },
            Paste(s) => replace(content, from, to, &s),
            _ => None
        },
        None => match event {
            Key(ref key, modifiers) => match (key, modifiers) {
                (Char(c), 0) => insert_char(cursor, *c),
                (Enter, 0) => insert_char(cursor, '\n'),
                (Backspace, 0) => backspace(cursor, content),
                (Delete, 0) => delete_char(cursor, content),
                _ => None
            },
            Paste(s) => insert(cursor, &s),
            _ => None

        }
    }
}

fn insert_char(cursor: &Cursor, c: char) -> EditCommand {
    insert(cursor, &c.to_string())
}

fn delete_char(cursor: &Cursor, content: &EditorContent) -> EditCommand {
    cursor.move_right(content).and_then(|cursor_right| {
        delete(cursor.pos(), cursor_right.pos(), content)
    })
}

fn backspace(cursor: &Cursor, content: &EditorContent) -> EditCommand {
    cursor.move_left(content).and_then(|cursor_left| {
        delete(cursor_left.pos(), cursor.pos(), content)
    })
}

fn insert(cursor: &Cursor, str: &str) -> EditCommand {
    Some(EditOp::insert(cursor.pos(), str))
}

fn delete(from: PosInDocument, to: PosInDocument, content: &EditorContent) -> EditCommand {
    Some(EditOp::delete(content, from, to))
}

fn replace(content: &EditorContent, from: PosInDocument, to: PosInDocument, str: &str) -> EditCommand {
    Some(EditOp::replace(content, from, to, str))
}
