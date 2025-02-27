use crate::point::Point2d;
use crate::traits::Position;


#[derive(Debug)]
pub struct Player {
    pub position: Point2d<f32>,
    pub speed: f32,
    pub health: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Point2d::new(0.0, 0.0),
            speed: 1.0,
            health: 100,
        }
    }
}
pub struct PlayerBuilder {
    speed: f32,
    health: u8,
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self {
            speed: 1.0,   // Default speed
            health: 100,  // Default health
        }
    }

    pub fn health(mut self, health: u8) -> Self {
        self.health = health;
        self
    }

    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = speed as f32;
        self
    }

    pub fn build(self) -> Player {
        Player {
            position: Point2d { x: 0.0, y: 0.0 }, // Default position
            speed: self.speed,
            health: self.health,
        }
    }
}


impl Position<f32> for Player {
    fn position(&self) -> Point2d<f32> {
        self.position
    }

    fn set_position(&mut self, position: Point2d<f32>) {
        self.position = position;
    }
}



