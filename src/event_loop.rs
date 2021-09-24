use std::ffi::OsStr;

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
use crate::settings::{HEIGHT, WIDTH};

pub fn run(filename: &OsStr) -> Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("binocle")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let (mut pixels, mut gui) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor();
        let surface_texture =
            SurfaceTexture::new(window_size.width / 2, window_size.height / 2, &window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
        let gui = Gui::new(window_size.width, window_size.height, scale_factor, &pixels);

        (pixels, gui)
    };

    let mut binocle = Binocle::new(filename)?;

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
                gui.render(encoder, render_target, context)
                    .expect("egui render error");
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

                if input.key_pressed(VirtualKeyCode::Plus) {
                    settings.zoom += 1;
                } else if input.key_pressed(VirtualKeyCode::Minus) {
                    settings.zoom -= 1;
                }

                if input.key_pressed(VirtualKeyCode::Left) {
                    settings.width -= 1;
                } else if input.key_pressed(VirtualKeyCode::Right) {
                    settings.width += 1;
                }

                let offset_factor = if input.held_shift() { 1 } else { 160 };

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

                if input.key_pressed(VirtualKeyCode::Home) {
                    settings.offset = 0;
                } else if input.key_pressed(VirtualKeyCode::End) {
                    settings.offset = settings.buffer_length
                        - settings.width * (HEIGHT as isize) * settings.stride;
                }

                if input.scroll_diff().abs() > 0.5 {
                    let scroll = input.scroll_diff() as isize;
                    if input.held_control() {
                        settings.zoom += scroll;
                    } else if input.held_alt() {
                        settings.width -= scroll;
                    } else {
                        settings.offset -=
                            offset_factor * scroll * settings.width * settings.stride;
                    }
                }

                settings.zoom = settings.zoom.max(0);
                settings.zoom = settings.zoom.min(settings.max_zoom);

                settings.width = settings.width.max(1);
                settings.width = settings.width.min(WIDTH as isize);

                settings.offset = settings.offset.max(0);
                settings.offset = settings.offset.min(settings.buffer_length);

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

            // Update internal state and request a redraw
            binocle.update();
            window.request_redraw();
        }
    });
}
