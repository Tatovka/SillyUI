use super::*;
use crate::shape_builder;

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

impl HitboxPadding for CircleShape {
    fn padded(&self, extra: f32) -> Self {
        let mut res = *self;
        res.radius += extra;
        res
    }
}

shape_builder! { 
    CircleShapeBuilder for CircleShape {
        center: Point = Point{x: 0., y: 0.},
        radius: f32 = 15.0,
    }
    position: center
    => CircleShape::new(center, radius)
}