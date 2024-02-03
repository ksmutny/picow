use std::cmp::min;

use super::state::{AbsPosition, EditorState, Viewport};


#[derive(PartialEq, Debug)]
pub struct MoveCursorTo(pub usize, pub usize);

#[derive(PartialEq, Debug)]
pub struct ScrollViewportTo(pub usize, pub usize);

pub type NavigationCommand = (Option<ScrollViewportTo>, Option<MoveCursorTo>);

impl EditorState {

    pub fn move_to(&self, pos: AbsPosition) -> NavigationCommand {
        let new_cursor_pos = self.within_text(pos);
        let new_scroll_pos = self.scroll_into_view(new_cursor_pos);

        (self.scroll_cmd(new_scroll_pos), self.move_cmd(new_cursor_pos))
    }

    fn within_text(&self, (x, y): AbsPosition) -> AbsPosition {
        let new_y = min(y, self.last_line_y());
        let new_x = min(self.vertical_navigation_x_or(x), self.line_len(new_y));
        (new_x, new_y)
    }

    fn scroll_into_view(&self, (x, y): AbsPosition) -> AbsPosition {
        let Viewport { left, top, width, height } = self.viewport;

        let scroll_into = |cursor_pos, viewport_start, viewport_size| {
            if cursor_pos < viewport_start { cursor_pos }
            else if cursor_pos >= viewport_start + viewport_size as usize { cursor_pos - viewport_size as usize + 1 }
            else { viewport_start }
        };

        (scroll_into(x, left, width), scroll_into(y, top, height))
    }

    fn move_cmd(&self, new_pos @ (x, y): AbsPosition) -> Option<MoveCursorTo> {
        if new_pos == self.cursor_pos { None } else { Some(MoveCursorTo(x, y)) }
    }

    fn scroll_cmd(&self, new_pos @ (x, y): AbsPosition) -> Option<ScrollViewportTo> {
        if new_pos == self.viewport.pos() { None } else { Some(ScrollViewportTo(x, y)) }
    }

    pub fn move_up(&self, n: usize) -> NavigationCommand {
        self.move_vertical(|y| y - min(n, y))
    }

    pub fn move_down(&self, n: usize) -> NavigationCommand {
        self.move_vertical(|y| y + min(n, self.last_line_y() - y))
    }

    fn move_vertical<F>(&self, new: F) -> NavigationCommand
    where
        F: Fn(usize) -> usize,
    {
        let (x, y) = self.cursor_pos;
        self.move_to((x, new(y)))
    }

    pub fn move_left(&self) -> NavigationCommand {
        let move_to = match self.cursor_pos {
            (0, 0) => (0, 0),
            (0, y) => self.line_end(y - 1),
            (x, y) => (x - 1, y)
        };
        self.move_to(move_to)
    }

    pub fn move_right(&self) -> NavigationCommand {
        let move_to = match self.cursor_pos {
            (x, y) if x < self.line_len(y) => (x + 1, y),
            (_, y) if y < self.last_line_y() => (0, y + 1),
            _ => self.cursor_pos
        };
        self.move_to(move_to)
    }

    pub fn move_line_start(&self) -> NavigationCommand {
        self.move_to((0, self.cursor_y()))
    }

    pub fn move_line_end(&self) -> NavigationCommand {
        self.move_to(self.line_end(self.cursor_y()))
    }

    pub fn move_document_start(&self) -> NavigationCommand {
        self.move_to((0, 0))
    }

    pub fn move_document_end(&self) -> NavigationCommand {
        self.move_to(self.last_line_end())
    }

    fn line_end(&self, y: usize) -> AbsPosition {
        (self.line_len(y), y)
    }

    fn line_len(&self, y: usize) -> usize {
        self.lines[y].len()
    }

    fn last_line_end(&self) -> AbsPosition {
        self.line_end(self.last_line_y())
    }

    fn last_line_y(&self) -> usize {
        self.lines.len() - 1
    }

    pub fn scroll_to(&self, (scroll_left, scroll_top): AbsPosition) -> NavigationCommand {
        let new_scroll_top = min(scroll_top, self.last_line_y());
        (self.scroll_cmd((scroll_left, new_scroll_top)), None)
    }

    pub fn scroll_up(&self, n: usize) -> NavigationCommand {
        self.scroll_vertical(|y| y - min(n, y))
    }

    pub fn scroll_down(&self, n: usize) -> NavigationCommand {
        self.scroll_vertical(|y| y + min(n, self.last_line_y() - y))
    }

    fn scroll_vertical<F>(&self, new: F) -> NavigationCommand
    where
        F: Fn(usize) -> usize,
    {
        let Viewport { left, top, .. } = self.viewport;
        self.scroll_to((left, new(top)))
    }
}
