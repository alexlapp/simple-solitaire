use crate::graphics::context::DrawContext;
use crate::graphics::WgpuState;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::*, window::Window
};

pub struct Application {
    window: Option<crate::WindowState>,
    wgpu: Option<WgpuState>,
    app_logic: crate::logic::SolitaireLogic,
}

impl<'a> Application {
    pub(crate) fn new(app_logic: crate::logic::SolitaireLogic) -> Self {
        Self {
            window: None,
            wgpu: None,
            app_logic,
        }
    }
}

impl<'a> ApplicationHandler<()> for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let initial_size: LogicalSize<u32> = LogicalSize::new(800, 600);

        let window_attributes = Window::default_attributes()
            .with_title("Hello World!")
            .with_visible(false)
            .with_inner_size(initial_size);
        let window = event_loop.create_window(window_attributes).unwrap();

        #[cfg(target_arch="wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on the web.
            use winit::dpi::PhysicalSize;

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas()?);
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");


            // window.request_inner_size(PhysicalSize::new(800, 600)).unwrap();
        }

        let window = Arc::new(window);

        self.window = Some(crate::WindowState::new(window.clone(), event_loop, initial_size));

        let scale = window.scale_factor();

        self.wgpu = Some(pollster::block_on(WgpuState::new(window.clone(), initial_size.to_physical(scale))));

        self.window.as_ref().unwrap().handle.set_visible(true);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let (Some(window), Some(wgpu_state)) = (&mut self.window, &mut self.wgpu) {
            if window_id == window.handle.id() {
                let game_event: Option<crate::GameEvent> = match event {
                    /*
                        CLOSE REQUESTED
                    */
                    WindowEvent::CloseRequested => {
                        event_loop.exit();
                        None
                    },
                    /*
                        HANDLE INPUT
                    */
                    WindowEvent::CursorMoved { position, .. } => {
                        let scale_factor = window.handle.scale_factor();
                        let logical_position = position.to_logical(scale_factor);
                        let mouse_pos = glam::vec2(logical_position.x - window.size.width as f32 / 2., logical_position.y);
                        window.mouse_pos = mouse_pos;

                        Some(crate::GameEvent::MouseMoved(mouse_pos))
                    },
                    WindowEvent::MouseInput {
                        state,
                        button: MouseButton::Left | MouseButton::Right,
                        ..
                    } => {
                        match state {
                            ElementState::Pressed => Some(crate::GameEvent::MousePressed(window.mouse_pos)),
                            ElementState::Released => Some(crate::GameEvent::MouseReleased(window.mouse_pos)),
                        }
                    },
                    /*
                        HANDLE REDRAWING AND RESIZING
                    */
                    WindowEvent::RedrawRequested => {
                        let mut draw_context = DrawContext::from_render_config(&wgpu_state.render_config);
                        let card_info = wgpu_state.render_config.create_card_info();
                        self.app_logic.render(&mut draw_context, &card_info);

                        match wgpu_state.render(&draw_context.card_instances, &draw_context.char_instances) {
                            Ok(_) => {},
                            Err(wgpu::SurfaceError::Lost) => wgpu_state.resize(wgpu_state.size, window.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                            Err(e) => eprintln!("{:?}", e),
                        }

                        None
                    },
                    WindowEvent::Resized(physical_size) => {
                        let scale_factor = window.handle.scale_factor();
                        window.size = physical_size.to_logical(scale_factor);

                        wgpu_state.resize(physical_size, window.size);
                        window.handle.set_title(&format!("Hello World! ({}x{})", window.size.width, window.size.height));
                        window.handle.request_redraw();

                        None
                    },
                    WindowEvent::ScaleFactorChanged { mut inner_size_writer, scale_factor } => {
                        // TODO: This causes a crash in wasm
                        // Removing it still crashes wasm, because the canvas size becomes unreasonable

                        let phys_size = window.size.to_physical(scale_factor);
                        let _ = inner_size_writer.request_inner_size(phys_size);

                        window.resize_cursors(scale_factor, event_loop);
                        // window.set_cursor(window.active_pointer);

                        None
                    },
                    _ => None
                };

                if let Some(game_event) = game_event {
                    // TODO: Find a clean way to get sizing info in update without a draw_context
                    let draw_context = DrawContext::from_render_config(&wgpu_state.render_config);
                    let card_info = wgpu_state.render_config.create_card_info();
                    let cursor = self.app_logic.update(game_event, &card_info, &draw_context);

                    if cursor != window.active_pointer { window.set_cursor(cursor); }

                    window.handle.request_redraw();
                }
            }
        }

        // if let (Some(window), Some(wgpu_state)) = (&self.window, &mut self.wgpu) {
        //     if window_id == window.handle.id() && !wgpu_state.input(&event, &mut request_redraw) {
        //         match event {
        //             WindowEvent::CloseRequested
        //             | WindowEvent::KeyboardInput {
        //                 event:
        //                     KeyEvent {
        //                         state: ElementState::Pressed,
        //                         physical_key: PhysicalKey::Code(KeyCode::Escape),
        //                         ..
        //                     },
        //                 ..
        //             } => event_loop.exit(),
        //             WindowEvent::KeyboardInput {
        //                 event: KeyEvent {
        //                     state: ElementState::Pressed,
        //                     physical_key: PhysicalKey::Code(code),
        //                     ..
        //                 },
        //                 ..
        //             } => {
        //                 match code {
        //                     KeyCode::Digit1 => { window.set_cursor(SolitaireCursor::Pointer); }
        //                     KeyCode::Digit2 => { window.set_cursor(SolitaireCursor::Grab); }
        //                     KeyCode::Digit3 => { window.set_cursor(SolitaireCursor::Grabbing); }
        //                     _ => {}
        //                 }
        //             }
        //             WindowEvent::Resized(physical_size) => {
        //                 wgpu_state.resize(physical_size);
        //                 window.handle.set_title(&format!("Hello World! ({}x{})", physical_size.width, physical_size.height));
        //                 request_redraw = true;
        //             },
        //             WindowEvent::ScaleFactorChanged { mut inner_size_writer, .. } => {
        //                 inner_size_writer.request_inner_size(PhysicalSize::new(800, 600)).unwrap();
        //             },
        //             WindowEvent::RedrawRequested => {
        //                 wgpu_state.update();
        //
        //                 match wgpu_state.render() {
        //                     Ok(_) => {},
        //                     Err(wgpu::SurfaceError::Lost) => wgpu_state.resize(wgpu_state.size),
        //                     Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
        //                     Err(e) => eprintln!("{:?}", e),
        //                 }
        //             },
        //             _ => {}
        //         }
        //     }
        //
        //     if event_redraw {
        //         window.handle.request_redraw();
        //     }
        // }
    }
}