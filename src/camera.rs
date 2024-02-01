use rayon::prelude::*;

use crate::{
    map::{Hit, Map},
    math::{Ray, Vector2},
};

const CLIP_DEGREES: fn(f32) -> f32 = |d| {
    if d > 360.0 {
        d - 360.0
    } else if d < 0.0 {
        d + 360.0
    } else {
        d
    }
};

pub struct Camera {
    pub pos: Vector2,
    angle_deg: f32,
    initial_rays: Vec<Ray>,
    rotated_rays: Vec<Ray>,
}

impl Camera {
    pub fn new(pos: Vector2, angle: f32, fov: i32, width: u32) -> Self {
        let rays = Self::init_rays(width as u32, fov);
        let mut camera = Self {
            pos,
            angle_deg: angle,
            initial_rays: rays.clone(),
            rotated_rays: rays,
        };
        camera.update_rays();
        camera
    }

    pub fn translate(&mut self, v: &Vector2) {
        self.pos = self.pos.add(v);
        self.update_rays();
    }

    pub fn rotate(&mut self, angle: f32) {
        self.angle_deg += angle;
        self.angle_deg = CLIP_DEGREES(self.angle_deg);
        self.update_rays();
    }

    fn update_rays(&mut self) {
        self.rotated_rays = self
            .initial_rays
            .par_iter()
            .map(|ray| ray.translate(&self.pos))
            .map(|ray| ray.rotate(self.angle_deg.to_radians()))
            .collect::<Vec<Ray>>();
    }

    pub fn forward(&self) -> Vector2 {
        Vector2::new(
            self.angle_deg.to_radians().cos(),
            self.angle_deg.to_radians().sin(),
        )
    }

    pub fn backward(&self) -> Vector2 {
        self.forward().multiply(-1.0)
    }

    pub fn left(&self) -> Vector2 {
        self.forward().rotate(90f32.to_radians())
    }

    pub fn right(&self) -> Vector2 {
        self.left().multiply(-1.0)
    }

    pub fn compute_frame<'a>(&self, map: &'a Map) -> Vec<Option<Hit<'a>>> {
        self.rotated_rays
            .par_iter()
            .map(|ray| map.cast(&ray))
            .collect()
    }

    fn init_rays(screen_width: u32, fov: i32) -> Vec<Ray> {
        let angle_step = fov as f32 / (screen_width - 1) as f32;
        let half_fov = fov as f32 / 2.0;
        (0..screen_width)
            .map(|i| {
                let angle = CLIP_DEGREES(half_fov - i as f32 * angle_step).to_radians();
                Ray::new(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(angle.cos(), angle.sin()),
                )
            })
            .collect()
    }
}
