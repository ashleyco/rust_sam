use crate::point::Point2d;
use crate::traits::Position;

#[derive(Debug)]
pub struct Enemy {
    pub position: Point2d<f32>,
    pub speed: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32, speed: f32) -> Self {
        Self {
            position: Point2d::new(x, y),
            speed,
        }
    }

    pub fn with_speed(speed: f32) -> Self {
        Self {
            position: Point2d::new(0.0, 0.0), 
            speed,
        }
    }
}

impl Position<f32> for Enemy {
    fn position(&self) -> Point2d<f32> {
        self.position
    }

    fn set_position(&mut self, position: Point2d<f32>) {
        self.position = position;
    }
}