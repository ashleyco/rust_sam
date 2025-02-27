use crate::point::Point2d;
use crate::traits::Position;


#[derive(Debug)]
pub struct Collectible {
    pub position: Point2d<u32>,
}

impl Default for Collectible {
    fn default() -> Self {
        Self {
            position: Point2d::new(0, 0),
        }
    }
}

impl Position<u32> for Collectible {
    fn position(&self) -> Point2d<u32> {
        self.position
    }

    fn set_position(&mut self, position: Point2d<u32>) {
        self.position = position;
    }
}