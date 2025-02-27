use crate::point::Point2d;
use crate::traits::Position;

#[derive(Debug)]
pub struct Wall {
    pub position: Point2d<u32>,
}

impl Wall {
    pub fn new(x: u16, y: u16) -> Self {
        Wall {
            position: Point2d::new(x as u32, y as u32), 
        }
    }
}

impl Default for Wall {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Position<u32> for Wall {
    fn position(&self) -> Point2d<u32> {
        self.position
    }

    fn set_position(&mut self, position: Point2d<u32>) {
        self.position = position;
    }
}