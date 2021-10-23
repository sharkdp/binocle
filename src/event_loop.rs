use std::path::Path;

use anyhow::Result;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, WindowBuilder};
use winit_input_helper::WinitInputHelper;

use crate::binocle::Binocle;
use crate::gui::Gui;
use crate::options::CliOptions;
use crate::settings::{HEIGHT, WIDTH};

enum MouseDragAction {
    Nothing,
    ControlOffset {
        start_y: f32,
        start_offset: isize,
    },
    ControlOffsetFine {
        start_x: f32,
        start_offset_fine: isize,
    },
    ControlWidth {
        start_x: f32,
        start_width: isize,
    },
}

pub fn run(options: CliOptions) -> Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title(&format!(
                "binocle - {}",
                Path::new(&options.filename)
                    .file_name()
                    .map(|f| f.to_string_lossy())
                    .as_deref()
                    .unwrap_or("<unknown>")
            ))
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let (mut pixels, mut gui) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
        let gui = Gui::new(window_size.width, window_size.height, scale_factor, &pixels);

        (pixels, gui)
    };

    let mut binocle = Binocle::new(options)?;

    let mut mouse_drag_action = MouseDragAction::Nothing;

    event_loop.run(move |event, _, control_flow| {
        // Update egui inputs
        gui.handle_event(&event);

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // Draw the binocle
            binocle.draw(pixels.get_frame());

            // Prepare egui
            gui.prepare(&window, &mut binocle.settings);

            // Render everything together
            let render_result = pixels.render_with(|encoder, render_target, context| {
                // Render the binocle texture
                context.scaling_renderer.render(encoder, render_target);

                // Render egui
                gui.render(encoder, render_target, context)?;

                Ok(())
            });

            // Basic error handling
            if render_result
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            {
                let mut settings = &mut binocle.settings;

                let offset_factor = if input.held_shift() { 1 } else { 160 };

                if !settings.gui_wants_keyboard {
                    // Close events
                    if input.key_pressed(VirtualKeyCode::Escape)
                        || input.key_pressed(VirtualKeyCode::Q)
                        || input.quit()
                    {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    // Fullscreen
                    if input.key_pressed(VirtualKeyCode::F) {
                        if window.fullscreen().is_none() {
                            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                        } else {
                            window.set_fullscreen(None)
                        }
                    }

                    // Hex view
                    if input.key_pressed(VirtualKeyCode::H) {
                        settings.hex_view_visible = !settings.hex_view_visible;
                    }

                    if input.key_pressed(VirtualKeyCode::Plus)
                        || input.key_pressed(VirtualKeyCode::NumpadAdd)
                    {
                        settings.zoom += 1;
                    } else if input.key_pressed(VirtualKeyCode::Minus)
                        || input.key_pressed(VirtualKeyCode::NumpadSubtract)
                    {
                        settings.zoom -= 1;
                    }

                    if input.key_pressed(VirtualKeyCode::Left) {
                        settings.width -= 1;
                    } else if input.key_pressed(VirtualKeyCode::Right) {
                        settings.width += 1;
                    }

                    if input.key_pressed(VirtualKeyCode::Up) {
                        settings.offset -= offset_factor * settings.width * settings.stride;
                    } else if input.key_pressed(VirtualKeyCode::Down) {
                        settings.offset += offset_factor * settings.width * settings.stride;
                    }

                    if input.key_pressed(VirtualKeyCode::N) {
                        settings.offset -= 1;
                    } else if input.key_pressed(VirtualKeyCode::M) {
                        settings.offset += 1;
                    }

                    if input.key_pressed(VirtualKeyCode::Comma) {
                        settings.stride -= 1;
                    } else if input.key_pressed(VirtualKeyCode::Period) {
                        settings.stride += 1;
                    }

                    if input.key_pressed(VirtualKeyCode::PageUp) {
                        settings.offset -= settings.width * settings.stride * (HEIGHT as isize);
                    } else if input.key_pressed(VirtualKeyCode::PageDown) {
                        settings.offset += settings.width * settings.stride * (HEIGHT as isize);
                    }

                    if input.key_pressed(VirtualKeyCode::Key1)
                        || input.key_pressed(VirtualKeyCode::Numpad1)
                    {
                        settings.stride = 1;
                    } else if input.key_pressed(VirtualKeyCode::Key2)
                        || input.key_pressed(VirtualKeyCode::Numpad2)
                    {
                        settings.stride = 2;
                    } else if input.key_pressed(VirtualKeyCode::Key3)
                        || input.key_pressed(VirtualKeyCode::Numpad3)
                    {
                        settings.stride = 3;
                    } else if input.key_pressed(VirtualKeyCode::Key4)
                        || input.key_pressed(VirtualKeyCode::Numpad4)
                    {
                        settings.stride = 4;
                    } else if input.key_pressed(VirtualKeyCode::Key5)
                        || input.key_pressed(VirtualKeyCode::Numpad5)
                    {
                        settings.stride = 5;
                    } else if input.key_pressed(VirtualKeyCode::Key6)
                        || input.key_pressed(VirtualKeyCode::Numpad6)
                    {
                        settings.stride = 6;
                    } else if input.key_pressed(VirtualKeyCode::Key7)
                        || input.key_pressed(VirtualKeyCode::Numpad7)
                    {
                        settings.stride = 7;
                    } else if input.key_pressed(VirtualKeyCode::Key8)
                        || input.key_pressed(VirtualKeyCode::Numpad8)
                    {
                        settings.stride = 8;
                    } else if input.key_pressed(VirtualKeyCode::Key9)
                        || input.key_pressed(VirtualKeyCode::Numpad9)
                    {
                        settings.stride = 9;
                    }

                    if input.key_pressed(VirtualKeyCode::Home) {
                        settings.offset = 0;
                        settings.offset_fine = 0;
                    } else if input.key_pressed(VirtualKeyCode::End) {
                        settings.offset = settings.buffer_length
                            - settings.width * (HEIGHT as isize) * settings.stride;
                        settings.offset_fine = 0;
                    }
                }

                if !settings.gui_wants_mouse {
                    if input.scroll_diff().abs() > 0.5 {
                        let scroll = input.scroll_diff() as isize;
                        if input.held_control() {
                            settings.zoom += scroll;
                        } else if input.held_alt() {
                            settings.width += scroll;
                        } else {
                            settings.offset -=
                                offset_factor * scroll * settings.width * settings.stride;
                        }
                    }

                    if let Some((x, y)) = input.mouse() {
                        if input.mouse_pressed(0) {
                            if input.held_shift() {
                                mouse_drag_action = MouseDragAction::ControlOffsetFine {
                                    start_x: x,
                                    start_offset_fine: settings.offset_fine,
                                };
                            } else {
                                mouse_drag_action = MouseDragAction::ControlOffset {
                                    start_y: y,
                                    start_offset: settings.offset,
                                };
                            }
                        } else if input.mouse_pressed(1) {
                            mouse_drag_action = MouseDragAction::ControlWidth {
                                start_x: x,
                                start_width: settings.width,
                            };
                        }
                    }

                    if input.mouse_released(0) || input.mouse_released(1) {
                        mouse_drag_action = MouseDragAction::Nothing;
                    }

                    if input.mouse_held(0) || input.mouse_held(1) {
                        if let Some((x, y)) = input.mouse() {
                            let zoom_factor = settings.zoom_factor() as f32;
                            match mouse_drag_action {
                                MouseDragAction::ControlOffset {
                                    start_y,
                                    start_offset,
                                } => {
                                    let delta_y = (y - start_y) / zoom_factor;
                                    let min_offset =
                                        start_offset % (settings.width * settings.stride);
                                    settings.offset = min_offset.max(
                                        start_offset
                                            - (delta_y as isize) * settings.width * settings.stride,
                                    );
                                }
                                MouseDragAction::ControlOffsetFine {
                                    start_x,
                                    start_offset_fine,
                                } => {
                                    let delta_x = (x - start_x) / zoom_factor;
                                    settings.offset_fine =
                                        start_offset_fine - (delta_x as isize) * settings.stride;
                                }
                                MouseDragAction::ControlWidth {
                                    start_x,
                                    start_width,
                                } => {
                                    let delta_x = (x - start_x) / zoom_factor;
                                    settings.width = start_width + (delta_x as isize);
                                }
                                MouseDragAction::Nothing => {}
                            }
                        }
                    }
                }

                settings.zoom = settings.zoom.max(settings.zoom_range.0);
                settings.zoom = settings.zoom.min(settings.zoom_range.1);

                settings.width = settings.width.max(1);
                settings.width = settings.width.min(settings.max_width());

                settings.offset = settings.offset.max(0);
                settings.offset = settings.offset.min(settings.buffer_length);

                settings.offset_fine = settings.offset_fine.max(0);
                settings.offset_fine = settings.offset_fine.min(settings.max_offset_fine());

                settings.stride = settings.stride.max(1);
                settings.stride = settings.stride.min(settings.max_stride);

                // Update the scale factor
                if let Some(scale_factor) = input.scale_factor() {
                    gui.scale_factor(scale_factor);
                }

                // Resize the window
                if let Some(size) = input.window_resized() {
                    pixels.resize_surface(size.width, size.height);
                    gui.resize(size.width, size.height);
                }
            }

            binocle.update_hex_view();
            window.request_redraw();
        }
    });
}
