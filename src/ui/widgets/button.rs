use super::*;
pub struct Button<M: Clone, S: Shape> {
    shape: S,
    pub main_style: Style,

    pub on_click: M,
}

impl<M: Clone + 'static, S: Shape + 'static> Button<M, S> {
    pub fn new(shape: S, main_style: Style, on_click: M) -> Self {
        Self { shape, main_style, on_click: on_click.clone() }
    }
}

impl<M: Clone,  S: Shape> Hitbox for Button<M, S> {
    fn hit(&self, p: Point) -> bool {
        self.shape.hit(p)
    }
}

impl<M: Clone + 'static, S: Shape> Drawable for Button<M, S> {
    fn draw(&self, d: &mut RaylibDrawHandle, state: WidgetState) {
       self.shape.draw(d, DarkenOnInteract::default().style(self.main_style, state));
    }
}

impl<M: Clone + 'static, S: Shape> Widget<M> for Button<M, S> {
    fn on_release(&mut self, _: Point, inside: bool) -> Option<M>{
        if inside {
            Some(self.on_click.clone())
        } else { None }
    }

    fn cursor_icon(&self, _: WidgetState) -> Option<MouseCursor> {
        Some(MouseCursor::MOUSE_CURSOR_POINTING_HAND)
    }
}