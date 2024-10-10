use std::sync::Arc;

use crate::cursors::{SolitaireCursor, SolitaireCursors};
use crate::logic::SolitaireLogic;
use crate::window::Application;
use cfg_if::cfg_if;
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
use winit::event_loop::ActiveEventLoop;
use winit::{dpi::LogicalSize, event_loop::{ControlFlow, EventLoop}, window::Window};

mod graphics;
mod logic;
mod window;
mod cursors;


struct WindowState {
    handle: Arc<Window>,
    mouse_pos: glam::Vec2,
    active_pointer: SolitaireCursor,
    size: LogicalSize<u32>,
    cursors: SolitaireCursors
}

impl WindowState {
    fn new(window: Arc<Window>, event_loop: &ActiveEventLoop, size: LogicalSize<u32>) -> Self {
        let scale = window.scale_factor();
        let active_pointer = SolitaireCursor::Pointer;
        let cursors = SolitaireCursors::create(scale, event_loop);

        let mut result = Self {
            handle: window.clone(),
            cursors,
            mouse_pos: glam::Vec2::ZERO,
            active_pointer,
            size,
        };

        result.set_cursor(active_pointer);

        result
    }

    fn resize_cursors(&mut self, scale_factor: f64, event_loop: &ActiveEventLoop) {
        self.cursors.resize_cursors(scale_factor, event_loop);
        let cursor = self.cursors.get_cursor(self.active_pointer);
        self.handle.set_cursor(cursor);
    }

    fn set_cursor(&mut self, cursor: SolitaireCursor) {
        self.active_pointer = cursor;
        let cursor = self.cursors.get_cursor(self.active_pointer);
        self.handle.set_cursor(cursor);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if! {
        if #[cfg(target_arch="wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = Application::new(SolitaireLogic::new());

    let _ = event_loop.run_app(&mut app);
}

pub struct InputState {
    pub mouse_pos: glam::Vec2,
}

pub enum GameEvent {
    MouseMoved(glam::Vec2),
    MousePressed(glam::Vec2),
    MouseReleased(glam::Vec2),
}



