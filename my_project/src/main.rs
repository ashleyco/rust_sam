mod app;
use my_project::game;
use my_project::unit::{Enemy, Wall};

fn main() {
    let mut game = game::Game::builder()
        .player_starting_health(10)
        .player_starting_speed(0.5)
        .enemies(
            (1..15)
                .map(|i| Enemy::with_speed(i as f32 * 0.05))
                .collect(),
        )
        .build();
    game.run();
    let x = 5; // or any appropriate value
    game.add_wall(Wall::new(x, game.get_height() as u16 - 1));
}

