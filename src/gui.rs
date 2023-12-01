use egui::{ClippedPrimitive, TexturesDelta, WidgetText};

use egui_wgpu::renderer::ScreenDescriptor;
use egui_wgpu::Renderer;
use egui_winit::winit::event_loop::EventLoopWindowTarget;

use humansize::{file_size_opts, FileSize};
use pixels::{wgpu, PixelsContext};
use winit::window::Window;

use crate::{
    datatype::{Endianness, Signedness},
    settings::{GuiDatatype, PixelStyle, Settings, HEIGHT},
};

pub struct Gui {
    // State for egui.
    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    screen_descriptor: ScreenDescriptor,
    paint_jobs: Vec<ClippedPrimitive>,
    renderer: Renderer,
    textures: TexturesDelta,
}

impl Gui {
    pub fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        width: u32,
        height: u32,
        scale_factor: f32,
        pixels: &pixels::Pixels,
    ) -> Self {
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_ctx = egui::Context::default();
        let mut egui_state = egui_winit::State::new(event_loop);
        egui_state.set_max_texture_side(max_texture_size);
        egui_state.set_pixels_per_point(scale_factor);
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: scale_factor,
        };
        let renderer = Renderer::new(pixels.device(), pixels.render_texture_format(), None, 1);
        let textures = TexturesDelta::default();

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            renderer,
            paint_jobs: Vec::new(),
            textures,
        }
    }

    pub fn handle_event(&mut self, event: &winit::event::WindowEvent) {
        let _ = self.egui_state.on_event(&self.egui_ctx, event);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.size_in_pixels = [width, height];
        }
    }

    pub fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.pixels_per_point = scale_factor as f32;
    }

    pub fn prepare(&mut self, window: &Window, settings: &mut Settings) {
        // Run the egui frame and create all paint jobs to prepare for rendering.
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_ctx.run(raw_input, |egui_ctx| {
            // Draw the demo application.
            Self::ui(egui_ctx, settings);
        });

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(window, &self.egui_ctx, output.platform_output);
        self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
    }

    fn ui(ctx: &egui::Context, settings: &mut Settings) {
        let max_offset_fine = settings.max_offset_fine();
        let max_width = settings.max_width();
        egui::SidePanel::right("Settings").show(ctx, |ui| {
            ui.heading("Layout");
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

            ui.heading("Offset");
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

            ui.heading("Pixel style");
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

            ui.heading("Information");
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
                        egui::Label::new(WidgetText::from(&settings.hex_view).monospace())
                            .wrap(false),
                    );
                    ui.add(
                        egui::Label::new(WidgetText::from(&settings.hex_ascii).monospace())
                            .wrap(false),
                    );
                });
            });
        }

        settings.gui_wants_keyboard = ctx.wants_keyboard_input();
        settings.gui_wants_mouse = ctx.wants_pointer_input();
    }

    pub fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) {
        // Upload all resources to the GPU.
        for (id, image_delta) in &self.textures.set {
            self.renderer
                .update_texture(&context.device, &context.queue, *id, image_delta);
        }
        self.renderer.update_buffers(
            &context.device,
            &context.queue,
            encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Render egui with WGPU
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: render_target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.renderer
                .render(&mut rpass, &self.paint_jobs, &self.screen_descriptor);
        }

        // Cleanup
        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.renderer.free_texture(id);
        }
    }
}
