use super::*;

pub struct Checkbox<M: Clone, S: Shape, C: Shape> {
    pub box_shape: S,
    pub check_shape: C,   
    pub box_color: Color,
    pub check_color: Color,
    pub checked: bool,
    pub on_toggle: Box<dyn Fn(bool) -> M>,
}

impl<M: Clone + 'static, S: Shape, C: Shape> Checkbox<M, S, C> {
    pub fn new(
        box_shape: S,
        check_shape: C,
        box_color: Color,
        check_color: Color,
        checked: bool,
        on_toggle: Box<dyn Fn(bool) -> M>,
    ) -> Self {
        Self { box_shape, check_shape, box_color, check_color, checked, on_toggle }
    }

    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, val: bool) {
        self.checked = val;
    }
}

impl<M: Clone, S: Shape, C: Shape> Hitbox for Checkbox<M, S, C> {
    fn hit(&self, p: Point) -> bool {
        self.box_shape.hit(p)
    }
}

impl<M: Clone + 'static, S: Shape, C: Shape> Drawable for Checkbox<M, S, C> {
    fn draw(&self, d: &mut RaylibDrawHandle, state: WidgetState) {
        self.box_shape.draw(d, self.get_color(state));
        if self.checked {
            self.check_shape.draw(d, self.check_color);
        }
    }

    fn get_color(&self, state: WidgetState) -> Color {
        if state.pressed && state.hovered {
            return self.box_color.brightness(-0.5);
        }
        if state.pressed || state.hovered {
            return self.box_color.brightness(-0.3);
        }
        self.box_color
    }
}

impl<M: Clone + 'static, S: Shape, C: Shape> Widget<M> for Checkbox<M, S, C> {
    fn on_release(&mut self, _: Point, inside: bool) -> Option<M> {
        if inside {
            self.checked = !self.checked;
            Some((self.on_toggle)(self.checked))
        } else {
            None
        }
    }

    fn cursor_icon(&self, _: WidgetState) -> Option<MouseCursor> {
        Some(MouseCursor::MOUSE_CURSOR_POINTING_HAND)
    }
}