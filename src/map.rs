
use crate::{
    config::WallConfig,
    math::{geometry::Line, Intersection, Ray},
};
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Map {
    pub walls: Vec<Wall>,
}

#[derive(Debug, Clone)]
pub struct Wall {
    pub line: Line,
    pub length: f32,
    pub texture_name: String,
    pub repeat_texture: i32,
}

#[derive(Debug, Clone)]
pub struct Hit<'a> {
    pub intersec: Intersection,
    pub wall: &'a Wall,
}

impl Map {
    pub fn from_lines(walls: Vec<WallConfig>) -> Self {
        Self {
            walls: walls
                .into_iter()
                .map(
                    |WallConfig {
                         line,
                         texture: texture_name,
                     }| Wall {
                        line,
                        length: line.end.subtract(&line.start).magnitude(),
                        texture_name,
                        repeat_texture: line.end.subtract(&line.start).magnitude() as i32,
                    },
                )
                .collect(),
        }
    }

    pub fn cast(&self, ray: &Ray) -> Option<Hit> {
        self.walls
            .par_iter()
            .filter_map(|wall| {
                wall.line.cast(&ray).map(|intersection| Hit {
                    intersec: intersection,
                    wall,
                })
            })
            .min_by(|a, b| {
                a.intersec
                    .distance
                    .partial_cmp(&b.intersec.distance)
                    .unwrap()
            })
    }
}
