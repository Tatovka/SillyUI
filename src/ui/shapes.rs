
use std::f32::consts::{PI, TAU};
use std::marker::PhantomData;

use super::*;

pub mod circle_shape;
pub mod rect_shape;
pub mod ring_shape;

pub trait HitboxPadding: Sized {
    fn padded(&self, extra: f32) -> Self;
}
pub trait ShapeBuilder<T> 
where Self: Sized {
    fn build(&self) -> T;
    fn set_position(self, p: Point) -> Self;
} 

pub trait ShapeSlicer<S: Shape, V> {
    fn shape_slice(&self, shape: &S , val: V) -> S;
}

#[macro_export]
macro_rules! shape_builder {
    (
        $name:ident for $target:ty {
            $( $field:ident : $ty:ty = $default:expr ),* $(,)?
        }
        position: $pos_field:ident
        => $build:expr
    ) => {
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name {
            $( $field: Option<$ty>, )*
        }

        impl $name {
            pub fn empty() -> Self { Self::default() }
            $(
                pub fn $field(mut self, v: $ty) -> Self {
                    self.$field = Some(v);
                    self
                }
            )*
        }

        impl ShapeBuilder<$target> for $name {
            fn build(&self) -> $target {
                $( let $field = self.$field.unwrap_or($default); )*
                $build
            }
            fn set_position(self, p: Point) -> Self {
                self.$pos_field(p)
            }
        }
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

    fn draw(&self, d: &mut RaylibDrawHandle, style: Style) {
        self.drawable.draw(d, style);
    }
}

impl<H, D> Movable for Combined<H, D>
where H: Shape + Movable, D: Shape + Movable{
    fn move_by(&mut self, p: Point) {
        self.hitbox.move_by(p);
        self.drawable.move_by(p);
    }

    fn move_to(&mut self, dp: Point) {
        self.hitbox.move_to(dp);
        self.drawable.move_to(dp);
    }
}

pub struct CombinedBuilder<HB, DB> {
    pub hitbox: HB,
    pub drawable: DB
}

impl<H, D, HB, DB> ShapeBuilder<Combined<H, D>> for CombinedBuilder<HB, DB> 
where
    H: Shape, 
    D: Shape, 
    HB: ShapeBuilder<H>, 
    DB: ShapeBuilder<D>
{
    fn set_position(mut self, p: Point) -> Self {
        self.hitbox = self.hitbox.set_position(p);
        self.drawable = self.drawable.set_position(p);
        self
    }

    fn build(&self) -> Combined<H, D> {
        Combined { hitbox: self.hitbox.build(), drawable: self.drawable.build() }
    }
}
pub struct CombinedSlicer<HS, DS> {
    pub hitbox_slicer: HS,
    pub drawable_slicer: DS
}

impl<H, D, HS, DS, V> ShapeSlicer<Combined<H, D>, V> for CombinedSlicer<HS, DS> 
where 
    V: Clone,
    H: Shape,
    D: Shape,
    HS: ShapeSlicer<H, V>,
    DS: ShapeSlicer<D, V>
{
    fn shape_slice(&self, shape: &Combined<H, D> , val: V) -> Combined<H, D>{
        Combined {hitbox: self.hitbox_slicer.shape_slice(&shape.hitbox, val.clone()), drawable: self.drawable_slicer.shape_slice(&shape.drawable, val)}
    }
}