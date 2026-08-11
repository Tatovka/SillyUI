use super::*;
use crate::shape_builder;

#[derive(Clone, Copy)]
pub struct RingShape {
    pub center: Point,
    pub radius: f32,
    pub width: f32,
    pub start_angle: f32,
    pub radians: f32
}

impl RingShape {
    pub fn new(center: Point, radius: f32, width: f32) -> Self { RingShape { center, radius, width, start_angle: -PI, radians: TAU} }
    pub fn sector(center: Point, radius: f32, width: f32, start_angle: f32, radians: f32) -> Self { 
        RingShape { center, radius, width, start_angle, radians} 
    }
    
    pub fn inner_radius(&self) -> f32 {
        self.radius - self.width
    }
    pub fn end_angle(&self) -> f32 {
        self.start_angle + self.radians
    }
}

impl Shape for RingShape {
    fn hit(&self, p: Point) -> bool {
        let sq_len = (p - self.center).sq();
        let out_inner = self.inner_radius() * self.inner_radius() <= sq_len;
        let in_outer = sq_len <= self.radius * self.radius;
        let angle = ((p - self.center).angle() - self.start_angle).rem_euclid(TAU);
        let in_sector = angle >= 0.0 && angle <= self.radians;
        out_inner && in_outer && in_sector
    }

    fn draw(&self, d: &mut RaylibDrawHandle, color: Color) {
        let segments = 36;
        d.draw_ring(self.center, self.inner_radius(), self.radius, self.start_angle.to_degrees(), (self.start_angle + self.radians).to_degrees(), segments, color);
    }
}

impl Movable for RingShape {
    fn move_by(&mut self, p: Point) {
        self.center = self.center + p;
    }

    fn move_to(&mut self, p: Point) {
        self.center = p;
    }
}

impl HitboxPadding for RingShape {
    fn padded(&self, extra: f32) -> Self {
        let mut res = *self;
        res.width += 2. * extra;
        res.radius += extra;
        res
    }
}


impl Path<f32, RingTrajectory> for RingShape {
    fn start_pos(&self) -> Point {
        self.center + Point::from_angle(self.start_angle) * (self.radius - self.width / 2.) 
    }

    fn get_trajectory(&self) -> RingTrajectory {
        RingTrajectory {center: self.center, radius: self.radius - self.width / 2., start_angle: self.start_angle, radians: self.radians}
    }

    fn slice_to(&self, val: f32) -> impl Shape {
        RingShape::sector(self.center, self.radius, self.width, self.start_angle, val * self.radians)
    }
}

shape_builder! { 
    RingShapeBuilder for RingShape {
        center: Point = Point{x: 0., y: 0.},
        radius: f32 = 15.0,
        width: f32 = 15.0,
        start_angle: f32 = 0.0,
        radians: f32 = TAU
    }
    position: center
    => RingShape::sector(center, radius, width, start_angle, radians)
}