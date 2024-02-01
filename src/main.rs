use game::Game;

mod camera;
mod config;
mod game;
mod map;
mod math;
mod scene;
mod texture;

/// TODO
/// - Optim
/// - Skybox
/// - Better collision
/// - Map editor
/// - Fog

fn main() {
    env_logger::init();
    let config = serde_json::from_str(include_str!("../assets/config.json")).expect("read config");
    Game::new(config).run();
}
