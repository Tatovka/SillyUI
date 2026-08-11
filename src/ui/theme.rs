use raylib::{prelude::Color};

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub tertiary_color: Color
}

impl Theme {
    pub fn track(&self) -> Color {
        self.tertiary_color
    } 

    pub fn surface(&self) -> Color {
        self.primary_color
    }

    pub fn slider_active(&self) -> Color {
        self.secondary_color
    }
}

pub fn default_theme() -> Theme {
    Theme { primary_color: Color::BLUE, secondary_color: Color::DARKGRAY, tertiary_color: Color::GRAY }
}