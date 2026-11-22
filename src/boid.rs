use rand::random_range;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Boid {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Boid {
    pub fn random(max_x: f32, max_y: f32, max_speed: f32) -> Self {
        Self {
            x: random_range(0.0..max_x),
            y: random_range(0.0..max_y),
            vx: random_range(-max_speed..max_speed),
            vy: random_range(-max_speed..max_speed),
        }
    }
}
