use super::{content::{EditorContent, PosInDocument}, cursor::{Cursor, NavigationCommand}, edit::{self, EditOp, EditOpKind::*}, viewport::{ScrollCommand, Viewport}};


pub struct EditorState {
    pub content: EditorContent,
    pub viewport: Viewport,
    pub cursor: Cursor,
    pub marked_for_render: bool,
}

impl EditorState {
    pub fn new(content: EditorContent, viewport: Viewport, cursor_pos: PosInDocument) -> Self {
        let (row, col) = cursor_pos;
        let cursor = Cursor::new(row, col);
        Self { content, viewport, cursor, marked_for_render: true }
    }

    pub fn process(&mut self, op: &EditOp) {
        edit::process(&mut self.content, &op);

        self.move_cursor(Some(Cursor::from(match op.kind {
            Insert => op.to(),
            Delete => op.from,
        })));

        self.mark_for_render()
    }

    pub fn move_cursor(&mut self, cursor_cmd: NavigationCommand) {
        if let Some(cursor) = cursor_cmd {
            self.cursor = cursor;
            let scroll_cmd = self.viewport.scroll_into_view(self.cursor.pos());
            self.scroll(scroll_cmd)
        }
    }

    pub fn scroll(&mut self, scroll_cmd: ScrollCommand) {
        if let Some((top, left)) = scroll_cmd {
            self.viewport.scroll(top, left);
            self.mark_for_render()
        }
    }


    fn mark_for_render(&mut self) {
        self.marked_for_render = true
    }

    pub fn mark_rendered(&mut self) {
        self.marked_for_render = false;
    }
}
