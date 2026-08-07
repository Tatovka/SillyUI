use super::*;

pub mod button;
pub mod checkbox;
pub mod handler;
pub mod slider;
pub mod trajectories;

pub trait Widget<M>: Hitbox + Drawable {
    fn on_hover(&mut self, _: Point) -> Option<M> {
        None
    }
    fn on_unhover(&mut self) -> Option<M> {
        None
    }
    fn on_pointer_move(&mut self, _: Point, _: Point) -> Option<M> {
        None
    }

    fn on_click(&mut self, _: Point) -> Option<M> {
        None
    }
    fn on_release(&mut self, _: Point, _: bool) -> Option<M> {
        None
    }

    fn follow_pointer(&mut self, _: Point) -> bool {
        false
    }
    fn on_drag(&mut self, _: Point, _: Point) -> Option<M> {
        None
    }

    fn cursor_icon(&self, _: WidgetState) -> Option<MouseCursor> {
        None
    }
}

#[derive(Clone, Copy)]
pub struct WidgetState {
    pub hovered: bool,
    pub pressed: bool,
    pub captured: bool,
}
