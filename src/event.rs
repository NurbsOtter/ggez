//! The `event` module contains traits and structs to actually run your game mainloop
//! and handle top-level state, as well as handle input events such as keyboard
//! and mouse.

use glutin;
use context::Context;
use GameResult;
use timer;

use std::time::Duration;

// This is an ugly hack - this should be done by fricking glutin, not us.
// But hey, thanks glutin, now we have to have an explicit dependency on winit
pub use winit::ModifiersState;

/// A trait defining event callbacks; your primary interface with
/// `ggez`'s event loop.  Have a type implement this trait and
/// override at least the update() and draw() methods, then pass it to
/// `event::run()` to run the game's mainloop.
///
/// The default event handlers do nothing, apart from
/// `key_down_event()`, which will by default exit the game if escape
/// is pressed.  Just override the methods you want to do things with.
pub trait EventHandler {
    /// Called upon each physics update to the game.
    /// This should be where the game's logic takes place.
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()>;

    /// Called to do the drawing of your game.
    /// You probably want to start this with
    /// `graphics::clear()` and end it with
    /// `graphics::present()` and `timer::sleep_until_next_frame()`
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;

    // fn mouse_button_down_event(&mut self, _button: mouse::MouseButton, _x: i32, _y: i32) {}

    // fn mouse_button_up_event(&mut self, _button: mouse::MouseButton, _x: i32, _y: i32) {}

    // fn mouse_motion_event(&mut self,
    //                       _state: mouse::MouseState,
    //                       _x: i32,
    //                       _y: i32,
    //                       _xrel: i32,
    //                       _yrel: i32) {
    // }

    // fn mouse_wheel_event(&mut self, _x: i32, _y: i32) {}

    fn key_down_event(&mut self, scan_code: glutin::ScanCode, virtual_key: Option<glutin::VirtualKeyCode>, modifiers: ModifiersState) {}

    fn key_up_event(&mut self, scan_code: glutin::ScanCode, virtual_key: Option<glutin::VirtualKeyCode>, modifiers: ModifiersState) {}

    // fn controller_button_down_event(&mut self, _btn: Button) {}
    // fn controller_button_up_event(&mut self, _btn: Button) {}
    // fn controller_axis_event(&mut self, _axis: Axis, _value: i16) {}

    // fn focus_event(&mut self, _gained: bool) {}

    /// Called upon a quit event.  If it returns true,
    /// the game does not exit.
    fn quit_event(&mut self) -> bool {
        println!("Quitting game");
        false
    }
}

/// Runs the game's main loop, calling event callbacks on the given state
/// object as events occur.
///
/// It does not try to do any type of framerate limiting.  See the
/// documentation for the `timer` module for more info.
pub fn run<S>(ctx: &mut Context, state: &mut S) -> GameResult<()>
    where S: EventHandler
{
    let mut continuing = true;
    while continuing {
        ctx.timer_context.tick();

        ctx.event_context.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::KeyboardInput(glutin::ElementState::Pressed, scan_code, virtual_key, modifiers) => {
                        state.key_down_event(scan_code, virtual_key, modifiers);
                    },
                    glutin::WindowEvent::KeyboardInput(glutin::ElementState::Released, scan_code, virtual_key, modifiers) => {
                        state.key_up_event(scan_code, virtual_key, modifiers);
                    },
                    glutin::WindowEvent::Closed => {
                        continuing = state.quit_event();
                    }
                    _ => (),
                },
            }
        });

        let dt = timer::get_delta(ctx);
        state.update(ctx, dt);
        state.draw(ctx);
    }
    // {
    //     let mut event_pump = ctx.sdl_context.event_pump()?;

    //     let mut continuing = true;
    //     while continuing {
    //         ctx.timer_context.tick();

    //         for event in event_pump.poll_iter() {
    //             match event {
    //                 Quit { .. } => {
    //                     continuing = state.quit_event();
    //                     // println!("Quit event: {:?}", t);
    //                 }
    //                 KeyDown {
    //                     keycode,
    //                     keymod,
    //                     repeat,
    //                     ..
    //                 } => {
    //                     if let Some(key) = keycode {
    //                         if key == keyboard::Keycode::Escape {
    //                             ctx.quit();
    //                         } else {
    //                             state.key_down_event(key, keymod, repeat)
    //                         }
    //                     }
    //                 }
    //                 KeyUp {
    //                     keycode,
    //                     keymod,
    //                     repeat,
    //                     ..
    //                 } => {
    //                     if let Some(key) = keycode {
    //                         state.key_up_event(key, keymod, repeat)
    //                     }
    //                 }
    //                 MouseButtonDown { mouse_btn, x, y, .. } => {
    //                     state.mouse_button_down_event(mouse_btn, x, y)
    //                 }
    //                 MouseButtonUp { mouse_btn, x, y, .. } => {
    //                     state.mouse_button_up_event(mouse_btn, x, y)
    //                 }
    //                 MouseMotion {
    //                     mousestate,
    //                     x,
    //                     y,
    //                     xrel,
    //                     yrel,
    //                     ..
    //                 } => state.mouse_motion_event(mousestate, x, y, xrel, yrel),
    //                 MouseWheel { x, y, .. } => state.mouse_wheel_event(x, y),
    //                 ControllerButtonDown { button, .. } => {
    //                     state.controller_button_down_event(button)
    //                 }
    //                 ControllerButtonUp { button, .. } => state.controller_button_up_event(button),
    //                 ControllerAxisMotion { axis, value, .. } => {
    //                     state.controller_axis_event(axis, value)
    //                 }
    //                 Window { win_event: event::WindowEvent::FocusGained, .. } => {
    //                     state.focus_event(true)
    //                 }
    //                 Window { win_event: event::WindowEvent::FocusLost, .. } => {
    //                     state.focus_event(false)
    //                 }
    //                 _ => {}
    //             }
    //         }

    //         let dt = timer::get_delta(ctx);
    //         state.update(ctx, dt)?;
    //         state.draw(ctx)?;
    //     }
    // }

    // Until a proper eventhandling is implemented, this will be our game loop :D
    Ok(())
}
