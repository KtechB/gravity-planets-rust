use std::collections::VecDeque;

use super::vec3::Vec3;

const DELTA_T: f64 = 0.1;
const HISTORY_NUM: usize = 300;
pub struct Planet {
    pub m: f64,
    pub position: Vec3,
    pub velocity: Vec3,
    pub position_history: VecDeque<Vec3>,
}

impl Planet {
    pub fn new(m: f64, position: Vec3, velocity: Vec3) -> Self {
        if m <= 0.0 {
            panic!("m must be positive value.")
        }

        return Self {
            m,
            position,
            velocity,
            position_history: VecDeque::new(),
        };
    }
    pub fn next(&mut self, f: Vec3) {
        // add_history
        self.position_history.push_front(self.position);
        if self.position_history.len() > HISTORY_NUM {
            self.position_history.pop_back();
        }

        // update pos by currenct velocity
        self.position.x += self.velocity.x * DELTA_T;
        self.position.y += self.velocity.y * DELTA_T;
        self.position.z += self.velocity.z * DELTA_T;

        // update velocity
        self.velocity.x += (f.x / self.m) * DELTA_T;
        self.velocity.y += (f.y / self.m) * DELTA_T;
        self.velocity.z += (f.z / self.m) * DELTA_T;
    }
    pub fn calc_gravitation(&self, other_p: &Planet) -> Vec3 {
        const G: f64 = 9.8;
        let r = (&self.position - &other_p.position).length();
        let direction = (&other_p.position - &self.position) * (1.0 / r);
        let f = G * self.m * other_p.m / r.powi(2);
        return direction * f;
    }
}
