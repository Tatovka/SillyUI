
use std::f32::consts::{PI, TAU};
use std::marker::PhantomData;

use super::*;
use super::trajectories::*;

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
pub trait Path<V, T: Trajectory<V>>: Shape {
    fn start_pos(&self) -> Point;

    fn get_trajectory(&self) -> T;

    fn slice_to(&self, val: V) -> impl Shape;
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

#[derive(Clone, Copy)]
pub struct FnShapeBuilder<U, V, F, UB> 
where
    U: Shape,
    V: Shape,
    UB: ShapeBuilder<U>,
    F: Copy + FnOnce(U) -> V 
{
    u_builder: UB,
    fun: F,
    _marker: std::marker::PhantomData<(U, V)>
}

impl<U, V, F, UB> ShapeBuilder<V> for FnShapeBuilder<U, V, F, UB> 
where
    U: Shape,
    V: Shape,
    UB: ShapeBuilder<U>,
    F: Copy + FnOnce(U) -> V + Copy 
{
    fn build(&self) -> V {
        let fun = self.fun;
        fun(self.u_builder.build())
    }

    fn set_position(mut self, p: Point) -> Self {
        self.u_builder = self.u_builder.set_position(p);
        self
    }
}

fn fn_builder<U, V, F, UB> (ub: UB, f: F) -> FnShapeBuilder<U, V, F, UB>
where 
    U: Shape,
    V: Shape,
    UB: ShapeBuilder<U>,
    F: Copy + FnOnce(U) -> V + Copy 
{
        FnShapeBuilder { u_builder: ub, fun: f, _marker: PhantomData }
}

pub fn padded<U, UB> (ub: UB, extra: f32) -> impl ShapeBuilder<Combined<U, U>> + Copy
where 
    U: Shape + HitboxPadding + Copy,
    UB: ShapeBuilder<U> + Copy, {
        fn_builder(ub, move |u: U| Combined{hitbox: u.padded(extra), drawable: u})
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

impl<V, T, H, D> Path<V, T> for Combined<H, D> 
where 
    T: Trajectory<V>, 
    H: Path<V, T>, 
    D: Path<V, T>
{
    fn start_pos(&self) -> Point {
        self.hitbox.start_pos()
    }
    fn get_trajectory(&self) -> T {
        self.hitbox.get_trajectory()
    }
    fn slice_to(&self, val: V) -> impl Shape {
        self.drawable.slice_to(val)
    }
}