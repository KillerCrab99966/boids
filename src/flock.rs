use crate::boid::Boid;

pub struct Flock {
    boids: Vec<Boid>,

    protected_range: u32,
    visible_range: u32,

    avoid_factor: u32,
    matching_factor: u32,
    centering_factor: u32,
    turn_factor: u32,

    margins: Margins,

    max_speed: u32,
    min_speed: u32,
}

enum Margins {
    Top(u32),
    Right(u32),
    Bottom(u32),
    Left(u32),
}
