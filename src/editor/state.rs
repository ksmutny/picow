pub type CursorPosition = (u16, u16);
pub type ViewportDimensions = (u16, u16);
pub type ScrollPosition = (usize, usize);


pub struct EditorState {
    pub viewport_size: ViewportDimensions,
    pub scroll_pos: ScrollPosition,
    pub cursor_pos: CursorPosition,
    pub lines: Vec<String>,
    pub vertical_nav: VerticalNavigation,
}

impl EditorState {
    pub fn viewport_width(&self) -> u16 { self.viewport_size.0 }
    pub fn viewport_height(&self) -> u16 { self.viewport_size.1 }

    pub fn scroll_left(&self) -> usize { self.scroll_pos.0 }
    pub fn scroll_top(&self) -> usize { self.scroll_pos.1 }

    pub fn cursor_x(&self) -> u16 { self.cursor_pos.0 }
    pub fn cursor_y(&self) -> u16 { self.cursor_pos.1 }

    pub fn cursor_pos_abs(&self) -> ScrollPosition { (self.cursor_x_abs(), self.cursor_y_abs()) }
    pub fn cursor_x_abs(&self) -> usize { self.scroll_left() + self.cursor_x() as usize - 1 }
    pub fn cursor_y_abs(&self) -> usize { self.scroll_top() + self.cursor_y() as usize - 1 }
}


pub struct VerticalNavigation {
    in_progress: bool,
    last_x: usize,
}

impl VerticalNavigation {

    pub fn new() -> Self {
        VerticalNavigation {
            in_progress: false,
            last_x: 0,
        }
    }

    pub fn start(&mut self, x: usize) {
        if !self.in_progress {
            self.in_progress = true;
            self.last_x = x;
        }
    }

    pub fn end(&mut self) {
        self.in_progress = false;
    }

    pub fn x(&self, x: usize) -> usize {
        if self.in_progress { self.last_x } else { x }
    }
}
