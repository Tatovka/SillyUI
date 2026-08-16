use super::*;
use crate::shape_builder;

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

    pub fn axes(&self) -> Vec2<Point> {
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
    
    fn draw(&self, d: &mut RaylibDrawHandle, style: Style) {
        let rec = Rectangle::new(-self.size.x / 2.0, -self.size.y / 2.0, self.size.x, self.size.y);
        
        let mut d = d.rl_push_matrix();
        
        d.rl_translatef(self.center.x, self.center.y, 0.0);

        d.rl_rotatef(self.angle.to_degrees(), 0.0, 0.0, 1.0);

        if let Some(color) = style.shadow {
            let shadow_size = self.size + Point::from_scalar(3.5);
            let shadow_rect = Rectangle::new(-shadow_size.x / 2.0, -shadow_size.y / 2.0, shadow_size.x, shadow_size.y);
            
            d.draw_rectangle_rounded(shadow_rect, 0.5, 128, color);        
        }

        d.draw_rectangle_rounded(rec, 0.5, 128, style.color);

        if let Some((width, outline_color)) = style.outline {
            d.draw_rectangle_rounded_lines_ex(rec, 0.5, 128, width, outline_color);
        }
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

impl HitboxPadding for RectShape {
    fn padded(&self, extra: f32) -> Self {
        let mut pad_size = self.size;
        pad_size.y += 2. * extra;
        RectShape { center: self.center, size: pad_size, angle: self.angle }
    }
}


shape_builder! { 
    RectShapeBuilder for RectShape {
        center: Point = Point{x: 0., y: 0.},
        size: Point = Point{x: 15., y: 15.},
        angle: f32 = 0.0
    }
    position: center
    => RectShape::new_vec(center, size, angle)
}

impl RectShapeBuilder {
    pub fn from_top_left(origin: Point, size: Point, angle: f32) -> Self {
        let u = Point::from_angle(angle);

        let sft = dot(Vec2::new(u, u.ortog()), size);

        Self{center: Some(origin + sft * 0.5), size: Some(size.into()),angle: Some(angle)}
    }
}

pub struct LinearSlicer {}

impl ShapeSlicer<RectShape, f32> for LinearSlicer {
    fn shape_slice(&self, shape: &RectShape , val: f32) -> RectShape {
        let mut sz = shape.size;
        sz.x = val * shape.size.x;
        let pos = shape.center - shape.axes().sum();
        rect_shape::RectShapeBuilder::from_top_left(pos, sz, shape.angle).build()
    }
}
