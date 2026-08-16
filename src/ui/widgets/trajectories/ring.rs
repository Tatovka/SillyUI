use super::*;

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


#[derive(Clone, Copy)]
pub struct RingPath {
    shape: RingShapeBuilder
}

impl RingPath {
    pub fn from_builder(ring: RingShapeBuilder) -> Self {
        RingPath { shape: ring }
    }
}

impl Path<f32> for RingPath {
    type S = RingShape;
    type T = RingTrajectory;
    type SL = RingSlicer;

    fn slicer(&self) -> Self::SL {
        RingSlicer {}
    }

    fn shape(&self) -> Self::S {
        self.shape.build()
    }

    fn get_trajectory(&self) -> Self::T {
        let shape = self.shape();
        RingTrajectory {
            center: shape.center, 
            radius: shape.radius - shape.width / 2., 
            start_angle: shape.start_angle, 
            radians: shape.radians
        }
    }

    fn position(mut self, p: Point) -> Self {
        self.shape = self.shape.set_position(p);
        self
    }
}