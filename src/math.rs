use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Pos2 {
    pub x: i32,
    pub y: i32,
}

impl Pos2 {
    pub fn new(x: i32, y: i32) -> Self {
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
