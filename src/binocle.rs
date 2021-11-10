use anyhow::Result;

use crate::buffer::Buffer;
use crate::datatype::Datatype;
use crate::options::{BackingOption, CliOptions};
use crate::settings::{GuiDatatype, PixelStyle, Settings, WIDTH};
use crate::style::{
    Category, ColorGradient, Colorful, DatatypeStyle, Entropy, Grayscale, Style, ABGR, BGR, RGB,
    RGBA,
};
use crate::view::View;

pub struct Binocle {
    pub settings: Settings,
    buffer: Buffer,
}

impl Binocle {
    pub fn new(options: CliOptions) -> Result<Self> {
        let buffer = match options.backing {
            BackingOption::File => Buffer::from_file(options.filename),
            BackingOption::Mmap => Buffer::from_mmap(options.filename),
        }?;

        let buffer_length = buffer.len();
        let settings = Settings {
            buffer_length: buffer_length as isize,
            ..Default::default()
        };

        Ok(Self { buffer, settings })
    }

    pub fn update_hex_view(&mut self) {
        if !self.settings.hex_view_visible {
            return;
        }

        let mut hex_view = String::new();
        let mut hex_ascii = String::new();

        let view = View::new(
            self.buffer.data(),
            self.settings.offset + self.settings.offset_fine,
            1,
        );

        let width = (self.settings.width * self.settings.stride).min(36);
        let height = 24;

        for i in 0..(width * height) {
            if i > 0 && i % width == 0 {
                hex_view.push('\n');
                hex_ascii.push('\n');
            } else if i > 0 && (i % width) % 8 == 0 {
                hex_view.push(' ');
            }

            if let Some(byte) = view.byte_at(i) {
                hex_view.push_str(&format!("{:02x} ", byte));

                if byte.is_ascii_graphic() || (byte as char) == ' ' {
                    hex_ascii.push(byte as char);
                } else {
                    hex_ascii.push('·');
                }
            } else {
                hex_view.push_str("  ");
                hex_ascii.push(' ');
            }
        }
        self.settings.hex_view = hex_view;
        self.settings.hex_ascii = hex_ascii;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        puffin::profile_scope!("Binocle::draw");

        let settings = &self.settings;

        let view = View::new(
            self.buffer.data(),
            settings.offset + settings.offset_fine,
            settings.stride,
        );

        let mut style: Box<dyn Style> = match settings.pixel_style {
            PixelStyle::Colorful => Box::new(Colorful {}),
            PixelStyle::Grayscale => Box::new(Grayscale {}),
            PixelStyle::Category => Box::new(Category {}),
            PixelStyle::GradientMagma => Box::new(ColorGradient::new(colorgrad::magma())),
            PixelStyle::GradientPlasma => Box::new(ColorGradient::new(colorgrad::plasma())),
            PixelStyle::GradientViridis => Box::new(ColorGradient::new(colorgrad::viridis())),
            PixelStyle::GradientRainbow => Box::new(ColorGradient::new(colorgrad::rainbow())),
            PixelStyle::GradientTurbo => Box::new(ColorGradient::new(colorgrad::turbo())),
            PixelStyle::GradientCubehelix => {
                Box::new(ColorGradient::new(colorgrad::cubehelix_default()))
            }
            PixelStyle::RGBA => Box::new(RGBA {}),
            PixelStyle::ABGR => Box::new(ABGR {}),
            PixelStyle::RGB => Box::new(RGB {}),
            PixelStyle::BGR => Box::new(BGR {}),
            PixelStyle::Entropy => Box::new(Entropy::with_window_size(32)),
            PixelStyle::Datatype => Box::new(DatatypeStyle::new(
                match (
                    &settings.datatype_settings.datatype,
                    settings.datatype_settings.signedness,
                ) {
                    (GuiDatatype::Integer8, signedness) => Datatype::Integer8(signedness),
                    (GuiDatatype::Integer16, signedness) => Datatype::Integer16(signedness),
                    (GuiDatatype::Integer32, signedness) => Datatype::Integer32(signedness),
                    (GuiDatatype::Integer64, signedness) => Datatype::Integer64(signedness),
                    (GuiDatatype::Float32, _) => Datatype::Float32,
                    (GuiDatatype::Float64, _) => Datatype::Float64,
                },
                settings.datatype_settings.endianness,
                settings.value_range,
            )),
        };
        style.init(&view);

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let zoom_factor = settings.zoom_factor();
            let x = (((i as isize) % WIDTH as isize) as isize) / zoom_factor;
            let y = (((i as isize) / WIDTH as isize) as isize) / zoom_factor;

            let color = if x >= settings.width {
                [0, 0, 0, 0]
            } else {
                let view_index = y * settings.width + x;

                style.color_at_index(&view, view_index)
            };

            pixel.copy_from_slice(&color);
        }
    }
}
