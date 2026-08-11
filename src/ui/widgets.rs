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

pub trait WidgetStyle {
    fn style(&self, base_style: Style, state: WidgetState) -> Style;
}

pub struct DarkenOnInteract {
    pub hover_amount: f32,
    pub press_amount: f32,
}

impl Default for DarkenOnInteract {
    fn default() -> Self { Self { hover_amount: 0.3, press_amount: 0.5 } }
}

impl WidgetStyle for DarkenOnInteract {
    fn style(&self, base: Style, state: WidgetState) -> Style {
        let amount = if state.captured || state.pressed { self.press_amount }
                     else if state.hovered { self.hover_amount }
                     else { return base };
        base.color(base.color.brightness(-amount))
    }
}

#[derive(Clone, Copy)]
pub struct OutlineOnHover { pub color: Color, pub width: f32 }

impl WidgetStyle for OutlineOnHover {
    fn style(&self, base: Style, state: WidgetState) -> Style {
        if state.hovered { base.outline(self.width, self.color) } else { base }
    }
}

#[derive(Clone, Copy)]

pub struct StatefulOutline {
    pub hover_color: Color,
    pub press_color: Color,
    pub hover_width: f32,
    pub press_width: f32,
}

impl WidgetStyle for StatefulOutline {
    fn style(&self, mut base: Style, state: WidgetState) -> Style {
        base.outline = if state.captured || state.pressed {
            Some((self.press_width, self.press_color))
        } else if state.hovered {
           Some((self.hover_width, self.hover_color))
        } else { base.outline };

        base
    }
}

impl OutlineOnHover {
    pub fn new(color: Color) -> Self {
        Self { color, width: 2.0 }
    }
    pub fn color(mut self, c: Color) -> Self {
        self.color = c;
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
}

impl StatefulOutline {
    pub fn new(hover_color: Color, press_color: Color) -> Self {
        Self { hover_color, press_color, press_width: 2.0, hover_width: 2.0 }
    }

    pub fn hover_color(mut self, c: Color) -> Self {
        self.hover_color = c;
        self
    }
    pub fn press_color(mut self, c: Color) -> Self {
        self.press_color = c;
        self
    }

    pub fn press_width(mut self, w: f32) -> Self {
        self.press_width = w;
        self
    }

    pub fn hover_width(mut self, w: f32) -> Self {
        self.hover_width = w;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.press_width = w;
        self.hover_width = w;
        self
    }

}