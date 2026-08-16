use super::*;

pub trait Path<V>: Copy {
    type S: Shape;
    type T: Trajectory<V>;
    type SL: ShapeSlicer<Self::S, V>;

    fn get_trajectory(&self) -> Self::T;

    fn slicer(&self) -> Self::SL;

    fn shape(&self) -> Self::S;

    fn position(self, p: Point) -> Self;

    fn pad_hitbox(self, extra: f32) -> PaddedPath<Self> {
        PaddedPath::padded(self, extra)
    }

    fn scaled<Sc>(self, scale: Sc) -> ScaledPath<Self, Sc> {
        ScaledPath { inner: self, scale }
    }
}

#[derive(Clone, Copy)]
pub struct PaddedPath<P> {
    inner: P,
    hit_padding: f32,
}

impl<P> PaddedPath<P> {
    pub fn new(inner: P) -> Self {
        PaddedPath { inner, hit_padding: 0. }
    }

    pub fn padded(inner: P, extra: f32) -> Self {
        PaddedPath { inner, hit_padding: extra }
    }
}

impl<P> PaddedPath<P> {
    pub fn padding(mut self, extra: f32) -> Self {
        self.hit_padding = extra;
        self
    }
}

impl<V: Clone, P: Path<V>> Path<V> for PaddedPath<P>
where P::S: HitboxPadding {
    type S = Combined<P::S, P::S>;
    type T = P::T;
    type SL = CombinedSlicer<P::SL, P::SL>;

    fn shape(&self) -> Self::S {
        let drawable = self.inner.shape();
        let hitbox = drawable.padded(self.hit_padding);
        Combined { hitbox, drawable }
    }

    fn get_trajectory(&self) -> Self::T { self.inner.get_trajectory() }

    fn slicer(&self) -> Self::SL { CombinedSlicer{hitbox_slicer: self.inner.slicer(), drawable_slicer: self.inner.slicer()} }
    fn position(mut self, p: Point) -> Self {
        self.inner = self.inner.position(p);
        self
    }
}