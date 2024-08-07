use std::collections::LinkedList;

use super::{content::EditorContent, cursor::Cursor, edit::{self, EditOp}, pos::{PosInDocument, PosInDocumentExt}, viewport::Viewport};


pub struct EditorState {
    pub content: EditorContent,
    pub viewport: Viewport,
    pub cursor: Cursor,
    selection_pos: Option<PosInDocument>,
    undo_stack: LinkedList<EditOp>,
    redo_stack: LinkedList<EditOp>,
}

pub type ReRenderContent = bool;
pub type Selection = Option<(PosInDocument, PosInDocument)>;

impl EditorState {
    pub fn new(content: EditorContent, viewport: Viewport, cursor_pos: PosInDocument, selection_pos: Option<PosInDocument>) -> Self {
        Self {
            content, viewport,
            cursor: Cursor::from(cursor_pos),
            selection_pos,
            undo_stack: LinkedList::new(),
            redo_stack: LinkedList::new(),
        }
    }

    pub fn selection(&self) -> Selection {
        self.selection_pos.map(|selection_pos|
            match selection_pos.is_before(&self.cursor.pos()) {
                true => (selection_pos, self.cursor.pos()),
                false => (self.cursor.pos(), selection_pos)
            }
        )
    }


    pub fn move_cursor(&mut self, new_cursor: Cursor, is_selection: bool) -> ReRenderContent {
        let selection_updated = self.update_selection(is_selection);
        self.cursor = new_cursor;

        self.viewport.scroll_into_view(self.cursor.pos())
            .map(|scroll_to| self.scroll(scroll_to))
            .unwrap_or(selection_updated)
    }

    fn update_selection(&mut self, is_selection: bool) -> ReRenderContent {
        let was_selected = self.selection_pos.is_some();

        self.selection_pos = match is_selection {
            true => self.selection_pos.or(Some(self.cursor.pos())),
            false => None,
        };

        was_selected || self.selection_pos.is_some()
    }


    pub fn select_all(&mut self) -> ReRenderContent {
        self.selection_pos = Some((0, 0));
        self.cursor = Cursor::from(self.content.last_line_end());
        true
    }


    pub fn scroll(&mut self, scroll_to: PosInDocument) -> ReRenderContent {
        let (top, left) = scroll_to;
        self.viewport.scroll(top, left);
        true
    }


    pub fn edit(&mut self, edit_op: EditOp) -> ReRenderContent {
        self.process(&edit_op);
        self.undo_stack.push_front(edit_op);
        self.redo_stack.clear();
        true
    }

    fn process(&mut self, op: &EditOp) {
        let next_pos = edit::process(&mut self.content, &op);
        self.move_cursor(Cursor::from(next_pos), false);
    }

    pub fn undo(&mut self) -> ReRenderContent {
        self.undo_stack.pop_front()
            .map(|edit_op| {
                self.process(&edit_op.inverse());
                self.redo_stack.push_front(edit_op)
            })
            .is_some()
    }

    pub fn redo(&mut self) -> ReRenderContent {
        self.redo_stack.pop_front()
            .map(|edit_op| {
                self.process(&edit_op);
                self.undo_stack.push_front(edit_op)
            })
            .is_some()
    }
}
