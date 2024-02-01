use std::time;

use raylib::prelude::*;

use crate::{
    config::{Config, Screen},
    scene::Scene,
};

const CEILING_COLOR: Color = Color::LIGHTGRAY;
const FLOOR_COLOR: Color = Color::BROWN;

pub enum Command {
    Move(Direction, f32),
    Look(f32),
}

pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

pub struct Game {
    config: Config,
    rl: RaylibHandle,
    thread: RaylibThread,
    scene: Scene,
}

impl Game {
    pub fn new(config: Config) -> Self {
        let (rl, thread) = raylib::init()
            .size(config.screen.width as i32, config.screen.height as i32)
            .title("Rayna")
            .build();

        let scene_config =
            serde_json::from_str(include_str!("../assets/scene.json")).expect("read scene");

        Self {
            config: config.clone(),
            rl,
            thread,
            scene: Scene::from_config(&config, scene_config),
        }
    }

    pub fn run(&mut self) {
        let mut time_keeper = time::Instant::now();
        while !self.rl.window_should_close() {
            let now = time::Instant::now();
            self.handle_inputs(now.duration_since(time_keeper).as_secs_f32());
            self.draw();
            time_keeper = now;
        }
    }

    fn handle_inputs(&mut self, delta: f32) {
        let translation_speed = delta * self.config.player.move_speed;
        let rotation_speed = delta * self.config.player.look_speed;

        let mut inputs = vec![];

        if self.rl.is_key_down(KeyboardKey::KEY_W) {
            inputs.push(Command::Move(Direction::Forward, translation_speed));
        } else if self.rl.is_key_down(KeyboardKey::KEY_S) {
            inputs.push(Command::Move(Direction::Backward, translation_speed));
        }

        if self.rl.is_key_down(KeyboardKey::KEY_A) {
            inputs.push(Command::Move(Direction::Left, translation_speed));
        } else if self.rl.is_key_down(KeyboardKey::KEY_D) {
            inputs.push(Command::Move(Direction::Right, translation_speed));
        }

        if self.rl.is_key_down(KeyboardKey::KEY_Q) {
            inputs.push(Command::Look(rotation_speed));
        } else if self.rl.is_key_down(KeyboardKey::KEY_E) {
            inputs.push(Command::Look(-rotation_speed));
        }

        self.scene.handle_inputs(&inputs);
    }

    fn draw(&mut self) {
        let Screen { width, height, .. } = self.config.screen;
        let (width, height) = (width as i32, height as i32);

        let mut d = self.rl.begin_drawing(&self.thread);

        // Draw the ceiling and the floor
        d.draw_rectangle(0, 0, width, height / 2, CEILING_COLOR);
        d.draw_rectangle(0, height / 2, width, height / 2, FLOOR_COLOR);

        // Draw the walls
        self.scene
            .get_frame()
            .into_iter()
            .for_each(|((x, y), color)| d.draw_pixel(x, y, color));

        // Draw the crosshair
        // let crosshair_size = 5;
        // d.draw_line(
        //     width / 2 - crosshair_size,
        //     height / 2,
        //     width / 2 + crosshair_size,
        //     height / 2,
        //     Color::BLACK,
        // );
        // d.draw_line(
        //     width / 2,
        //     height / 2 - crosshair_size,
        //     width / 2,
        //     height / 2 + crosshair_size,
        //     Color::BLACK,
        // );

        // Draw the FPS counter
        d.draw_text(&d.get_fps().to_string(), 10, 10, 20, Color::BLACK);
    }
}
