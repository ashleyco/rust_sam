pub mod collectible;
pub mod enemy;
pub mod player;
pub mod wall;

pub use collectible::Collectible;
pub use enemy::Enemy;
pub use player::{Player,PlayerBuilder};
pub use wall::Wall;

pub struct Game{
    pub width: u32,
    pub height: u32,
}




