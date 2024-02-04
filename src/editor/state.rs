pub type CursorPosition = (u16, u16);
pub type ViewportDimensions = (u16, u16);
pub type AbsPosition = (usize, usize);


pub struct EditorState {
    pub viewport: Viewport,
    pub cursor_pos: AbsPosition,
    pub lines: Vec<String>,
    vertical_nav: VerticalNavigation,
}

pub struct Viewport {
    pub left: usize,
    pub top: usize,
    pub width: u16,
    pub height: u16,
}

impl Viewport {
    pub fn new(left: usize, top: usize, width: u16, height: u16) -> Self {
        Self { left, top, width, height }
    }

    pub fn pos(&self) -> AbsPosition { (self.left, self.top) }
    pub fn size(&self) -> ViewportDimensions { (self.width, self.height) }

    pub fn cursor_within(&self, (cursor_x, cursor_y): AbsPosition) -> bool {
        cursor_x >= self.left && cursor_x < self.left + self.width as usize &&
        cursor_y >= self.top && cursor_y < self.top + self.height as usize
    }

    pub fn to_relative(&self, (x, y): AbsPosition) -> CursorPosition {
        ((x - self.left + 1) as u16, (y - self.top + 1) as u16)
    }

    pub fn to_absolute(&self, (x, y): CursorPosition) -> AbsPosition {
        (x as usize + self.left - 1, y as usize + self.top - 1)
    }
}

struct VerticalNavigation {
    in_progress: bool,
    last_x: usize,
}

impl EditorState {

    pub fn new(lines: Vec<String>, viewport: Viewport, cursor_pos: AbsPosition) -> Self {
        let vertical_nav = VerticalNavigation { in_progress: false, last_x: 0 };
        Self { viewport, cursor_pos, lines, vertical_nav }
    }

    pub fn scroll_viewport(&mut self, x: usize, y: usize) {
        self.viewport.left = x;
        self.viewport.top = y;
    }

    pub fn resize_viewport(&mut self, width: u16, height: u16) {
        self.viewport.width = width;
        self.viewport.height = height - 1;
    }

    pub fn cursor_x(&self) -> usize { self.cursor_pos.0 }
    pub fn cursor_y(&self) -> usize { self.cursor_pos.1 }

    pub fn start_or_keep_vertical_navigation(&mut self) {
        if !self.vertical_nav.in_progress {
            self.vertical_nav.in_progress = true;
            self.vertical_nav.last_x = self.cursor_x();
        }
    }

    pub fn end_vertical_navigation(&mut self) {
        self.vertical_nav.in_progress = false;
    }

    pub fn vertical_navigation_x(&self) -> usize {
        self.vertical_nav.last_x
    }
}
