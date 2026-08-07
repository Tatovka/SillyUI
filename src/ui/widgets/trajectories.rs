use super::*;
use std::f32::{self, consts::TAU};

pub trait Trajectory<V> {
    fn capture_val(&self, pos: Point) -> V;
    fn change_val(&self, delta: Point, pos: Point, old_val: V) -> V;
    fn change_pos(&self, val: V) -> Point;
}

#[derive(Copy, Clone)]
pub struct LinearTrajectory {
    pub angle: f32,
    pub start: Point,
    pub length: f32,
}

impl Trajectory<f32> for LinearTrajectory {
    fn capture_val(&self, pos: Point) -> f32 {
        self.change_val(Point::new(0.0, 0.0), pos, 0.0)
    }

    fn change_val(&self, _: Point, pos: Point, _: f32) -> f32 {
        let proj = dot(Point::from_angle(self.angle), pos - self.start);
        (proj / self.length).clamp(0.0, 1.0)
    }

    fn change_pos(&self, val: f32) -> Point {
        self.start + Point::from_angle(self.angle) * self.length * val
    }
}

impl Movable for LinearTrajectory {
    fn move_by(&mut self, v: Point) {
        self.start = self.start + v;
    }
    fn move_to(&mut self, p: Point) {
        self.start = p;
    }
}

#[derive(Copy, Clone)]
pub struct RingTrajectory {
    pub center: Point,
    pub radius: f32,
    pub start_angle: f32,
    pub radians: f32,
}

impl Trajectory<f32> for RingTrajectory {
    fn capture_val(&self, pos: Point) -> f32 {
        self.change_val(Point::new(0.0, 0.0), pos, 0.0)
    }

    fn change_val(&self, _: Point, pos: Point, prev_val: f32) -> f32 {
        let local_angle = ((pos - self.center).angle() - self.start_angle).rem_euclid(TAU);

        if local_angle <= self.radians {
            (local_angle / self.radians).clamp(0.0, 1.0)
        } else {
            prev_val
        }
    }

    fn change_pos(&self, val: f32) -> Point {
        self.center + Point::from_angle(self.start_angle + val * self.radians) * self.radius
    }
}

impl Movable for RingTrajectory {
    fn move_by(&mut self, v: Point) {
        self.center = self.center + v;
    }
    fn move_to(&mut self, p: Point) {
        self.center = p;
    }
}
