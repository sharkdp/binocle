use std::convert::TryInto;

use crate::datatype::{Datatype, Endianness};
use crate::view::View;

use lazy_static::lazy_static;

pub type Color = [u8; 4];

fn rgba_from_color(color: colorgrad::Color) -> Color {
    [
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8,
        255,
    ]
}

pub trait Style {
    fn init(&mut self, _view: &View) {}
    fn color_at_index(&mut self, view: &View, view_index: isize) -> Color;
}

pub struct Colorful;

impl Style for Colorful {
    fn color_at_index(&mut self, view: &View, view_index: isize) -> Color {
        if let Some(b) = view.byte_at(view_index) {
            [b, b.overflowing_mul(2).0, b.overflowing_mul(4).0, 255]
        } else {
            [0, 0, 0, 0]
        }
    }
}
pub struct Grayscale;

impl Style for Grayscale {
    fn color_at_index(&mut self, view: &View, view_index: isize) -> Color {
        if let Some(b) = view.byte_at(view_index) {
            [b, b, b, 255]
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub struct Category;

impl Style for Category {
    fn color_at_index(&mut self, view: &View, view_index: isize) -> Color {
        if let Some(b) = view.byte_at(view_index) {
            if b == 0x00 {
                [0, 0, 0, 255]
            } else if b == 0xFF {
                [255, 255, 255, 255]
            } else if b.is_ascii_alphanumeric() {
                [60, 178, 255, 255]
            } else if b.is_ascii_punctuation() {
                [0, 129, 213, 255]
            } else if b.is_ascii_whitespace() {
                [162, 218, 255, 255]
            } else if b.is_ascii() {
                [60, 255, 137, 255]
            } else {
                [249, 53, 94, 255]
            }
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub struct ColorGradient {
    byte_color: [Color; 256],
}

impl ColorGradient {
    pub fn new(gradient: colorgrad::Gradient) -> Self {
        let mut byte_color = [[0, 0, 0, 0]; 256];
        for (byte, color) in byte_color.iter_mut().enumerate() {
            let gradient_color = gradient.at((byte as f64) / 255.0f64);
            *color = rgba_from_color(gradient_color);
        }

        ColorGradient { byte_color }
    }
}

impl Style for ColorGradient {
    fn color_at_index(&mut self, view: &View, view_index: isize) -> Color {
        if let Some(b) = view.byte_at(view_index) {
            self.byte_color[b as usize]
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub struct DatatypeStyle {
    datatype: Datatype,
    endianness: Endianness,
    colors: Vec<Color>,
    range: (f32, f32),
}

impl DatatypeStyle {
    pub fn new(datatype: Datatype, endianness: Endianness, range: (f32, f32)) -> Self {
        let num_colors = 1024;
        let mut colors = Vec::new();
        colors.reserve(num_colors);

        let gradient = colorgrad::plasma();
        for i in 0..num_colors {
            colors.push(rgba_from_color(
                gradient.at((i as f64) / (num_colors as f64)),
            ));
        }

        DatatypeStyle {
            datatype,
            endianness,
            colors,
            range,
        }
    }

    pub fn color_from_float(&self, t: f32) -> Color {
        let num_colors = self.colors.len();
        let index = ((t - f32::EPSILON) * num_colors as f32) as isize;

        index
            .try_into()
            .ok()
            .and_then(|i: usize| self.colors.get(i))
            .copied()
            .unwrap_or([0, 0, 0, 0])
    }
}

impl Style for DatatypeStyle {
    fn color_at_index(&mut self, view: &View, view_index: isize) -> Color {
        view.slice_at(view_index, self.datatype.size())
            .and_then(|slice| self.datatype.read_as_float_from(slice, self.endianness))
            .map(|t| {
                let (min, max) = self.range;
                self.color_from_float((t - min) / (max - min))
            })
            .unwrap_or([0, 0, 0, 0])
    }
}

pub struct RGBA;

impl Style for RGBA {
    fn color_at_index(&mut self, view: &View, view_index: isize) -> Color {
        if let Some(int) = view.be_u32_at(view_index) {
            int.to_be_bytes()
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub struct ABGR;

impl Style for ABGR {
    fn color_at_index(&mut self, view: &View, view_index: isize) -> Color {
        if let Some(int) = view.be_u32_at(view_index) {
            int.to_le_bytes()
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub struct RGB;

impl Style for RGB {
    fn color_at_index(&mut self, view: &View, view_index: isize) -> Color {
        if let Some([r, g, b]) = view.rgb_at(view_index) {
            [r, g, b, 255]
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub struct BGR;

impl Style for BGR {
    fn color_at_index(&mut self, view: &View, view_index: isize) -> Color {
        if let Some([b, g, r]) = view.rgb_at(view_index) {
            [r, g, b, 255]
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub struct Entropy {
    window_size: usize,
    window_size_f64: f64,
    counts: [i32; 256],
}

impl Entropy {
    pub fn with_window_size(window_size: usize) -> Entropy {
        Entropy {
            window_size,
            window_size_f64: window_size as f64,
            counts: [0; 256],
        }
    }
}

lazy_static! {
    static ref MAGMA: colorgrad::Gradient = colorgrad::magma();
}

impl Style for Entropy {
    fn init(&mut self, _: &View) {}

    fn color_at_index(&mut self, view: &View, view_index: isize) -> Color {
        if let Some(bytes) = view.slice_at(view_index, self.window_size) {
            self.counts.fill(0);

            for byte in bytes.iter() {
                self.counts[*byte as usize] += 1;
            }

            let mut entropy = 0.0f64;
            for count in self.counts {
                if count > 0 {
                    let p = (count as f64) / self.window_size_f64;
                    entropy -= p * p.log2();
                }
            }
            entropy *= 1.0f64 / 8.0f64;

            let color = MAGMA.at(entropy);
            rgba_from_color(color)
        } else {
            [0, 0, 0, 0]
        }
    }
}
