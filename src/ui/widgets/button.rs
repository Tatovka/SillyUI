use crate::ui::shapes::ShapeBuilder;
use crate::ui::theme::default_theme;

use super::*;
pub struct Button<M: Clone, S: Shape> {
    shape: S,

    pub main_style: Style,
    pub style_fn: Box<dyn WidgetStyle>,

    pub on_click: M,
}

impl<M: Clone + 'static, S: Shape + 'static> Button<M, S> {
    pub fn new(
        shape: S, 
        main_style: Style, 
        style_fn: Box<dyn WidgetStyle>,
        on_click: M) -> Self {
        Self { shape, main_style,style_fn, on_click: on_click.clone() }
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

use crate::opt_setters;
use crate::setters;

#[derive(Clone, Copy)]
pub struct ButtonBuilder<SB, WS = DarkenOnInteract> {
    shape: SB,

    pub main_style: Option<Style>,
    pub style_fn: WS,
}
impl<SB> ButtonBuilder<SB, DarkenOnInteract> {
    pub fn new(shape: SB) -> Self {
            ButtonBuilder {shape, main_style: None, style_fn: DarkenOnInteract::default()}
    }
}
impl<SB, WS> ButtonBuilder<SB, WS>  
where WS: WidgetStyle + 'static   
{
    setters! {
        shape: SB
    }

    opt_setters! {
        main_style: Style,
    }

    pub fn position<S: Shape>(mut self, p: Point) -> Self 
    where SB: ShapeBuilder<S> {
        self.shape = self.shape.set_position(p);
        self
    }

    pub fn style_fn<S: Shape, WS2>(self, f: WS2) -> ButtonBuilder<SB, WS2> 
    where SB: ShapeBuilder<S>, WS2: WidgetStyle {
        ButtonBuilder { shape: self.shape, main_style: self.main_style, style_fn: f}
    }

    pub fn build<S: Shape + 'static, M: Clone + 'static>(self, on_click: M) -> Button<M, S> 
    where SB: ShapeBuilder<S> {
        let style = self.main_style.unwrap_or(default_theme().surface());
        Button::new(self.shape.build(), style, Box::new(self.style_fn), on_click)
    }
}