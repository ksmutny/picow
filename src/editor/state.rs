pub type CursorPosition = (u16, u16);
pub type ViewportDimensions = (u16, u16);
pub type AbsPosition = (usize, usize);


pub struct EditorState {
    pub viewport_size: ViewportDimensions,
    pub scroll_pos: AbsPosition,
    pub cursor_pos: AbsPosition,
    pub lines: Vec<String>,
    vertical_nav: VerticalNavigation,
}

struct VerticalNavigation {
    in_progress: bool,
    last_x: usize,
}

impl EditorState {

    pub fn new(lines: Vec<String>, viewport_size: ViewportDimensions, scroll_pos: AbsPosition, cursor_pos: AbsPosition) -> Self {
        Self {
            viewport_size,
            scroll_pos,
            cursor_pos,
            lines,
            vertical_nav: VerticalNavigation { in_progress: false, last_x: 0 },
        }
    }

    pub fn viewport_usize(&self) -> AbsPosition { (self.viewport_width() as usize, self.viewport_height() as usize) }
    pub fn viewport_width(&self) -> u16 { self.viewport_size.0 }
    pub fn viewport_height(&self) -> u16 { self.viewport_size.1 }

    pub fn scroll_left(&self) -> usize { self.scroll_pos.0 }
    pub fn scroll_top(&self) -> usize { self.scroll_pos.1 }

    pub fn cursor_x(&self) -> usize { self.cursor_pos.0 }
    pub fn cursor_y(&self) -> usize { self.cursor_pos.1 }

    pub fn keep_vertical_navigation(&mut self) {
        if !self.vertical_nav.in_progress {
            self.vertical_nav.in_progress = true;
            self.vertical_nav.last_x = self.cursor_x();
        }
    }

    pub fn end_vertical_navigation(&mut self) {
        self.vertical_nav.in_progress = false;
    }

    pub fn vertical_navigation_x_or(&self, x: usize) -> usize {
        if self.vertical_nav.in_progress { self.vertical_nav.last_x } else { x }
    }
}
