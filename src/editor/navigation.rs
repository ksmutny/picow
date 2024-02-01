#[path="./tuple_ops.rs"]
mod tuple_ops;

use std::cmp::min;

use super::state::{CursorPosition, EditorState, ScrollPosition};
use CursorCommand::*;
use ScrollCommand::*;


#[derive(PartialEq, Debug)]
pub enum CursorCommand {
    NoMove,
    MoveTo(u16, u16)
}

#[derive(PartialEq, Debug)]
pub enum ScrollCommand {
    NoScroll,
    ScrollTo(usize, usize)
}

pub type NavigationCommand = (ScrollCommand, CursorCommand);

impl EditorState {

    fn move_to_abs(&self, pos_abs: ScrollPosition) -> NavigationCommand {
        let new_cursor_abs = self.within_text(pos_abs);
        let new_scroll_pos = self.scroll_into_view(new_cursor_abs);
        let new_cursor_pos = Self::to_relative(new_cursor_abs, new_scroll_pos);

        (self.scroll_cmd(new_scroll_pos), self.move_cmd(new_cursor_pos))
    }

    fn within_text(&self, (x_abs, y_abs): ScrollPosition) -> ScrollPosition {
        let new_y_abs = min(y_abs, self.lines.len() - 1);
        let new_x_abs = min(self.vertical_nav.x(x_abs), self.lines[new_y_abs].len());
        (new_x_abs, new_y_abs)
    }

    fn scroll_into_view(&self, (x_abs, y_abs): ScrollPosition) -> ScrollPosition {
        let (scroll_left, scroll_top) = self.scroll_pos;
        let (width, height) = self.viewport_usize();

        (
            if x_abs < scroll_left { x_abs }
            else if x_abs >= scroll_left + width { x_abs - width + 1 }
            else { scroll_left },

            if y_abs < scroll_top { y_abs }
            else if y_abs >= scroll_top + height { y_abs - height + 1 }
            else { scroll_top }
        )
    }

    fn to_relative((x_abs, y_abs): ScrollPosition, (scroll_left, scroll_top): ScrollPosition) -> CursorPosition {
        ((x_abs - scroll_left + 1) as u16, (y_abs - scroll_top + 1) as u16)
    }

    fn to_absolute((x, y): CursorPosition, (scroll_left, scroll_top): ScrollPosition) -> ScrollPosition {
        (x as usize + scroll_left - 1, y as usize + scroll_top - 1)
    }

    fn move_cmd(&self, new_pos @ (x, y): CursorPosition) -> CursorCommand {
        if new_pos == self.cursor_pos { NoMove } else { MoveTo(x, y) }
    }

    fn scroll_cmd(&self, new_pos @ (x, y): ScrollPosition) -> ScrollCommand {
        if new_pos == self.scroll_pos { NoScroll } else { ScrollTo(x, y) }
    }

    pub fn click(&self, x: u16, y: u16) -> NavigationCommand {
        self.move_to_abs(Self::to_absolute((x, y), self.scroll_pos))
    }

    pub fn move_up(&self, n: usize) -> NavigationCommand {
        self.move_vertical(|y| y - min(n, y))
    }

    pub fn move_down(&self, n: usize) -> NavigationCommand {
        self.move_vertical(|y| y + min(n, self.lines.len() - 1 - y))
    }

    fn move_vertical<F>(&self, new: F) -> NavigationCommand
    where
        F: Fn(usize) -> usize,
    {
        let (x, y) = self.cursor_pos_abs();
        self.move_to_abs((x, new(y)))
    }

    pub fn move_left(&self) -> NavigationCommand {
        let move_to = match self.cursor_pos_abs() {
            (0, 0) => (0, 0),
            (0, y) => self.line_end(y - 1),
            (x, y) => (x - 1, y)
        };
        self.move_to_abs(move_to)
    }

    pub fn move_right(&self) -> NavigationCommand {
        let (x, y) = self.cursor_pos_abs();
        let move_to = if x < self.lines[y].len() {
            (x + 1, y)
        } else if y < self.lines.len() - 1 {
            (0, y + 1)
        } else {
            (x, y)
        };
        self.move_to_abs(move_to)
    }

    pub fn move_line_start(&self) -> NavigationCommand {
        self.move_to_abs((0, self.cursor_y_abs()))
    }

    pub fn move_line_end(&self) -> NavigationCommand {
        self.move_to_abs(self.line_end(self.cursor_y_abs()))
    }

    pub fn move_document_start(&self) -> NavigationCommand {
        self.move_to_abs((0, 0))
    }

    pub fn move_document_end(&self) -> NavigationCommand {
        self.move_to_abs(self.last_line_end())
    }

    fn line_end(&self, y: usize) -> ScrollPosition {
        (self.lines[y].len(), y)
    }

    fn last_line_end(&self) -> ScrollPosition {
        self.line_end(self.lines.len() - 1)
    }

    pub fn scroll_to(&self, (scroll_left, scroll_top): ScrollPosition) -> NavigationCommand {
        let new_scroll_top = min(scroll_top, self.lines.len() - 1);
        (self.scroll_cmd((scroll_left, new_scroll_top)), NoMove)
    }

    pub fn scroll_up(&self, n: usize) -> NavigationCommand {
        self.scroll_vertical(|y| y - min(n, y))
    }

    pub fn scroll_down(&self, n: usize) -> NavigationCommand {
        self.scroll_vertical(|y| y + min(n, self.lines.len() - 1 - y))
    }

    fn scroll_vertical<F>(&self, new: F) -> NavigationCommand
    where
        F: Fn(usize) -> usize,
    {
        let (x, y) = self.scroll_pos;
        self.scroll_to((x, new(y)))
    }
}
