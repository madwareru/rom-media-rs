use super::{PixelWindowHandler, WindowParameters, Key, PixelWindowControlFlow};
use glutin::window::{WindowBuilder, Fullscreen};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::event::{Event, WindowEvent, KeyboardInput, ElementState, MouseButton};
use glutin::ContextBuilder;
use glutin::dpi::{Size, LogicalSize};
use std::time::Instant;
use glutin::monitor::VideoMode;

pub fn start_pixel_window<W: PixelWindowHandler>(window_params: WindowParameters) {
    let scale_up_x = window_params.scale_up.max(1);
    let mut scale_up_y = scale_up_x as f64;

    let actual_w = window_params.window_width * scale_up_x;
    let actual_h = window_params.window_height * scale_up_x;

    let event_loop = EventLoop::new();
    let mut exclusive_fullscreen = false;
    let window_builder = match window_params.fullscreen {
        true => {
            let primary_monitor = event_loop.primary_monitor();
            match primary_monitor {
                None => {WindowBuilder::new()
                    .with_title(window_params.title)
                    .with_resizable(false)
                    .with_inner_size(Size::Logical(
                        LogicalSize::new(
                            actual_w as f64,
                            actual_h as f64
                        )
                    ))
                }
                Some(monitor) => {
                    exclusive_fullscreen = true;
                    let video_mode = match monitor.video_modes().find(|mode|
                        mode.size().width == actual_w as u32 &&
                        mode.size().height == actual_h as u32
                    ) {
                        None => {
                            // in the case of non-found video mode for our expected resolution
                            // we are trying to find nearest video mode with exactly same width
                            // it would have pixels stretched in y dimension in this case, so
                            // we need to add some extra correction for our scale_up_y variable
                            match monitor.video_modes().find(|mode|
                                mode.size().width == actual_w as u32
                            ) {
                                Some(nearest_mode) => {
                                    scale_up_y *= nearest_mode.size().height as f64 / actual_h as f64;
                                    nearest_mode
                                }
                                None => panic!("failed to initialize fullscreen video with a {}x{} mode", actual_w, actual_h)
                            }
                        }
                        Some(mode) => mode
                    };

                    WindowBuilder::new()
                        .with_title(window_params.title)
                        .with_resizable(false)
                        .with_fullscreen(Some(Fullscreen::Exclusive(video_mode)))
                }
            }
        }
        false => WindowBuilder::new()
            .with_title(window_params.title)
            .with_resizable(false)
            .with_inner_size(Size::Logical(
                LogicalSize::new(
                    actual_w as f64,
                    actual_h as f64
                )
            ))
    };

    let windowed_context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let windowed_context = crate::graphics::init(windowed_context);
    let mut scale_factor = windowed_context.window().scale_factor();
    windowed_context.window().set_cursor_visible(window_params.cursor_visible);

    let mut win = W::create(&window_params);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::LoopDestroyed => {
                win.on_window_closed();
                return
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ScaleFactorChanged { scale_factor: factor, .. } => {
                    scale_factor = factor;
                },
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size)
                },
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                },
                WindowEvent::KeyboardInput {input, ..} => {
                    match input {
                        KeyboardInput { state, virtual_keycode, .. } => {
                            match (state, virtual_keycode) {
                                (ElementState::Pressed, Some(keycode)) => win.on_key_pressed(Key::map_from_keycode(keycode)),
                                (ElementState::Released, Some(keycode)) => win.on_key_released(Key::map_from_keycode(keycode)),
                                _ => ()
                            }
                        },
                    }
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    match state {
                        ElementState::Pressed => win.on_mouse_button_pressed(match button {
                            MouseButton::Left => 0,
                            MouseButton::Right => 2,
                            MouseButton::Middle => 1,
                            MouseButton::Other(id) => id
                        }),
                        ElementState::Released => win.on_mouse_button_released(match button {
                            MouseButton::Left => 0,
                            MouseButton::Right => 2,
                            MouseButton::Middle => 1,
                            MouseButton::Other(id) => id
                        }),
                    }
                },
                WindowEvent::CursorMoved { position, .. } => {
                    if exclusive_fullscreen {
                        let mouse_x = position.x;
                        let mouse_y = position.y;
                        win.on_mouse_moved(
                            mouse_x / (scale_up_x as f64),
                            mouse_y / (scale_up_y)
                        );
                    } else {
                        let logical: glutin::dpi::LogicalPosition<f64> = position.to_logical(scale_factor);
                        let mouse_x = logical.x;
                        let mouse_y = logical.y;
                        win.on_mouse_moved(
                            mouse_x / (scale_up_x as f64),
                            mouse_y / (scale_up_y)
                        );
                    }
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                crate::graphics::clear_background(0.0, 0.0, 0.0, 1.0);
                win.render();
                windowed_context.swap_buffers().unwrap();
            },
            _ => {
                let instant = Instant::now();
                match win.update() {
                    PixelWindowControlFlow::Continue => {
                        win.prerender();
                        windowed_context.window().request_redraw();
                        *control_flow = ControlFlow::WaitUntil(instant + W::FRAME_INTERVAL);
                    }
                    PixelWindowControlFlow::Exit => {
                        *control_flow = ControlFlow::Exit
                    }
                }
            },
        }
    });
}