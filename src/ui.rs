use std::cell::RefCell;
use std::rc::Rc;

use raylib::prelude::*;

pub mod point;
pub mod shapes;
pub mod widgets;
pub mod raylib_launch;
pub mod theme;

use self::widgets::*;
use point::*;

pub type Shared<T> = Rc<RefCell<T>>;

pub fn shared<T>(val: T) -> Shared<T> {
    Rc::new(RefCell::new(val))
}

pub trait Hitbox {
    fn hit(&self, p: Point) -> bool;
}

pub trait Drawable {
    fn draw(&self, rl: &mut RaylibDrawHandle, state: WidgetState);
    fn get_color(&self, state: WidgetState) -> Color;
}

pub trait Shape {
    fn hit(&self, p: Point) -> bool;
    fn draw(&self, d: &mut RaylibDrawHandle, color: Color);
}

pub trait Movable {
    fn move_to(&mut self, p: Point);
    fn move_by(&mut self, v: Point);
}

fn moved_to<T: Clone + Movable>(obj: &T, p: Point) -> T {
    let mut res = obj.clone();
    res.move_to(p);
    res
}

fn moved_by<T: Clone + Movable>(obj: &T, p: Point) -> T {
    let mut res = obj.clone();
    res.move_by(p);
    res
}


pub struct Window {
    pub width: f32,
    pub height: f32,
}

pub struct GuiContext<M> {
    pub widgets: Vec<Shared<dyn Widget<M>>>,

    hovered: Option<usize>,
    pressed: Option<usize>,
    captured: Option<usize>,

    first_frame: bool,
}

#[derive(Clone, Copy)]
pub struct MouseState {
    pub pressed: bool,
    pub released: bool,
    pub moved: bool,
    pub pos: Point,
    pub delta: Point,
}


impl<M: Clone> GuiContext<M> {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            hovered: None,
            pressed: None,
            captured: None,
            first_frame: true
        }
    }

    pub fn first_frame(&mut self) -> bool{
        let res = self.first_frame;
        self.first_frame = false;
        res
    }
    pub fn add<W>(&mut self, widget: W) -> Shared<W>
    where
        W: Widget<M> + 'static,
    {
        let res = shared(widget);
        self.widgets.push(res.clone());
        res
    }
    
    pub fn set_cursor(&self, rl: &mut RaylibHandle) {
        let arrow = self
            .hovered
            .or(self.pressed)
            .or(self.captured)
            .and_then(|id| self.widgets[id].borrow().cursor_icon(self.widget_state(id)))
            .unwrap_or(MouseCursor::MOUSE_CURSOR_DEFAULT);
        rl.set_mouse_cursor(arrow);
    }

    pub fn draw_widgets(&mut self, rl: &mut RaylibDrawHandle) {
        for (i, widget) in self.widgets.iter().enumerate() {
            widget.borrow().draw(rl, self.widget_state(i));
        }
    }

    pub fn process_mouse(&mut self, state: MouseState, event_handler: &mut impl FnMut(M) -> ()) -> bool{
        let mouse_pos = state.pos;

        let mut was_action = false;

        if let Some(widget_id) = self.captured {
            self.update_captured(widget_id, state, event_handler);
            return true;
        }
        
        if state.moved {
            if let Some(widget_id) = self.hovered {
                let widget = self.widgets[widget_id].clone();
                if !widget.borrow().hit(mouse_pos) {
                    if let Some(event) = widget.borrow_mut().on_unhover() {
                        event_handler(event);
                    }
                    self.hovered = None;
                } else if let Some(event) =
                    widget.borrow_mut().on_pointer_move(state.delta, state.pos)
                {
                    event_handler(event);
                }
            }

            if self.hovered.is_none() {
                self.try_hover(event_handler, state);
            }
            was_action = true;
        }

        if state.pressed {
            was_action = true;
            if let Some(widget_id) = self.hovered {
                let widget = self.widgets[widget_id].clone();
                if widget.borrow_mut().follow_pointer(state.pos) {
                    self.captured = Some(widget_id);
                    if let Some(event) = widget.borrow_mut().on_unhover() {
                        event_handler(event);
                    }
                    self.hovered = None;
                    return true;
                }
                self.pressed = Some(widget_id);
                if let Some(event) = widget.borrow_mut().on_click(mouse_pos) {
                    event_handler(event);
                }
            }
        }

        if state.released {
            if let Some(pressed_id) = self.pressed {
                let pressed = self.widgets[pressed_id].clone();
                if let Some(event) = pressed
                    .borrow_mut()
                    .on_release(mouse_pos, Some(pressed_id) == self.hovered)
                {
                    event_handler(event);
                }
            }
            self.pressed = None;
            was_action = true;
        }
        was_action
    }

    fn widget_state(&self, id: usize) -> WidgetState {
        WidgetState {
            hovered: self.hovered == Some(id),
            pressed: self.pressed == Some(id),
            captured: self.captured == Some(id),
        }
    }

    fn update_captured(
        &mut self,
        widget_id: usize,
        state: MouseState,
        event_handler: &mut impl FnMut(M) -> (),
    ) {
        let widget = self.widgets[widget_id].clone();
        if state.released {
            if let Some(event) = widget.borrow_mut().on_release(state.pos, true) {
                event_handler(event);
            }
            self.captured = None;
            self.try_hover(event_handler, state);
        }

        if state.moved {
            if let Some(event) = widget.borrow_mut().on_drag(state.delta, state.pos) {
                event_handler(event);
            }
        }
    }

    fn try_hover(&mut self, event_handler: &mut impl FnMut(M) -> (), state: MouseState) {
        for (i, widget) in self.widgets.iter().enumerate() {
            if widget.borrow().hit(state.pos) {
                self.hovered = Some(i);
                if let Some(event) = widget.borrow_mut().on_hover(state.pos) {
                    event_handler(event);
                }
                break;
            }
        }
    }
}

pub fn to_screen_space(x: f32, y: f32, window: &Window) -> Point {
    Point::new(x * window.width, y * window.height)
}
