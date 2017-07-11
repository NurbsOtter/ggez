//! The `event` module contains traits and structs to actually run your game mainloop
//! and handle top-level state, as well as handle input events such as keyboard
//! and mouse.

use glutin;
use context::Context;
use GameResult;
use timer;
use gilrs::Gilrs;
use gilrs::Event;
pub use gilrs::{Button,NativeEvCode,Axis};

use std::time::Duration;

use graphics;

// This is an ugly hack - this should be done by fricking glutin, not us.
// But hey, thanks glutin, now we have to have an explicit dependency on winit
pub use winit::ModifiersState;
pub use glutin::{MouseButton, ScanCode, TouchPhase, VirtualKeyCode};

/// A trait defining event callbacks; your primary interface with
/// `ggez`'s event loop.  Have a type implement this trait and
/// override at least the update() and draw() methods, then pass it to
/// `event::run()` to run the game's mainloop.
///
/// The default event handlers do nothing, apart from
/// `key_down_event()`, which will by default exit the game if escape
/// is pressed.  Just override the methods you want to do things with.
#[allow(unused_variables)]
pub trait EventHandler {
    /// Called upon each physics update to the game.
    /// This should be where the game's logic takes place.
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()>;

    /// Called to do the drawing of your game.
    /// You probably want to start this with
    /// `graphics::clear()` and end it with
    /// `graphics::present()` and `timer::sleep_until_next_frame()`
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
    
    fn mouse_button_down_event(&mut self, button: glutin::MouseButton, position: graphics::Point) {}

    fn mouse_button_up_event(&mut self, button: glutin::MouseButton, position: graphics::Point) {}

    fn mouse_motion_event(&mut self, position: graphics::Point) {}

    fn mouse_wheel_event(&mut self, mouse_scroll: glutin::MouseScrollDelta, touch_phase: glutin::TouchPhase) {}

    fn key_down_event(&mut self, scan_code: glutin::ScanCode, virtual_key: Option<glutin::VirtualKeyCode>, modifiers: ModifiersState) {}

    fn key_up_event(&mut self, scan_code: glutin::ScanCode, virtual_key: Option<glutin::VirtualKeyCode>, modifiers: ModifiersState) {}

    fn gamepad_button_down_event(&mut self,id:usize,button:Button,ev_code:NativeEvCode){}
    fn gamepad_button_up_event(&mut self,id:usize,button:Button,ev_code:NativeEvCode){}
    fn gamepad_axis_change_event(&mut self,id:usize,axis:Axis,value:f32,ev_code:NativeEvCode){}
    fn gamepad_disconnected_event(&mut self,id:usize){}
    fn gamepad_connected_event(&mut self,id:usize){}

    fn focus_event(&mut self, gained: bool) {}

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
    let mut running = true;
    let mut new_mouse_position : graphics::Point = graphics::Point::zero();
    while running {
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
                    glutin::WindowEvent::MouseMoved(x, y) => {
                        new_mouse_position = graphics::Point::new(x as f32, y as f32);
                        state.mouse_motion_event(new_mouse_position);
                    },
                    glutin::WindowEvent::MouseInput(glutin::ElementState::Pressed, mouse_button) => {
                        state.mouse_button_down_event(mouse_button, ctx.mouse_position);
                    },
                    glutin::WindowEvent::MouseInput(glutin::ElementState::Released, mouse_button) => {
                        state.mouse_button_up_event(mouse_button, ctx.mouse_position);
                    },
                    glutin::WindowEvent::MouseWheel(mouse_scroll, touch_phase) => {
                        state.mouse_wheel_event(mouse_scroll, touch_phase);
                    },
                    glutin::WindowEvent::Focused(focus) => {
                        state.focus_event(focus);
                    }
                    glutin::WindowEvent::Closed => {
                        running = state.quit_event();
                    }
                    _ => (),
                },
            }            
        });
        for event in ctx.gamepad_context.poll_events(){
                match event{
                    (id,Event::ButtonPressed(but,ev))=>{
                        state.gamepad_button_down_event(id,but,ev);
                    },
                    (id,Event::ButtonReleased(but,ev))=>{
                        state.gamepad_button_up_event(id,but,ev);
                    },
                    (id,Event::AxisChanged(axis,val,ev))=>{
                        state.gamepad_axis_change_event(id,axis,val,ev);
                    },
                    (id,Event::Disconnected)=>{
                        state.gamepad_disconnected_event(id);
                    },
                    (id,Event::Connected)=>{
                        state.gamepad_connected_event(id);
                    },
                    _=>(),
                }
        }
        ctx.mouse_position = new_mouse_position;
        

        let dt = timer::get_delta(ctx);
        let _ = state.update(ctx, dt);
        let _ = state.draw(ctx);

        if running {
            running = ctx.running;
        }
    }
    Ok(())
}
