use raylib::{prelude::Color};
use crate::ui::widgets::WidgetStyle;
use crate::ui::widgets::DarkenOnInteract;

use super::Style;
#[derive(Clone, Copy)]
pub struct Theme<W: WidgetStyle> {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub tertiary_color: Color,
    pub style_fn: W
}

impl<W: WidgetStyle> Theme<W> {
    pub fn surface(&self) -> Style {
        Style::new(self.primary_color)
    }
}

pub fn default_theme() -> Theme<DarkenOnInteract> {
    Theme { 
        primary_color: Color::BLUE, 
        secondary_color: Color::BLUE, 
        tertiary_color: Color::DARKBLUE,
        style_fn: DarkenOnInteract::default()
     }
}