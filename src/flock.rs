use crate::boid::Boid;

pub struct Flock {
    boids: Vec<Boid>,

    protected_range_sqrd: f32,
    visual_range: f32,
    visual_range_sqrd: f32,

    avoid_factor: f32,
    matching_factor: f32,
    centering_factor: f32,
    turn_factor: f32,

    pub margins: Margins,

    max_speed: f32,
    min_speed: f32,
}

impl Flock {
    pub fn new(
        boids: Vec<Boid>,
        protected_range: f32,
        visual_range: f32,
        avoid_factor: f32,
        matching_factor: f32,
        centering_factor: f32,
        turn_factor: f32,
        margins: Margins,
        max_speed: f32,
        min_speed: f32,
    ) -> Self {
        Self {
            boids,
            protected_range_sqrd: protected_range * protected_range,
            visual_range,
            visual_range_sqrd: visual_range * visual_range,
            avoid_factor,
            matching_factor,
            centering_factor,
            turn_factor,
            margins,
            max_speed,
            min_speed,
        }
    }

    pub fn update(&mut self) {
        let mut states = Vec::with_capacity(self.boids.len());

        // Calculate the environment around the boid
        for boid in &self.boids {
            let mut state = State::new();
            for other_boid in &self.boids {
                let dx = boid.x - other_boid.x;
                let dy = boid.y - other_boid.y;

                if dx.abs() < self.visual_range && dy.abs() < self.visual_range {
                    let squared_dist = dx * dx + dy * dy;

                    if squared_dist < self.protected_range_sqrd {
                        if squared_dist > 0.0 {
                            let dist = squared_dist.sqrt();
                            state.close_dx += (dx / dist) / dist;
                            state.close_dy += (dy / dist) / dist;
                        } else {
                            state.close_dx += 0.1;
                            state.close_dy += 0.1;
                        }
                    } else if squared_dist < self.visual_range_sqrd {
                        state.x_pos_avg += other_boid.x;
                        state.y_pos_avg += other_boid.y;
                        state.x_vel_avg += other_boid.vx;
                        state.y_vel_avg += other_boid.vy;

                        state.neighboring_boids += 1;
                    }
                }
            }
            states.push(state);
        }

        // Update the boid's velocity and position
        for (boid, mut state) in &mut self.boids.iter_mut().zip(states) {
            if state.neighboring_boids > 0 {
                // Calculate averages
                state.x_pos_avg /= state.neighboring_boids as f32;
                state.y_pos_avg /= state.neighboring_boids as f32;
                state.x_vel_avg /= state.neighboring_boids as f32;
                state.y_vel_avg /= state.neighboring_boids as f32;

                boid.vx += (state.x_pos_avg - boid.x) * self.centering_factor
                    + (state.x_vel_avg - boid.vx) * self.matching_factor;

                boid.vy += (state.y_pos_avg - boid.y) * self.centering_factor
                    + (state.y_vel_avg - boid.vy) * self.matching_factor;
            }

            boid.vx += state.close_dx * self.avoid_factor;
            boid.vy += state.close_dy * self.avoid_factor;

            match self.margins.outside(boid.x, boid.y) {
                MarginBounds::Top => boid.vy += self.turn_factor,
                MarginBounds::Right => boid.vx -= self.turn_factor,
                MarginBounds::Bottom => boid.vy -= self.turn_factor,
                MarginBounds::Left => boid.vx += self.turn_factor,
                MarginBounds::NotOutside => (),
            }

            // Clamp the boid's speed
            let speed = (boid.vx * boid.vx + boid.vy * boid.vy).sqrt();
            if speed > 0.0 {
                if speed < self.min_speed {
                    boid.vx = boid.vx / speed * self.min_speed;
                    boid.vy = boid.vy / speed * self.min_speed;
                } else if speed > self.max_speed {
                    boid.vx = boid.vx / speed * self.max_speed;
                    boid.vy = boid.vy / speed * self.max_speed;
                }
            }

            // Move the boid
            boid.x += boid.vx;
            boid.y += boid.vy;
        }
    }

    pub fn boids(&self) -> &Vec<Boid> {
        &self.boids
    }
}

impl Default for Flock {
    fn default() -> Self {
        let boid_count = 800;
        let max_speed = 6.0;
        let mut boids = Vec::with_capacity(boid_count);
        for _ in 0..boid_count {
            boids.push(Boid::random(800.0, 600.0, max_speed));
        }

        Self {
            boids,
            protected_range_sqrd: 25.0 * 25.0,
            visual_range: 60.0,
            visual_range_sqrd: 60.0 * 60.0,
            avoid_factor: 0.05,
            matching_factor: 0.05,
            centering_factor: 0.00005,
            turn_factor: 0.2,
            margins: Margins::default(),
            max_speed,
            min_speed: 3.0,
        }
    }
}

pub struct Margins {
    pub width: f32,
    pub height: f32,
    pub inset: f32,
}

impl Margins {
    pub fn outside(&self, x: f32, y: f32) -> MarginBounds {
        if y < self.inset {
            MarginBounds::Top
        } else if x > self.width - self.inset {
            MarginBounds::Right
        } else if y > self.height - self.inset {
            MarginBounds::Bottom
        } else if x < self.inset {
            MarginBounds::Left
        } else {
            MarginBounds::NotOutside
        }
    }
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
            inset: 100.0,
        }
    }
}

pub enum MarginBounds {
    Top,
    Right,
    Bottom,
    Left,
    NotOutside,
}

pub struct State {
    x_pos_avg: f32,
    y_pos_avg: f32,

    x_vel_avg: f32,
    y_vel_avg: f32,

    close_dx: f32,
    close_dy: f32,

    neighboring_boids: u32,
}

impl State {
    pub fn new() -> Self {
        Self {
            x_pos_avg: 0.0,
            y_pos_avg: 0.0,
            x_vel_avg: 0.0,
            y_vel_avg: 0.0,
            close_dx: 0.0,
            close_dy: 0.0,
            neighboring_boids: 0,
        }
    }
}
