use crate::ui::shapes::rect_shape::{LinearSlicer, RectShape};
use crate::ui::shapes::ring_shape::{RingSlicer, RingShape, RingShapeBuilder};


use super::*;
use super::shapes::*;

pub mod paths;
pub mod linear;
pub mod ring;

use paths::*;

use std::f32::{self, consts::TAU};

pub trait Trajectory<V> {
    fn capture_val(&self, pos: Point) -> V;
    fn change_val(&self, delta: Point, pos: Point, old_val: V) -> V;
    fn change_pos(&self, val: V) -> Point;
}

pub trait Scale<V>: Copy {
    type U;
    fn to_unit(&self, val: V) -> Self::U;   
    fn from_unit(&self, t: Self::U) -> V;
    fn then<W>(self, other: impl Scale<W, U=V>) -> ComposedScale<Self, impl Scale<W, U = V>>{
        ComposedScale {
            f: self,
            g: other
        }
    }
}

#[derive(Clone, Copy)]
pub struct IntScale { pub min: i32, pub max: i32 }

impl Scale<i32> for IntScale {
    type U = f32;
    fn to_unit(&self, v: i32) -> f32 {
        (v - self.min) as f32 / (self.max - self.min) as f32
    }

    fn from_unit(&self, t: f32) -> i32 {
        self.min + (t * (self.max - self.min) as f32).round() as i32
    }
}

#[derive(Clone, Copy)]
pub struct TransformScale<UT, VT> {
    pub transform: UT,
    pub rev: VT,
} 

impl<U, V, UT, VT> Scale<V> for TransformScale<UT, VT>
where 
    UT: Fn(U) -> V + 'static + Copy,
    VT: Fn(V) -> U + 'static + Copy
{
    type U = U;
    fn to_unit(&self, v: V) -> Self::U {
        (self.rev)(v)
    }

    fn from_unit(&self, t: Self::U) -> V {
        (self.transform)(t)
    }
}

#[derive(Clone, Copy)]
pub struct ComposedScale<VS, WS> {
    pub f: VS,
    pub g: WS
} 

impl<V, W, VS, WS> Scale<W> for ComposedScale<VS, WS>
where 
    VS: Scale<V>,
    WS: Scale<W, U = V>
{
    type U = VS::U;
    fn to_unit(&self, v: W) -> Self::U {
        self.f.to_unit(self.g.to_unit(v))
    }

    fn from_unit(&self, t: Self::U) -> W {
        self.g.from_unit(self.f.from_unit(t))
    }
}


#[derive(Clone, Copy)]
pub struct ScaledTrajectory<T, Sc> {
    inner: T,
    scale: Sc,
}

impl<V, T: Trajectory<Sc::U>, Sc: Scale<V>> Trajectory<V> for ScaledTrajectory<T, Sc> {
    fn capture_val(&self, pos: Point) -> V {
        self.scale.from_unit(self.inner.capture_val(pos))
    }

    fn change_val(&self, delta: Point, pos: Point, old: V) -> V {
        let old_t = self.scale.to_unit(old);
        let new_t = self.inner.change_val(delta, pos, old_t);
        self.scale.from_unit(new_t)
    }

    fn change_pos(&self, val: V) -> Point {
        self.inner.change_pos(self.scale.to_unit(val))
    }
}

impl<T: Movable, Sc> Movable for ScaledTrajectory<T, Sc> {
    fn move_by(&mut self, v: Point) {
        self.inner.move_by(v);
    }
    fn move_to(&mut self, p: Point) {
        self.inner.move_to(p);
    }
}

#[derive(Clone, Copy)]
pub struct ScaledSlicer<SL, Sc> {
    inner: SL,
    scale: Sc,
}

impl<V, S: Shape, SL: ShapeSlicer<S, Sc::U>, Sc: Scale<V>> ShapeSlicer<S, V> for ScaledSlicer<SL, Sc> {
    fn shape_slice(&self, shape: &S , val: V) -> S {
        self.inner.shape_slice(shape, self.scale.to_unit(val))
    }
}

#[derive(Clone, Copy)]
pub struct ScaledPath<P, Sc> {
    inner: P,
    scale: Sc,
}

impl<V, P, Sc> Path<V> for ScaledPath<P, Sc>
where
    P: Path<Sc::U>,
    Sc: Scale<V>,
{
    type S = P::S;
    type T = ScaledTrajectory<P::T, Sc>;
    type SL = ScaledSlicer<P::SL, Sc>;

    fn shape(&self) -> Self::S {
        self.inner.shape()
    }
    fn get_trajectory(&self) -> Self::T {
        ScaledTrajectory { inner: self.inner.get_trajectory(), scale: self.scale }
    }
    fn slicer(&self) -> Self::SL {
        ScaledSlicer { inner: self.inner.slicer(), scale: self.scale }
    }
    fn position(self, p: Point) -> Self {
        ScaledPath { inner: self.inner.position(p), scale: self.scale }
    }
}