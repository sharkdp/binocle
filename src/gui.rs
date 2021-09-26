use egui::{ClippedMesh, FontDefinitions};
use egui_wgpu_backend::{BackendError, RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use pixels::{wgpu, PixelsContext};
use std::time::Instant;
use winit::window::Window;

use crate::settings::{PixelStyle, Settings};

pub struct Gui {
    // State for egui.
    start_time: Instant,
    platform: Platform,
    screen_descriptor: ScreenDescriptor,
    rpass: RenderPass,
    paint_jobs: Vec<ClippedMesh>,
}

impl Gui {
    pub fn new(width: u32, height: u32, scale_factor: f64, pixels: &pixels::Pixels) -> Self {
        let platform = Platform::new(PlatformDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor,
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });
        let screen_descriptor = ScreenDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor: scale_factor as f32,
        };
        let rpass = RenderPass::new(pixels.device(), pixels.render_texture_format(), 1);

        Self {
            start_time: Instant::now(),
            platform,
            screen_descriptor,
            rpass,
            paint_jobs: Vec::new(),
        }
    }

    pub fn handle_event(&mut self, event: &winit::event::Event<'_, ()>) {
        self.platform.handle_event(event);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.physical_width = width;
            self.screen_descriptor.physical_height = height;
        }
    }

    pub fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.scale_factor = scale_factor as f32;
    }

    pub fn prepare(&mut self, window: &Window, settings: &mut Settings) {
        self.platform
            .update_time(self.start_time.elapsed().as_secs_f64());

        // Begin the egui frame.
        self.platform.begin_frame();

        // Draw the application.
        self.ui(&self.platform.context(), settings);

        // End the egui frame and create all paint jobs to prepare for rendering.
        let (_output, paint_commands) = self.platform.end_frame(Some(window));
        self.paint_jobs = self.platform.context().tessellate(paint_commands);
    }

    /// Create the UI using egui.
    fn ui(&mut self, ctx: &egui::CtxRef, settings: &mut Settings) {
        egui::SidePanel::right("Settings").show(ctx, |ui| {
            ui.add(
                egui::Slider::new(&mut settings.zoom, 0..=settings.max_zoom)
                    .clamp_to_range(true)
                    .smart_aim(false)
                    .text("zoom (+/-)"),
            );
            ui.add(
                egui::Slider::new(&mut settings.width, 8..=settings.canvas_width)
                    .clamp_to_range(true)
                    .smart_aim(false)
                    .text("width (left/right)"),
            );
            ui.add(
                egui::Slider::new(&mut settings.stride, 1..=settings.max_stride)
                    .clamp_to_range(true)
                    .smart_aim(false)
                    .text("stride (,/.)"),
            );
            ui.separator();
            ui.label("Offset");
            ui.add(
                egui::Slider::new(&mut settings.offset, 0..=settings.buffer_length)
                    .clamp_to_range(true)
                    .smart_aim(false)
                    .text("coarse ([shift +] up/down)"),
            );
            ui.add(
                egui::Slider::new(&mut settings.offset_fine, 0..=settings.width)
                    .clamp_to_range(true)
                    .smart_aim(false)
                    .text("fine (m/n)"),
            );
            ui.separator();
            ui.label("Pixel style");
            ui.selectable_value(&mut settings.pixel_style, PixelStyle::Colorful, "Colorful");
            ui.selectable_value(
                &mut settings.pixel_style,
                PixelStyle::Grayscale,
                "Grayscale",
            );
            ui.selectable_value(&mut settings.pixel_style, PixelStyle::Category, "Category");
            ui.selectable_value(
                &mut settings.pixel_style,
                PixelStyle::GradientMagma,
                "Gradient (Magma)",
            );
            ui.selectable_value(
                &mut settings.pixel_style,
                PixelStyle::GradientPlasma,
                "Gradient (Plasma)",
            );
            ui.selectable_value(
                &mut settings.pixel_style,
                PixelStyle::GradientViridis,
                "Gradient (Viridis)",
            );
            ui.selectable_value(
                &mut settings.pixel_style,
                PixelStyle::GradientRainbow,
                "Gradient (Rainbow)",
            );
            ui.selectable_value(&mut settings.pixel_style, PixelStyle::RGBA, "RGBA");
            ui.selectable_value(&mut settings.pixel_style, PixelStyle::ABGR, "ABGR");
            ui.selectable_value(&mut settings.pixel_style, PixelStyle::RGB, "RGB");
            ui.selectable_value(&mut settings.pixel_style, PixelStyle::BGR, "BGR");
            ui.separator();
            ui.checkbox(&mut settings.hex_view_visible, "hex view");
        });

        if settings.hex_view_visible {
            egui::TopBottomPanel::bottom("hex view").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut settings.hex_view)
                            .text_style(egui::TextStyle::Monospace)
                            .enabled(false)
                            .frame(false)
                            .desired_width(720.0),
                    );
                    ui.add(
                        egui::TextEdit::multiline(&mut settings.hex_ascii)
                            .text_style(egui::TextStyle::Monospace)
                            .enabled(false)
                            .frame(false),
                    );
                });
            });
        }
    }

    /// Render egui.
    pub fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) -> Result<(), BackendError> {
        // Upload all resources to the GPU.
        self.rpass.update_texture(
            &context.device,
            &context.queue,
            &self.platform.context().texture(),
        );
        self.rpass
            .update_user_textures(&context.device, &context.queue);
        self.rpass.update_buffers(
            &context.device,
            &context.queue,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Record all render passes.
        self.rpass.execute(
            encoder,
            render_target,
            &self.paint_jobs,
            &self.screen_descriptor,
            None,
        )
    }
}
