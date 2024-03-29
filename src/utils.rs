
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }

    pub fn from_points(top_left: (u16, u16), bottom_right: (u16, u16)) -> Self {
        Self::new(
            top_left.0,
            top_left.1,
            bottom_right.0 - top_left.0,
            bottom_right.1 - top_left.1,
        )
    }
}
