use raylib::ffi::CSSPalette;

use super::*;

pub struct Checkbox<M: Clone, S: Shape, C: Shape> {
    pub box_shape: S,
    pub check_shape: C,   

    pub box_style: Style,

    pub check_color: Color,
    pub checked: bool,
    pub on_toggle: Box<dyn Fn(bool) -> M>,
}

impl<M: Clone + 'static, S: Shape, C: Shape> Checkbox<M, S, C> {
    pub fn new(
        box_shape: S,
        check_shape: C,
        box_style: Style,
        check_color: Color,
        checked: bool,
        on_toggle: Box<dyn Fn(bool) -> M>,
    ) -> Self {
        Self { box_shape, check_shape, box_style, check_color, checked, on_toggle }
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
        self.box_shape.draw(d, Style::new(Color::LIGHTGRAY).shadow(Color::new(0,0,0,50)));
        if self.checked {
            self.check_shape.draw(d, Style::new(self.check_color).shadow(Color::new(0,0,0,50)));
        }
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