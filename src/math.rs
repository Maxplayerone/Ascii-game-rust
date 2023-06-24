use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Pos2 {
    pub x: i16,
    pub y: i16,
}

impl Pos2 {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl Add for Pos2 {
    type Output = Pos2;
    fn add(self, rhs: Pos2) -> Pos2 {
        Pos2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
