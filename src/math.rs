use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn multiply(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn rotate(&self, angle: f32) -> Vector2 {
        let new_x = self.x * angle.cos() - self.y * angle.sin();
        let new_y = self.x * angle.sin() + self.y * angle.cos();
        Vector2::new(new_x, new_y)
    }

    // pub fn normalize(&mut self) {
    //     let length = (self.x * self.x + self.y * self.y).sqrt();
    //     self.x /= length;
    //     self.y /= length;
    // }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn cross(&self, other: &Vector2) -> f32 {
        self.x * other.y - self.y * other.x
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub distance: f32,
    pub position: Vector2,
    pub offset: f32, // Value between 0 and 1
}

pub mod geometry {
    use super::*;

    #[derive(Debug, Clone, Copy, Deserialize)]
    pub struct Line {
        pub start: Vector2,
        pub end: Vector2,
    }

    impl Line {
        // Casts the ray along the direction and returns the position of the intersection
        pub fn cast(&self, ray: &Ray) -> Option<Intersection> {
            let p0 = ray.origin;
            let d = ray.direction;
            let a = self.start;
            let b = self.end;
            let ab = b.subtract(&a);

            let denominator = d.cross(&ab);
            if denominator.abs() < f32::EPSILON {
                // The ray is parallel to the line
                return None;
            }

            let t = (a.subtract(&p0)).cross(&ab) / denominator;
            let u = (a.subtract(&p0)).cross(&d) / denominator;

            if t >= 0.0 && u >= 0.0 && u <= 1.0 {
                let intersection_point = p0.add(&d.multiply(t));
                let distance = t * ray.direction.magnitude(); // Distance along the ray

                Some(Intersection {
                    distance,
                    position: intersection_point,
                    offset: u,
                })
            } else {
                None
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector2,
    pub direction: Vector2,
}

impl Ray {
    pub fn new(origin: Vector2, direction: Vector2) -> Self {
        Self { origin, direction }
    }

    pub fn translate(&self, v: &Vector2) -> Self {
        Self {
            origin: self.origin.add(v),
            direction: self.direction,
        }
    }

    pub fn rotate(&self, angle_rad: f32) -> Self {
        Self {
            origin: self.origin,
            direction: self.direction.rotate(angle_rad),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{geometry::*, *};

    #[test]
    fn test_rotate() {
        assert_eq!(
            Vector2::new(1.0, 0.0).rotate(0_f32.to_radians()),
            Vector2::new(1.0, 0.0)
        );

        assert_eq!(
            Vector2::new(1.0, 0.0).rotate(90_f32.to_radians()),
            Vector2::new(-4.371139e-8, 1.0)
        );

        assert_eq!(
            Vector2::new(1.0, 0.0).rotate(180_f32.to_radians()),
            Vector2::new(-1.0, -8.742278e-8)
        );

        assert_eq!(
            Vector2::new(1.0, 0.0).rotate(270_f32.to_radians()),
            Vector2::new(1.1924881e-8, -1.0)
        );

        assert_eq!(
            Vector2::new(1.0, 0.0).rotate(360_f32.to_radians()),
            Vector2::new(1.0, 1.7484555e-7)
        );

        assert_eq!(
            Vector2::new(1.0, 0.0).rotate(45_f32.to_radians()),
            Vector2::new(0.70710677, 0.7071068)
        );
    }

    #[test]
    fn test_magnitude() {
        let a = Vector2::new(3.0, 4.0);
        assert_eq!(a.magnitude(), 5.0);
    }

    #[test]
    fn test_intersection() {
        let ray = Ray {
            origin: Vector2::new(0.0, 0.0),
            direction: Vector2::new(1.0, 0.0),
        };

        // Test case where ray and line intersect
        let intersection = Line {
            start: Vector2::new(1.0, -1.0),
            end: Vector2::new(1.0, 1.0),
        }
        .cast(&ray);
        assert!(intersection.is_some());
        let Intersection {
            distance, position, ..
        } = intersection.unwrap();
        assert_eq!(distance, 1.0);
        assert_eq!(position.x, 1.0);
        assert_eq!(position.y, 0.0);

        let intersection = Line {
            start: Vector2::new(2.0, -1.0),
            end: Vector2::new(2.0, 1.0),
        }
        .cast(&ray);
        assert!(intersection.is_some());
        let Intersection {
            distance, position, ..
        } = intersection.unwrap();
        assert_eq!(distance, 2.0);
        assert_eq!(position.x, 2.0);
        assert_eq!(position.y, 0.0);

        // Test case where ray and line do not intersect
        let intersection = Line {
            start: Vector2::new(2.0, -1.0),
            end: Vector2::new(2.0, -2.0),
        }
        .cast(&ray);
        assert!(intersection.is_none());

        // Test case where ray and line are parallel
        let intersection = Line {
            start: Vector2::new(1.0, 0.0),
            end: Vector2::new(2.0, 0.0),
        }
        .cast(&ray);
        assert!(intersection.is_none());

        // Test case where ray and line are collinear
        let intersection = Line {
            start: Vector2::new(0.0, 0.0),
            end: Vector2::new(2.0, 0.0),
        }
        .cast(&ray);
        assert!(intersection.is_none());

        // Test case where ray and line are collinear and ray is in the opposite direction
        let ray = Ray {
            origin: Vector2::new(2.0, 0.0),
            direction: Vector2::new(-1.0, 0.0),
        };
        let intersection = Line {
            start: Vector2::new(0.0, 0.0),
            end: Vector2::new(2.0, 0.0),
        }
        .cast(&ray);
        assert!(intersection.is_none());

        let intersection = Line {
            start: Vector2::new(1.0, -1.0),
            end: Vector2::new(1.0, 2.0),
        }
        .cast(
            &Ray {
                origin: Vector2::new(0.0, 0.0),
                direction: Vector2::new(1.0, 0.0),
            }
            .rotate(45f32.to_radians()),
        );
        assert!(intersection.is_some());
        let Intersection {
            distance, position, ..
        } = intersection.unwrap();
        assert_eq!(distance, 1.4142137);
        assert_eq!(position.x, 1.0);
        assert_eq!(position.y, 1.0000001);
    }

    #[test]
    fn test_intersection_offset() {
        let ray = Ray {
            origin: Vector2::new(0.0, 0.0),
            direction: Vector2::new(1.0, 0.0),
        };

        let intersection = Line {
            start: Vector2::new(1.0, -1.0),
            end: Vector2::new(1.0, 1.0),
        }
        .cast(&ray);
        assert!(intersection.is_some());
        let Intersection { offset, .. } = intersection.unwrap();
        assert_eq!(offset, 0.5);
    }
}
