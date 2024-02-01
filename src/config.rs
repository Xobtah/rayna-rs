use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

use crate::math::{geometry::Line, Vector2};

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Screen {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Player {
    #[serde(rename = "lookSpeed")]
    pub look_speed: f32,
    #[serde(rename = "moveSpeed")]
    pub move_speed: f32,
    #[serde(rename = "collisionRadius")]
    pub collision_radius: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub fov: i32,
    pub player: Player,
    pub screen: Screen,
    pub entry: PathBuf,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct CameraConfig {
    pub position: Vector2,
    pub angle: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WallConfig {
    pub texture: String,
    pub line: Line,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapConfig {
    pub walls: Vec<WallConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FogConfig {
    pub distance: f32,
    pub color: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SceneConfig {
    pub camera: CameraConfig,
    pub textures: HashMap<String, String>,
    pub map: MapConfig,
    pub fog: Option<FogConfig>,
}
