use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

use anyhow::Result;

use crate::buffer::Buffer;
use crate::settings::{PixelStyle, Settings, WIDTH};

fn grayscale(b: u8) -> [u8; 4] {
    [b, b, b, 255]
}

fn colorful(b: u8) -> [u8; 4] {
    [b, b.overflowing_mul(2).0, b.overflowing_mul(4).0, 255]
}

fn category(b: u8) -> [u8; 4] {
    if b == 0x00 {
        [0, 0, 0, 255]
    } else if b.is_ascii_graphic() {
        [60, 255, 96, 255]
    } else if b.is_ascii_whitespace() {
        [240, 240, 240, 255]
    } else if b.is_ascii() {
        [60, 178, 255, 255]
    } else {
        [249, 53, 94, 255]
    }
}

fn color_gradient(gradient: colorgrad::Gradient) -> Box<dyn Fn(u8) -> [u8; 4]> {
    Box::new(move |b| {
        let color = gradient.at((b as f64) / 255.0f64);
        [
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            255,
        ]
    })
}

pub struct Binocle {
    pub settings: Settings,
    buffer: Buffer,
}

impl Binocle {
    pub fn new(path: &OsStr) -> Result<Self> {
        let buffer = Buffer::from_file(path)?;

        let buffer_length = buffer.len();

        Ok(Self {
            buffer,
            settings: Settings {
                zoom: 0,
                max_zoom: 6,
                width: 804,
                offset: 0,
                offset_fine: 0,
                stride: 1,
                max_stride: 128,
                pixel_style: PixelStyle::Colorful,
                buffer_length: buffer_length as isize,
                canvas_width: WIDTH as isize,
                hex_view_visible: false,
                hex_view: "".into(),
                hex_ascii: "".into(),
            },
        })
    }

    fn buffer_index(&self, x: isize, y: isize) -> Option<usize> {
        let index = self.settings.offset
            + self.settings.offset_fine
            + (y * self.settings.width + x) * self.settings.stride;

        if index < 0 || index >= (self.buffer.len() as isize) {
            None
        } else {
            Some(index as usize)
        }
    }

    pub fn update(&mut self) {
        if !self.settings.hex_view_visible {
            return;
        }

        let mut hex_view = String::new();
        let mut hex_ascii = String::new();
        // if let Some(index) = self.buffer_index(0, 0) {
        //     for (i, byte) in self.buffer[index..].iter().take(32 * 24).enumerate() {
        //         if i > 0 && i % 32 == 0 {
        //             hex_view.push('\n');
        //             hex_ascii.push('\n');
        //         } else if i > 0 && i % 8 == 0 {
        //             hex_view.push(' ');
        //         }

        //         hex_view.push_str(&format!("{:02x} ", byte));

        //         if byte.is_ascii_graphic() || (*byte as char) == ' ' {
        //             hex_ascii.push(*byte as char);
        //         } else {
        //             hex_ascii.push('·');
        //         }
        //     }
        // }
        self.settings.hex_view = hex_view;
        self.settings.hex_ascii = hex_ascii;
    }

    pub fn draw(&self, frame: &mut [u8]) {
        let settings = &self.settings;

        let style: Box<dyn Fn(u8) -> [u8; 4]> = match settings.pixel_style {
            PixelStyle::Category => Box::new(category),
            PixelStyle::Colorful => Box::new(colorful),
            PixelStyle::Grayscale => Box::new(grayscale),
            PixelStyle::GradientMagma => color_gradient(colorgrad::magma()),
            PixelStyle::GradientPlasma => color_gradient(colorgrad::plasma()),
            PixelStyle::GradientViridis => color_gradient(colorgrad::viridis()),
            PixelStyle::GradientRainbow => color_gradient(colorgrad::rainbow()),
        };

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let zoom_factor = 2isize.pow(settings.zoom as u32);
            let x = (((i as isize) % WIDTH as isize) as isize) / zoom_factor;
            let y = (((i as isize) / WIDTH as isize) as isize) / zoom_factor;

            let color = if x > settings.width {
                [0, 0, 0, 0]
            } else {
                if let Some(index) = self.buffer_index(x, y) {
                    let byte = self.buffer[index];
                    style(byte)
                } else {
                    [0, 0, 0, 0]
                }
            };

            pixel.copy_from_slice(&color);
        }
    }
}
