use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2<S> {
    pub x: S,
    pub y: S,
}

impl<S: Clone + Copy> Vec2<S> {
    pub fn new(x: S, y: S) -> Self {
        Self { x, y }
    }

    pub fn from_scalar(x: S) -> Self {
        Self { x: x, y: x }
    }

    pub fn from_tuple((x, y): (S, S)) -> Self {
        Self { x, y }
    }
}

pub fn comp_mul<S, T>(lhs: Vec2<S>, rhs: Vec2<T>) -> Vec2<S>
where
    S: Mul<T, Output = S>,
{
    Vec2 {
        x: lhs.x * rhs.x,
        y: lhs.y * rhs.y,
    }
}

pub fn dot<S, T>(lhs: Vec2<S>, rhs: Vec2<T>) -> S
where
    S: Add<Output = S> + Mul<T, Output = S>,
{
    lhs.x * rhs.x + lhs.y * rhs.y
}

impl<S> Vec2<S>
where
    S: Add<Output = S> + Mul<Output = S> + Copy,
{
    pub fn sq(self) -> S {
        self.x * self.x + self.y * self.y
    }
}

impl<S: Mul<Output = S> + Copy> Mul<S> for Vec2<S> {
    type Output = Vec2<S>;

    fn mul(self, scalar: S) -> Self::Output {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<S: Div<Output = S> + Copy> Div<S> for Vec2<S> {
    type Output = Vec2<S>;

    fn div(self, scalar: S) -> Self::Output {
        Vec2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl<S: Sub<Output = S>> Sub<Vec2<S>> for Vec2<S> {
    type Output = Vec2<S>;

    fn sub(self, rhs: Vec2<S>) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<S: Add<Output = S>> Add<Vec2<S>> for Vec2<S> {
    type Output = Vec2<S>;

    fn add(self, rhs: Vec2<S>) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<S: Neg> Neg for Vec2<S> {
    type Output = Vec2<S::Output>;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub type Point = Vec2<f32>;

impl Point {
    pub fn scalar(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y
    }

    pub fn cross(a: Self, b: Self) -> f32 {
        a.x * b.y - a.y * b.x
    }

    pub fn from_angle(angle: f32) -> Point {
        Point {
            x: angle.cos(),
            y: angle.sin(),
        }
    }

    pub fn ortog(self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn rotate(self, angle: f32) -> Point {
        let (s, c) = angle.sin_cos();
        Point {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
        }
    }

    pub fn swap(self) -> Point {
        Point {
            x: self.y,
            y: self.x,
        }
    }

    pub fn angle(self) -> f32 {
        let res = self.y.atan2(self.x);
        res
    }
}

impl From<Point> for (f32, f32) {
    fn from(p: Point) -> Self {
        (p.x, p.y)
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Point { x, y }
    }
}

use raylib::prelude::Vector2;

impl From<Vector2> for Point {
    fn from(v: Vector2) -> Self {
        Point { x: v.x, y: v.y }
    }
}

impl From<Point> for Vector2 {
    fn from(v: Point) -> Self {
        Vector2 { x: v.x, y: v.y }
    }
}
