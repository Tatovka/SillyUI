
use raylib::ffi::PollInputEvents;

use super::*;

pub struct RaylibConfig {
    rl: RaylibHandle,
    thread: RaylibThread
}

pub fn init(window: Window, title: &str) -> RaylibConfig {
    let (rl, thread) = raylib::init()
        .size(window.width as i32, window.height as i32)
        .title(title)
        .build();
    RaylibConfig { rl, thread }
}

pub fn frame<M: Clone>(
    RaylibConfig{rl, thread}: &mut RaylibConfig, 
    clear_color: Color,
    gui_context: &mut GuiContext<M>, 
    event_handler: &mut impl FnMut(M) -> (),
    redraw: bool
) -> bool {
    if rl.window_should_close() {
        return false;
    }

    let pos = rl.get_mouse_position();
    let delta = rl.get_mouse_delta();
    let mut draw = redraw || gui_context.first_frame();

    let mouse_state = MouseState {
        pressed: rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT),
        released: rl.is_mouse_button_released(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT),
        moved: delta != raylib::math::Vector2::zero(),
        delta: delta.into(),
        pos: pos.into()
    }; 

    draw |= gui_context.process_mouse(
        mouse_state,                  
        event_handler
    );

    gui_context.set_cursor(rl);
    rl.set_target_fps(120);

    if draw {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(clear_color);

        gui_context.draw_widgets(&mut d);
        d.draw_fps(10, 10);
    } else {
        unsafe {
            PollInputEvents();
        }
    }

    true
}