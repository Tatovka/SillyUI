use super::*;

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

#[derive(Clone, Copy)]
pub struct LinearPath {
    from: Point,
    to: Point,
    thickness: f32,
    roundness: f32,
}

impl LinearPath {
    pub fn new(length: f32) -> Self {
        Self { from: Point::from_scalar(0.0), to: Point::new(length, 0.), thickness: 4.0, roundness: 0.5 }
    }

    pub fn from(mut self, p: Point) -> Self { self.from = p; self }
    pub fn to(mut self, p: Point) -> Self { self.to = p; self }
    pub fn thickness(mut self, t: f32) -> Self { self.thickness = t; self }
    pub fn roundness(mut self, r: f32) -> Self { self.roundness = r; self }
    pub fn angle(mut self, angle: f32) -> Self { 
        let l = self.get_length(); 
        self.to = self.from + Point::from_angle(angle) * l;
        self
    }

    pub fn length(mut self, length: f32) -> Self { 
        let angle = self.get_angle();
        self.to = self.from + Point::from_angle(angle) * length;
        self
    }

    pub fn get_angle(&self) -> f32 { (self.to - self.from).angle() }
    pub fn get_length(&self) -> f32 { (self.to - self.from).sq().sqrt() }
    
}

impl Path<f32> for LinearPath {
    type S = RectShape;
    type T = LinearTrajectory;
    type SL = LinearSlicer;

    fn slicer(&self) -> Self::SL { LinearSlicer {} }

    fn shape(&self) -> Self::S {
        RectShape::from_top_left(
            self.from,
            Point::new(self.get_length(), self.thickness),
            self.get_angle(),
        )
    }

    fn get_trajectory(&self) -> Self::T {
        LinearTrajectory { angle: self.get_angle(), start: self.from, length: self.get_length() }
    }

    fn position(mut self, p: Point) -> Self {
        let delta = p - self.from;
        self.from = self.from + delta;
        self.to = self.to + delta;
        self
    }
}
