use crate::ui::shapes::circle_shape::CircleShapeBuilder;
use crate::ui::theme::default_theme;
use crate::ui::widgets::handler::Handler;
use crate::ui::widgets::trajectories::*;
use crate::ui::shapes::*;
use crate::opt_setters;
use trajectories::paths::*;

use super::*;
pub struct Slider<M, V, S1, S2, SL, T> 
where
M : Clone, 
V : Clone, 
S1: Shape + Movable, 
S2: Shape,
SL: ShapeSlicer<S2, V>,
T: Trajectory<V> {
    pub handler: Handler<M, V, S1, T>,

    pub track_shape: S2,
    pub track_slicer: SL,

    pub track_style: Style,
    pub active_style: Style,

    handler_hovered: bool
}

impl<M, V, S1, S2, SL, T> Slider<M, V, S1, S2, SL, T> 
where 
    M : Clone + 'static, 
    V : Clone + 'static, 
    S1: Shape + Movable,
    S2: Shape,
    SL: ShapeSlicer<S2, V>,
    T: Trajectory<V>{

        pub fn new(
            handler_shape: S1, 
            handler_style: Style,
            handler_style_fn: Box<dyn WidgetStyle>,
            trajectory: T,

            track_shape: S2, 
            track_slicer: SL,
            track_style: Style,
            active_style: Style,
            
            base_val: V,

            on_capture: Box<dyn Fn(V) -> M>,
            on_drag: Box<dyn Fn(V) -> M>,
            on_release: Box<dyn Fn(V) -> M>,
        ) -> Self {
            let handler = Handler::new(
                handler_shape,
                handler_style,
                handler_style_fn,
                on_capture,
                on_release,
                on_drag,
                base_val.clone(),
                trajectory
            );

            let mut res = Slider { 
                handler,
                track_shape,
                track_slicer,
                track_style,
                active_style,
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

impl<M, V, S1, S2, SL, T> Movable for Slider<M, V, S1, S2, SL, T> 
where
    M : Clone, 
    V : Clone, 
    S1: Shape + Movable, 
    S2: Shape + Movable,
    SL: ShapeSlicer<S2, V>,
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

impl<M, V, S1, S2, SL, T> Hitbox for Slider<M, V, S1, S2, SL, T> 
where 
    M : Clone, 
    V : Clone, 
    S1: Shape + Movable,
    S2: Shape,
    SL: ShapeSlicer<S2, V>,
    T: Trajectory<V> + Copy
{
    fn hit(&self, p: Point) -> bool {
        self.handler.hit(p) || self.track_shape.hit(p)
    }
}

impl<M, V, S1, S2, SL, T> Drawable for Slider<M, V, S1, S2, SL, T> 
where 
    M : Clone + 'static, 
    V : Clone, 
    S1: Shape + Movable,
    S2: Shape,
    SL: ShapeSlicer<S2, V>,
    T: Trajectory<V> + Copy
{
    fn draw(&self, d: &mut RaylibDrawHandle, state: WidgetState) {
        let mut handler_state = state;

        handler_state.hovered &= self.handler_hovered;

        self.track_shape.draw(d, self.track_style);
        self.track_slicer.shape_slice(&self.track_shape, self.handler.val.clone()).draw(d, self.active_style);
        self.handler.draw(d, handler_state);
    }
}

impl<M, V, S1, S2, SL, T> Widget<M> for Slider<M, V, S1, S2, SL, T> 
where 
    M : Clone + 'static, 
    V : Clone, 
    S1: Shape + Movable,
    S2: Shape,
    SL: ShapeSlicer<S2, V>,
    T: Trajectory<V> + Copy
{
    
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

#[derive(Copy, Clone)]
pub struct SliderBuilder<V, P, HB = CircleShapeBuilder, WS = DarkenOnInteract>
where 
    V : Copy, 
    P: Path<V>
{
    handler_shape: HB,
    handler_style: Option<Style>,
    handler_style_fn: WS,
    path: P,

    track_style: Option<Style>,
    active_style: Option<Style>,

    value: Option<V>,
}

impl<V, P> SliderBuilder<V, P, CircleShapeBuilder, DarkenOnInteract>
where 
    V : Copy + 'static,
    P: Path<V> 
{
    pub fn from_path(path: P) -> Self {
        Self {
            path,
            handler_shape: CircleShapeBuilder::default(),
            handler_style: None,
            track_style: None,
            active_style: None,
            handler_style_fn: DarkenOnInteract::default(),
            value: None,
        }
    }
}

impl<V, P, HB, WS> SliderBuilder<V, P, HB, WS>
where 
    V : Copy + Default + 'static, 
    WS: WidgetStyle + 'static,
    P: Path<V>
{

    opt_setters! {
        handler_style: Style,
        track_style: Style,
        active_style: Style,
        value: V
    }

    pub fn handler_style_fn<WS2> (self, style_fn: WS2) -> SliderBuilder<V, P, HB, WS2> {
        SliderBuilder {
            path: self.path,
            handler_shape: self.handler_shape,
            handler_style: self.handler_style,
            track_style: self.track_style,
            active_style: self.active_style,
            handler_style_fn: style_fn,
            value: self.value,
        }
    }
    
    pub fn handler<HB2> (self, handler: HB2) -> SliderBuilder<V, P, HB2, WS> {
        SliderBuilder {
            path: self.path,
            handler_shape: handler,
            handler_style: self.handler_style,
            track_style: self.track_style,
            active_style: self.active_style,
            handler_style_fn: self.handler_style_fn,
            value: self.value,
        }
    }

    pub fn position(mut self, p: Point) -> Self {
        self.path = self.path.position(p);
        self
    }

    pub fn build<HS, M: Copy + 'static>(self, f: impl Fn(V) -> M + Clone + 'static) -> Slider<M, V, HS, P::S, P::SL, P::T>
    where
    HS: Shape + Movable, 
    HB: ShapeBuilder<HS> {
        let value = self.value.unwrap_or_default();
        let handler_shape = self.handler_shape.build();

        let default_theme = default_theme();

        Slider::new(
            handler_shape, 
            self.handler_style.unwrap_or(default_theme.surface()).shadow(Color::new(0,0,0,50)), 
            Box::new(self.handler_style_fn),
            self.path.get_trajectory(), 
            self.path.shape(),
            self.path.slicer(),
            self.track_style.unwrap_or(default_theme.surface()), 
            self.active_style.unwrap_or(default_theme.surface()),
            value,
            Box::new(f.clone()),
            Box::new(f.clone()), 
            Box::new(f),
        )
    }
}
