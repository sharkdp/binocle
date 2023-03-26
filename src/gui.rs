use std::time::Instant;

use egui::{ClippedMesh, FontDefinitions};
use egui_wgpu_backend::{BackendError, RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use humansize::{file_size_opts, FileSize};
use pixels::{wgpu, PixelsContext};
use winit::window::Window;

use crate::{
    datatype::{Endianness, Signedness},
    settings::{GuiDatatype, PixelStyle, Settings, HEIGHT},
};

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
        let max_offset_fine = settings.max_offset_fine();
        let max_width = settings.max_width();
        egui::SidePanel::right("Settings").show(ctx, |ui| {
            ui.add(egui::Label::new("Layout").heading());
            ui.add(
                egui::Slider::new(
                    &mut settings.zoom,
                    settings.zoom_range.0..=settings.zoom_range.1,
                )
                .clamp_to_range(true)
                .smart_aim(false)
                .text("zoom"),
            );
            ui.add(
                egui::Slider::new(&mut settings.width, 1..=max_width)
                    .clamp_to_range(true)
                    .smart_aim(false)
                    .text("width"),
            );
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(settings.width % 2 == 0, egui::Button::new("÷ 2"))
                    .clicked()
                {
                    settings.width /= 2;
                }
                if ui
                    .add_enabled(settings.width % 3 == 0, egui::Button::new("÷ 3"))
                    .clicked()
                {
                    settings.width /= 3;
                }
                if ui
                    .add_enabled(settings.width % 5 == 0, egui::Button::new("÷ 5"))
                    .clicked()
                {
                    settings.width /= 5;
                }
                if ui
                    .add_enabled(settings.width % 7 == 0, egui::Button::new("÷ 7"))
                    .clicked()
                {
                    settings.width /= 7;
                }
                if ui.button("× 2").clicked() && 2 * settings.width <= max_width {
                    settings.width *= 2;
                }
            });
            ui.add(
                egui::Slider::new(&mut settings.stride, 1..=settings.max_stride)
                    .clamp_to_range(true)
                    .smart_aim(false)
                    .text("stride"),
            );
            ui.separator();

            ui.add(egui::Label::new("Offset").heading());
            ui.add(
                egui::Slider::new(&mut settings.offset, 0..=settings.buffer_length)
                    .clamp_to_range(true)
                    .smart_aim(false)
                    .text("coarse"),
            );
            ui.add(
                egui::Slider::new(&mut settings.offset_fine, 0..=max_offset_fine)
                    .clamp_to_range(true)
                    .smart_aim(false)
                    .text("fine"),
            );
            ui.separator();

            ui.add(egui::Label::new("Pixel style").heading());
            ui.label("Single byte");
            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(&mut settings.pixel_style, PixelStyle::Colorful, "Default");
                ui.selectable_value(&mut settings.pixel_style, PixelStyle::Category, "Category");
                ui.selectable_value(
                    &mut settings.pixel_style,
                    PixelStyle::Grayscale,
                    "Grayscale",
                );
                ui.selectable_value(
                    &mut settings.pixel_style,
                    PixelStyle::GradientMagma,
                    "Magma",
                );
                ui.selectable_value(
                    &mut settings.pixel_style,
                    PixelStyle::GradientPlasma,
                    "Plasma",
                );
                ui.selectable_value(
                    &mut settings.pixel_style,
                    PixelStyle::GradientViridis,
                    "Viridis",
                );
                ui.selectable_value(
                    &mut settings.pixel_style,
                    PixelStyle::GradientRainbow,
                    "Rainbow",
                );
                ui.selectable_value(
                    &mut settings.pixel_style,
                    PixelStyle::GradientTurbo,
                    "Turbo",
                );
                ui.selectable_value(
                    &mut settings.pixel_style,
                    PixelStyle::GradientCubehelix,
                    "Cubehelix",
                );
                ui.selectable_value(
                    &mut settings.pixel_style,
                    PixelStyle::Entropy,
                    "Entropy (slow)",
                );
            });

            ui.label("Multi-byte");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut settings.pixel_style, PixelStyle::Rgba, "RGBA");
                ui.selectable_value(&mut settings.pixel_style, PixelStyle::Abgr, "ABGR");
                ui.selectable_value(&mut settings.pixel_style, PixelStyle::Rgb, "RGB");
                ui.selectable_value(&mut settings.pixel_style, PixelStyle::Bgr, "BGR");
            });
            ui.selectable_value(&mut settings.pixel_style, PixelStyle::Datatype, "Datatype");
            ui.separator();
            ui.label("Datatype");
            ui.vertical(|ui| {
                ui.set_enabled(settings.pixel_style == PixelStyle::Datatype);

                ui.horizontal_wrapped(|ui| {
                    ui.selectable_value(
                        &mut settings.datatype_settings.datatype,
                        GuiDatatype::Integer8,
                        "Integer (8 bit)",
                    );
                    ui.selectable_value(
                        &mut settings.datatype_settings.datatype,
                        GuiDatatype::Integer16,
                        "Integer (16 bit)",
                    );
                    ui.selectable_value(
                        &mut settings.datatype_settings.datatype,
                        GuiDatatype::Integer32,
                        "Integer (32 bit)",
                    );
                    ui.selectable_value(
                        &mut settings.datatype_settings.datatype,
                        GuiDatatype::Integer64,
                        "Integer (64 bit)",
                    );
                    ui.selectable_value(
                        &mut settings.datatype_settings.datatype,
                        GuiDatatype::Float32,
                        "Float (32 bit)",
                    );
                    ui.selectable_value(
                        &mut settings.datatype_settings.datatype,
                        GuiDatatype::Float64,
                        "Float (64 bit)",
                    );
                });
                ui.label("Signedness");
                ui.horizontal(|ui| {
                    // Only enable for datatypes that have 'signedness'
                    ui.set_enabled(match settings.datatype_settings.datatype {
                        GuiDatatype::Integer8
                        | GuiDatatype::Integer16
                        | GuiDatatype::Integer32
                        | GuiDatatype::Integer64 => true,
                        GuiDatatype::Float32 | GuiDatatype::Float64 => false,
                    });
                    ui.selectable_value(
                        &mut settings.datatype_settings.signedness,
                        Signedness::Unsigned,
                        "Unsigned",
                    );
                    ui.selectable_value(
                        &mut settings.datatype_settings.signedness,
                        Signedness::Signed,
                        "Signed",
                    );
                });
                ui.label("Endianness");
                ui.horizontal(|ui| {
                    // Only enable for datatypes that are multi-byte
                    ui.set_enabled(match settings.datatype_settings.datatype {
                        GuiDatatype::Integer8 => false,
                        GuiDatatype::Integer16
                        | GuiDatatype::Integer32
                        | GuiDatatype::Integer64
                        | GuiDatatype::Float32
                        | GuiDatatype::Float64 => true,
                    });
                    ui.selectable_value(
                        &mut settings.datatype_settings.endianness,
                        Endianness::Little,
                        "Little Endian",
                    );
                    ui.selectable_value(
                        &mut settings.datatype_settings.endianness,
                        Endianness::Big,
                        "Big Endian",
                    );
                });
                ui.label("");
                ui.horizontal(|ui| {
                    ui.label("min:");
                    ui.add(egui::DragValue::new(&mut settings.value_range.0).speed(10.0));
                    ui.label("max:");
                    ui.add(egui::DragValue::new(&mut settings.value_range.1).speed(10.0));
                });
            });

            ui.separator();

            ui.checkbox(&mut settings.hex_view_visible, "hex view");
            ui.separator();

            ui.add(egui::Label::new("Information").heading());
            let file_size = settings
                .buffer_length
                .file_size(file_size_opts::BINARY)
                .unwrap();
            ui.label(format!("file size: {}", file_size));
            let zoom_factor = settings.zoom_factor();
            let grid_size = (settings.width * (HEIGHT as isize) * settings.stride / zoom_factor)
                .file_size(file_size_opts::BINARY)
                .unwrap();
            ui.label(format!("grid size: {}", grid_size));
        });

        if settings.hex_view_visible {
            egui::TopBottomPanel::bottom("hex view").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Label::new(&mut settings.hex_view)
                            .monospace()
                            .wrap(false),
                    );
                    ui.add(
                        egui::Label::new(&mut settings.hex_ascii)
                            .monospace()
                            .wrap(false),
                    );
                });
            });
        }

        settings.gui_wants_keyboard = ctx.wants_keyboard_input();
        settings.gui_wants_mouse = ctx.wants_pointer_input();
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
