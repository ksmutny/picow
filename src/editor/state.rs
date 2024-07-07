use std::collections::LinkedList;

use super::{content::{EditorContent, PosInDocument}, cursor::Cursor, edit::{self, EditOp, EditOpKind::*}, viewport::Viewport};


pub struct EditorState {
    pub content: EditorContent,
    pub viewport: Viewport,
    pub cursor: Cursor,
    pub selection_pos: Option<PosInDocument>,
    pub marked_for_render: bool,
    undo_stack: LinkedList<EditOp>,
    redo_stack: LinkedList<EditOp>,
}

impl EditorState {
    pub fn new(content: EditorContent, viewport: Viewport, cursor_pos: PosInDocument) -> Self {
        let (row, col) = cursor_pos;
        let cursor = Cursor::new(row, col);

        Self {
            content, viewport, cursor,
            selection_pos: None,
            marked_for_render: true,
            undo_stack: LinkedList::new(),
            redo_stack: LinkedList::new(),
        }
    }

    pub fn move_cursor(&mut self, new_cursor: Cursor, is_selection: bool) {
        self.update_selection(is_selection);
        self.cursor = new_cursor;

        self.viewport.scroll_into_view(self.cursor.pos()).map(|scroll_to|
            self.scroll(scroll_to)
        );
    }

    fn update_selection(&mut self, is_selection: bool) {
        if is_selection {
            if self.selection_pos.is_none() {
                self.selection_pos = Some(self.cursor.pos());
            }
        } else {
            self.selection_pos = None;
        }
    }

    pub fn scroll(&mut self, scroll_to: PosInDocument) {
        let (top, left) = scroll_to;
        self.viewport.scroll(top, left);
        self.mark_for_render()
    }


    pub fn edit(&mut self, edit_op: EditOp) {
        self.process(&edit_op);
        self.undo_stack.push_front(edit_op);
        self.redo_stack.clear();
    }


    fn process(&mut self, op: &EditOp) {
        edit::process(&mut self.content, &op);

        let cursor = Cursor::from(match op.kind {
            Insert => op.to(),
            Delete => op.from,
        });

        self.move_cursor(cursor, false);

        self.mark_for_render()
    }

    pub fn undo(&mut self) {
        self.undo_stack.pop_front().map(|edit_op| {
            self.process(&edit_op.inverse());
            self.redo_stack.push_front(edit_op);
        });
    }

    pub fn redo(&mut self) {
        self.redo_stack.pop_front().map(|edit_op| {
            self.process(&edit_op);
            self.undo_stack.push_front(edit_op);
        });
    }

    fn mark_for_render(&mut self) {
        self.marked_for_render = true
    }

    pub fn mark_rendered(&mut self) {
        self.marked_for_render = false;
    }
}
