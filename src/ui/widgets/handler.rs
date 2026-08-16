use super::*;
use super::trajectories::*;

pub struct Handler<M : Clone, V : Clone, S: Shape + Movable, T: Trajectory<V>> {
    pub shape: S,

    pub main_style: Style,
    pub style_fn: Box<dyn WidgetStyle>,

    pub on_capture: Box<dyn Fn(V) -> M>,
    pub on_drag: Box<dyn Fn(V) -> M>,
    pub on_release: Box<dyn Fn(V) -> M>,

    pub val: V,
    pub trajectory: T,
}


impl<M: Clone + 'static, V: Clone + 'static, S: Shape + Movable, T: Trajectory<V>> Handler<M, V, S, T> {
    pub fn new(
        shape: S, main_style: Style, style_change: Box<dyn WidgetStyle>,
        on_capture: Box<dyn Fn(V) -> M>, on_release: Box<dyn Fn(V) -> M>, on_drag: Box<dyn Fn(V) -> M>,
        base_val: V, trajectory: T,
    ) -> Self {
        Self { shape, main_style, style_fn: style_change, 
            on_capture, on_release, on_drag,
            val: base_val.clone(), trajectory,
        }
    }
}

impl<M: Clone, V: Clone, S: Shape + Movable, T: Trajectory<V>> Hitbox for Handler<M, V, S, T> {
    fn hit(&self, p: Point) -> bool {
        self.shape.hit(p)
    }
}

impl<M: Clone + 'static, V: Clone, S: Shape + Movable, T: Trajectory<V>> Drawable for Handler<M, V, S, T> {
    fn draw(&self, d: &mut RaylibDrawHandle, state: WidgetState) {
        self.shape.draw(d, self.style_fn.style(self.main_style, state));
    }
}

impl<M: Clone + 'static, V: Clone, S: Shape + Movable, T: Trajectory<V>> Widget<M> for Handler<M, V, S, T> {
    fn on_click(&mut self, _: Point) -> Option<M> {
        Some((self.on_capture)(self.val.clone()))
    }

    fn on_release(&mut self, _: Point, inside: bool) -> Option<M>{
        if inside {
            Some((self.on_release)(self.val.clone()))
        } else { None }
    }

    fn follow_pointer(&mut self, pos: Point) -> bool {
        self.val = self.trajectory.capture_val(pos);
        let p = self.trajectory.change_pos(self.val.clone());
        self.shape.move_to(p);
        true
    }

    fn on_drag(&mut self, delta: Point, pos: Point) -> Option<M> {
        self.val = self.trajectory.change_val(delta, pos, self.val.clone());
        let p = self.trajectory.change_pos(self.val.clone());
        self.shape.move_to(p);       
        Some((self.on_drag)(self.val.clone()))
    }
}