

use crate::ui::widgets::handler::Handler;
use crate::ui::widgets::trajectories::*;

use super::*;
pub struct Slider<M : Clone, V : Clone, S1: Shape + Movable, S2: Shape, T: Trajectory<V>> {
    pub handler: Handler<M, V, S1, T>,

    pub track_shape: S2,
    pub track_color: Color,

    handler_hovered: bool
}

impl<M, V, S1, S2, T> Slider<M, V, S1, S2, T> 
where 
    M : Clone + 'static, 
    V : Clone + 'static, 
    S1: Shape + Movable,
    S2: Shape,
    T: Trajectory<V> {
        pub fn new(
            handler_shape: S1, 
            handler_color: Color,
            track_shape: S2, 
            track_color: Color,
            on_capture: Box<dyn Fn(V) -> M>,
            on_drag: Box<dyn Fn(V) -> M>,
            on_release: Box<dyn Fn(V) -> M>,
            base_val: V,
            trajectory: T,
        ) -> Self {
            let handler = Handler::new(
                handler_shape,
                handler_color,
                on_capture,
                on_release,
                on_drag,
                base_val.clone(),
                trajectory
            );

            let mut res = Slider { 
                handler,
                track_shape,
                track_color,
                handler_hovered: false
            };
            res.set_val(base_val);
            res
        }

        pub fn set_val(&mut self, val: V) {
            self.handler.val = val.clone();
            let new_pos = self.handler.trajectory.change_pos(val);
            self.handler.shape.move_to(new_pos);
        }
}

impl<M, V, S1, S2, T> Slider<M, V, S1, S2, T> 
where 
    M : Clone + 'static, 
    V : Clone + 'static, 
    S1: Shape + Movable + Copy,
    S2: Shape + Copy,
    T: Trajectory<V> + Copy{
        pub fn from(
            slider: &Self, 
            on_capture: Box<dyn Fn(V) -> M>,
            on_drag: Box<dyn Fn(V) -> M>,
            on_release: Box<dyn Fn(V) -> M>,
            base_val: V,
        ) -> Self {
            let handler = Handler::new(
                slider.handler.shape,
                slider.handler.main_color,
                on_capture,
                on_release,
                on_drag,
                base_val.clone(),
                slider.handler.trajectory
            );

            let mut res = Slider { 
                handler,
                track_shape: slider.track_shape,
                track_color: slider.track_color,
                handler_hovered: false
            };

            res.set_val(base_val);
            res
        }
}

impl<M, V, S1, S2, T> Movable for Slider<M, V, S1, S2, T> 
where 
    M : Clone + 'static, 
    V : Clone + 'static, 
    S1: Shape + Movable,
    S2: Shape + Movable,
    T: Trajectory<V> + Movable {
        fn move_by(&mut self, v: Point) {
            self.handler.shape.move_by(v);
            self.handler.trajectory.move_by(v);
            self.track_shape.move_by(v);
        }
        
        fn move_to(&mut self, v: Point) {
            self.handler.shape.move_to(v);
            self.handler.trajectory.move_to(v);
            self.track_shape.move_to(v);
        }
}

impl<M, V, S1, S2, T> Hitbox for Slider<M, V, S1, S2, T> 
where 
    M : Clone, 
    V : Clone, 
    S1: Shape + Movable,
    S2: Shape,
    T: Trajectory<V> + Copy
{
    fn hit(&self, p: Point) -> bool {
        self.handler.hit(p) || self.track_shape.hit(p)
    }
}

impl<M, V, S1, S2, T> Drawable for Slider<M, V, S1, S2, T> 
where 
    M : Clone + 'static, 
    V : Clone, 
    S1: Shape + Movable,
    S2: Shape,
    T: Trajectory<V> + Copy {

    fn draw(&self, d: &mut RaylibDrawHandle, state: WidgetState) {
        let mut handler_state = state;

        handler_state.hovered &= self.handler_hovered;

        self.track_shape.draw(d, self.track_color);
        self.handler.draw(d, handler_state);
    }

    fn get_color(&self, _: WidgetState) -> Color {
        self.track_color
    }
}

impl<M, V, S1, S2, T> Widget<M> for Slider<M, V, S1, S2, T> 
where 
    M : Clone + 'static, 
    V : Clone, 
    S1: Shape + Movable,
    S2: Shape,
    T: Trajectory<V> + Copy {
    
    fn on_pointer_move(&mut self, _: Point, pos: Point) -> Option<M> {
        if self.handler_hovered {
            if !self.handler.hit(pos) {
                self.handler_hovered = false;
                return self.handler.on_unhover();
            }
            None
        } else {
            if self.handler.hit(pos) {
                self.handler_hovered = true;
                return self.handler.on_hover(pos);
            }
            None
        }
    }

    fn on_hover(&mut self, pos: Point) -> Option<M> {
        if self.handler.hit(pos) {
            self.handler_hovered = true;
            return self.handler.on_hover(pos);
        }
        None
    }

    fn on_unhover(&mut self) -> Option<M> {
        if self.handler_hovered {
            self.handler_hovered = false;
            return self.handler.on_unhover();
        }
        None
    }

    fn on_click(&mut self, pos: Point) -> Option<M> {
        self.handler.on_click(pos)
    }

    fn on_release(&mut self, pos: Point, inside: bool) -> Option<M>{
        if inside {
            self.handler.on_release(pos, inside)
        } else { None }
    }

    fn follow_pointer(&mut self, pos: Point) -> bool {
        self.handler.follow_pointer(pos)
    }

    fn on_drag(&mut self, delta: Point, pos: Point) -> Option<M> {
        self.handler.on_drag(delta, pos)
    }

    fn cursor_icon(&self, _: WidgetState) -> Option<MouseCursor> {
        Some(MouseCursor::MOUSE_CURSOR_POINTING_HAND)
    }
}

