use std::marker::PhantomData;
use crate::ui::theme::default_theme;
use crate::ui::widgets::handler::Handler;
use crate::ui::widgets::trajectories::*;
use crate::ui::shapes::*;

use super::*;
pub struct Slider<M : Clone, V : Clone, S1: Shape + Movable, S2: Path<V, T>, T: Trajectory<V>> {
    pub handler: Handler<M, V, S1, T>,

    pub track_shape: S2,
    pub track_color: Color,
    pub active_color: Color,

    handler_hovered: bool
}

impl<M, V, S1, S2, T> Slider<M, V, S1, S2, T> 
where 
    M : Clone + 'static, 
    V : Clone + 'static, 
    S1: Shape + Movable,
    S2: Path<V, T>,
    T: Trajectory<V>{
        pub fn new(
            handler_shape: S1, 
            handler_color: Color,
            path: S2, 
            track_color: Color,
            active_color: Color,
            on_capture: Box<dyn Fn(V) -> M>,
            on_drag: Box<dyn Fn(V) -> M>,
            on_release: Box<dyn Fn(V) -> M>,
            base_val: V,
        ) -> Self {
            let handler = Handler::new(
                handler_shape,
                handler_color,
                on_capture,
                on_release,
                on_drag,
                base_val.clone(),
                path.get_trajectory()
            );

            let mut res = Slider { 
                handler,
                track_shape: path,
                track_color,
                active_color: active_color,
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

pub struct SliderBuilder<M, V, HB, TB, T, P>
where 
    M : Copy, 
    V : Copy, 
    T : Trajectory<V>,
    P : Path<V, T>,
    TB : ShapeBuilder<P>
{
    track: TB,
    handler_shape: HB,

    value: Option<V>,

    on_capture: Option<Box<dyn Fn(V) -> M>>,
    on_drag: Option<Box<dyn Fn(V) -> M>>,
    on_release: Option<Box<dyn Fn(V) -> M>>,

    handler_color: Option<Color>,
    track_color: Option<Color>,
    active_color: Option<Color>,

    _marker: std::marker::PhantomData<(T, P)>
}

impl<M, V, HB, TB, T, P> SliderBuilder<M, V, HB, TB, T, P>
where 
    M : Copy + 'static, 
    V : Copy + Default + 'static, 
    T : Trajectory<V>,
    P : Path<V, T>,
    TB : ShapeBuilder<P> + Copy,
    HB: Copy 
{

    pub fn from_shapes(track: TB, handler: HB) -> Self {
        Self {
            handler_shape: handler,
            track: track,
            handler_color: None,
            track_color: None,
            active_color: None,
            value: None,
            on_capture: None,
            on_drag: None,
            on_release: None,
            _marker: PhantomData
        }
    }

    pub fn as_template(&self) -> Self {
        let mut res = Self::from_shapes(self.track, self.handler_shape);
        res.handler_color = self.handler_color;
        res.track_color = self.track_color;
        res
    }

    pub fn handler_color(mut self, color: Color) -> Self {
        self.handler_color = Some(color);
        self
    }

    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = Some(color);
        self
    }

    pub fn active_color(mut self, color: Color) -> Self {
        self.active_color = Some(color);
        self
    }

    pub fn position(mut self, p: Point) -> Self {
        self.track = self.track.set_position(p);
        self
    }

    pub fn on_action(self, f: impl Fn(V) -> M + Clone + 'static) -> Self {
        let (a, b, c) = (f.clone(), f.clone(), f);
        self.on_capture(a).on_drag(b).on_release(c)
    }

    pub fn on_capture(mut self, f: impl Fn(V) -> M + 'static) -> Self {
        self.on_capture = Some(Box::new(f));
        self
    }

    pub fn on_drag(mut self, f: impl Fn(V) -> M + 'static) -> Self {
        self.on_drag = Some(Box::new(f));
        self
    }

    pub fn on_release(mut self, f: impl Fn(V) -> M + 'static) -> Self {
        self.on_release = Some(Box::new(f));
        self
    }

    pub fn value(mut self, v: V) -> Self {
        self.value = Some(v);
        self
    }

    pub fn build<S>(self) -> Slider<M, V, S, P, T>
    where
        S: Shape + Movable, 
        HB: ShapeBuilder<S>, 
        TB: ShapeBuilder<P> {

        let on_capture = self.on_capture.expect("SliderBuilder: on_capture is required");
        let on_drag = self.on_drag.expect("SliderBuilder: on_drag is required");
        let on_release = self.on_release.expect("SliderBuilder: on_release is required");

        let path = self.track.build();
        let handler_shape = self.handler_shape.set_position(path.start_pos()).build();

        let default_theme = default_theme();

        Slider::new(
            handler_shape, 
            self.handler_color.unwrap_or(default_theme.surface()), 
            path, 
            self.track_color.unwrap_or(default_theme.track()), 
            self.active_color.unwrap_or(default_theme.slider_active()),
            on_capture, 
            on_drag, 
            on_release, 
            self.value.unwrap_or_default()
        )
    }
}

impl<M, V, S1, S2, T> Movable for Slider<M, V, S1, S2, T> 
where 
    M : Clone + 'static, 
    V : Clone + 'static, 
    S1: Shape + Movable,
    S2: Path<V, T> + Movable,
    T: Trajectory<V> + Movable 
{
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
    S2: Path<V, T>,
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
    S2: Path<V, T>,
    T: Trajectory<V> + Copy {

    fn draw(&self, d: &mut RaylibDrawHandle, state: WidgetState) {
        let mut handler_state = state;

        handler_state.hovered &= self.handler_hovered;

        self.track_shape.draw(d, self.track_color);
        self.track_shape.slice_to(self.handler.val.clone()).draw(d, self.active_color);
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
    S2: Path<V, T>,
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

