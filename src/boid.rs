use rand::random_range;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Boid {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Boid {
    pub fn random(x_max: f32, y_max: f32) -> Self {
        Self {
            x: random_range(0.0..x_max),
            y: random_range(0.0..y_max),
            vx: random_range(-2.0..2.0),
            vy: random_range(-2.0..2.0),
        }
    }
}
