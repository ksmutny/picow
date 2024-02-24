use std::cmp::min;

use super::state::{AbsPosition, EditorState, Viewport};


#[derive(PartialEq, Debug)]
pub struct MoveCursorTo(pub usize, pub usize, pub bool);

#[derive(PartialEq, Debug)]
pub struct ScrollViewportTo(pub usize, pub usize);

pub type NavigationCommand = Option<MoveCursorTo>;
pub type ScrollCommand = Option<ScrollViewportTo>;

impl EditorState {

    fn move_to(&self, pos: AbsPosition, is_vertical: bool) -> NavigationCommand {
        let new_cursor_pos = self.within_text(pos, is_vertical);
        self.move_cmd(new_cursor_pos, is_vertical)
    }

    fn within_text(&self, (x, y): AbsPosition, is_vertical: bool) -> AbsPosition {
        let new_y = min(y, self.last_line_y());
        let new_x = min(if is_vertical { self.vertical_navigation_x() } else { x }, self.line_len(new_y));
        (new_x, new_y)
    }

    pub fn scroll_into_view(&self, (x, y): AbsPosition) -> ScrollCommand {
        let Viewport { left, top, width, height } = self.viewport;

        let scroll_into = |cursor_pos, viewport_start, viewport_size| {
            if cursor_pos < viewport_start { cursor_pos }
            else if cursor_pos >= viewport_start + viewport_size as usize { cursor_pos - viewport_size as usize + 1 }
            else { viewport_start }
        };

        self.scroll_cmd((scroll_into(x, left, width), scroll_into(y, top, height)))
    }

    fn move_cmd(&self, new_pos @ (x, y): AbsPosition, is_vertical: bool) -> NavigationCommand {
        if new_pos == self.cursor_pos { None } else { Some(MoveCursorTo(x, y, is_vertical)) }
    }

    fn scroll_cmd(&self, new_pos @ (x, y): AbsPosition) -> ScrollCommand {
        if new_pos == self.viewport.pos() { None } else { Some(ScrollViewportTo(x, y)) }
    }

    pub fn move_up(&self, n: usize) -> NavigationCommand {
        self.move_vertical(|y| y - min(n, y))
    }

    pub fn move_page_up(&self) -> NavigationCommand {
        self.move_up(self.viewport.height as usize - 1)
    }

    pub fn move_down(&self, n: usize) -> NavigationCommand {
        self.move_vertical(|y| y + min(n, self.last_line_y() - y))
    }

    pub fn move_page_down(&self) -> NavigationCommand {
        self.move_down(self.viewport.height as usize - 1)
    }

    fn move_vertical<F>(&self, new: F) -> NavigationCommand
    where
        F: Fn(usize) -> usize,
    {
        let (x, y) = self.cursor_pos;
        self.move_to((x, new(y)), true)
    }

    pub fn move_left(&self) -> NavigationCommand {
        let move_to = match self.cursor_pos {
            (0, 0) => (0, 0),
            (0, y) => self.line_end(y - 1),
            (x, y) => (x - 1, y)
        };
        self.move_to(move_to, false)
    }

    pub fn move_right(&self) -> NavigationCommand {
        let move_to = match self.cursor_pos {
            (x, y) if x < self.line_len(y) => (x + 1, y),
            (_, y) if y < self.last_line_y() => (0, y + 1),
            _ => self.cursor_pos
        };
        self.move_to(move_to, false)
    }

    pub fn move_line_start(&self) -> NavigationCommand {
        self.move_to((0, self.cursor_y()), false)
    }

    pub fn move_line_end(&self) -> NavigationCommand {
        self.move_to(self.line_end(self.cursor_y()), false)
    }

    pub fn move_document_start(&self) -> NavigationCommand {
        self.move_to((0, 0), false)
    }

    pub fn move_document_end(&self) -> NavigationCommand {
        self.move_to(self.last_line_end(), false)
    }

    pub fn click(&self, new_pos: AbsPosition) -> NavigationCommand {
        self.move_to(new_pos, false)
    }

    fn line_end(&self, y: usize) -> AbsPosition {
        (self.line_len(y), y)
    }

    fn line_len(&self, y: usize) -> usize {
        self.content.lines[y].len()
    }

    fn last_line_end(&self) -> AbsPosition {
        self.line_end(self.last_line_y())
    }

    fn last_line_y(&self) -> usize {
        self.content.lines.len() - 1
    }

    pub fn scroll_to(&self, (scroll_left, scroll_top): AbsPosition) -> ScrollCommand {
        let new_scroll_top = min(scroll_top, self.last_line_y());
        self.scroll_cmd((scroll_left, new_scroll_top))
    }

    pub fn scroll_up(&self, n: usize) -> ScrollCommand {
        self.scroll_vertical(|y| y - min(n, y))
    }

    pub fn scroll_down(&self, n: usize) -> ScrollCommand {
        self.scroll_vertical(|y| y + min(n, self.last_line_y() - y))
    }

    fn scroll_vertical<F>(&self, new: F) -> ScrollCommand
    where
        F: Fn(usize) -> usize,
    {
        let Viewport { left, top, .. } = self.viewport;
        self.scroll_to((left, new(top)))
    }
}
