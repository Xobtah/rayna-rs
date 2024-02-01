use rayon::prelude::*;
use std::{collections::HashMap, path::Path};

use log::{error, info};
use raylib::color::Color;

use crate::{
    camera::Camera,
    config::{Config, SceneConfig},
    game::{Command, Direction},
    map::Map,
    math::Ray,
    texture::Texture,
};

pub struct Scene {
    config: Config,
    camera: Camera,
    map: Map,
    default_texture: Texture,
    textures: HashMap<String, Texture>,
}

impl Scene {
    pub fn from_config(config: &Config, scene_config: SceneConfig) -> Self {
        let textures = scene_config
            .textures
            .into_iter()
            .map(|(name, path)| {
                info!("Loading texture: {path}");
                let path = Path::new(&path);
                if !path.exists() {
                    error!("Texture not found: {}", path.display());
                }
                (name, Texture::from_png(&path).unwrap_or_default())
            })
            .collect();

        Self {
            config: config.clone(),
            camera: Camera::new(
                scene_config.camera.position,
                scene_config.camera.angle,
                config.fov,
                config.screen.width,
            ),
            map: Map::from_lines(scene_config.map.walls),
            default_texture: Texture::default(),
            textures,
        }
    }

    pub fn handle_inputs(&mut self, inputs: &[Command]) {
        inputs.into_iter().for_each(|input| match input {
            Command::Move(direction, speed) if self.can_move(direction, *speed) => {
                match direction {
                    Direction::Forward => self
                        .camera
                        .translate(&self.camera.forward().multiply(*speed)),
                    Direction::Backward => self
                        .camera
                        .translate(&self.camera.backward().multiply(*speed)),
                    Direction::Left => self.camera.translate(&self.camera.left().multiply(*speed)),
                    Direction::Right => {
                        self.camera.translate(&self.camera.right().multiply(*speed))
                    }
                }
            }
            Command::Look(speed) => self.camera.rotate(*speed),
            _ => {}
        });
    }

    // TODO Can do better
    fn can_move(&self, direction: &Direction, speed: f32) -> bool {
        let direction = match direction {
            Direction::Forward => self.camera.forward().multiply(speed),
            Direction::Backward => self.camera.backward().multiply(speed),
            Direction::Left => self.camera.left().multiply(speed),
            Direction::Right => self.camera.right().multiply(speed),
        };
        if let Some(hit) = self.map.cast(&Ray::new(self.camera.pos, direction)) {
            hit.intersec.distance > self.config.player.collision_radius
        } else {
            true
        }
    }

    pub fn get_frame(&self) -> Vec<((i32, i32), Color)> {
        self.camera
            .compute_frame(&self.map)
            .into_par_iter()
            .enumerate()
            .filter_map(|(x, hit)| hit.map(|hit| (x, hit)))
            .filter(|(_, hit)| {
                hit.intersec.distance > f32::EPSILON
                    && hit.intersec.distance < self.config.screen.height as f32
            })
            .map(|(x, hit)| {
                let wall_height = self.config.screen.height as f32 / hit.intersec.distance;
                let wall_top = (self.config.screen.height as f32 - wall_height) / 2.0;
                let texture = self.textures
                    .get(&hit.wall.texture_name)
                    .unwrap_or(&self.default_texture);
                let texture_length = hit.wall.length / hit.wall.repeat_texture as f32;
                let offset_in_wall = hit.intersec.offset * hit.wall.length;
                let texture_offset = (offset_in_wall % texture_length) / texture_length;
                (
                    (x as i32, wall_top as i32),
                    texture.get_line(
                        (texture_offset * 255.) as u8,
                        wall_height as u32,
                    )
                )
            })
            .flat_map(|(pos, line)| {
                line.into_par_iter()
                    .enumerate()
                    .map(move |(i, color)| ((pos.0, pos.1 + i as i32), color))
            })
            .collect()
    }
}
