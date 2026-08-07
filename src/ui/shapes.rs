
use std::f32::consts::{PI, TAU};

use super::*;
use super::trajectories::*;

#[derive(Clone, Copy)]
pub struct CircleShape {
    pub center: Point,
    pub radius: f32
}

impl CircleShape {
    pub fn new(center: Point, r: f32) -> Self { CircleShape { center, radius: r }}
}

impl Shape for CircleShape {
    fn hit(&self, p: Point) -> bool {
        (p - self.center).sq() <= self.radius * self.radius
    }

    fn draw(&self, d: &mut RaylibDrawHandle, color: Color) {
        d.draw_circle_v(self.center, self.radius, color);
    }
}

impl Movable for CircleShape {
    fn move_by(&mut self, p: Point) {
        self.center = self.center + p;
    }

    fn move_to(&mut self, p: Point) {
        self.center = p;
    }
}

#[derive(Clone, Copy)]
pub struct RectShape {
    pub center: Point,
    pub size: Point,
    pub angle: f32
}

impl RectShape {
    pub fn new(x: f32, y:f32, w: f32, h: f32, angle: f32) -> Self { RectShape {center: (x, y).into(), size: (w, h).into(), angle} }
    pub fn new_vec(origin: Point, size: Point, angle: f32) -> Self { RectShape {center: origin, size, angle} }
    
    pub fn from_top_left(origin: Point, size: Point, angle: f32) -> Self {
        let u = Point::from_angle(angle);

        let sft = dot(Vec2::new(u, u.ortog()), size);

        Self{center: origin + sft * 0.5, size: size.into(), angle}
    }
    
    fn axes(&self) -> Vec2<Point> {
        let cs: Point = Point::from_angle(self.angle);
        comp_mul(Vec2::new(cs, cs.ortog()), self.size / 2.0)
    }

    pub fn get_origin(&self) -> Point {
        self.center
    }
}

impl Shape for RectShape {
    fn hit(&self, p: Point) -> bool {
        let axes = self.axes();

        let w = p - self.center;

        let proj_u = dot(w, axes.x) / axes.x.sq();
        let proj_v = dot(w, axes.y) / axes.y.sq();

        proj_u.abs() <= 1.0 && proj_v.abs() <= 1.0
    }
    
    fn draw(&self, d: &mut RaylibDrawHandle, color: Color) {
        let rec = Rectangle::new(-self.size.x / 2.0, -self.size.y / 2.0, self.size.x, self.size.y);
        
        let mut d = d.rl_push_matrix();
        
        d.rl_translatef(self.center.x, self.center.y, 0.0);

        d.rl_rotatef(self.angle.to_degrees(), 0.0, 0.0, 1.0);

        d.draw_rectangle_rounded(rec, 0.5, 128, color);
    }
}

impl Movable for RectShape {
    fn move_by(&mut self, p: Point) {
        self.center = self.center + p;
    }

    fn move_to(&mut self, p: Point) {
        self.center = p;
    }
}

#[derive (Clone, Copy)]
pub struct Combined<H: Shape, D: Shape> {
    pub hitbox: H,
    pub drawable: D
}

impl<H, D> Shape for Combined<H, D>
where H: Shape, D: Shape {
    fn hit(&self, p: Point) -> bool {
        self.hitbox.hit(p)
    }

    fn draw(&self, d: &mut RaylibDrawHandle, color: Color) {
        self.drawable.draw(d, color);
    }
}

impl<H, D> Movable for Combined<H, D>
where H: Shape + Movable, D: Shape + Movable {
    fn move_by(&mut self, p: Point) {
        self.hitbox.move_by(p);
        self.drawable.move_by(p);
    }

    fn move_to(&mut self, dp: Point) {
        self.hitbox.move_to(dp);
        self.drawable.move_to(dp);
    }
}

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
    pub fn sector(center: Point, radius: f32, width: f32, start_angle: f32, radians: f32) -> Self { RingShape { center, radius, width, start_angle, radians} }
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

pub trait Path<V, T: Trajectory<V>>: Shape {
    fn start_pos(&self) -> Point;

    fn get_trajectory(&self) -> T;
}

impl Path<f32, RingTrajectory> for RingShape {
    fn start_pos(&self) -> Point {
        self.center + Point::from_angle(self.start_angle) * (self.radius - self.width / 2.) 
    }

    fn get_trajectory(&self) -> RingTrajectory {
        RingTrajectory {center: self.center, radius: self.radius - self.width / 2., start_angle: self.start_angle, radians: self.radians}
    }
}

impl Path<f32, LinearTrajectory> for RectShape {
    fn start_pos(&self) -> Point {
        let u = self.axes().x;
        self.center - u
    }

    fn get_trajectory(&self) -> LinearTrajectory {
        LinearTrajectory { angle: self.angle, start: self.center - self.axes().x, length: self.size.x }
    }
}