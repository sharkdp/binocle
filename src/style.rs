use crate::view::View;

pub type Color = [u8; 4];

pub trait Style {
    fn init(&mut self, _view: &View) {}
    fn color_at_index(&self, view: &View, view_index: isize) -> Color;
}

pub struct Colorful;

impl Style for Colorful {
    fn color_at_index(&self, view: &View, view_index: isize) -> Color {
        if let Some(b) = view.byte_at(view_index) {
            [b, b.overflowing_mul(2).0, b.overflowing_mul(4).0, 255]
        } else {
            [0, 0, 0, 0]
        }
    }
}
pub struct Grayscale;

impl Style for Grayscale {
    fn color_at_index(&self, view: &View, view_index: isize) -> Color {
        if let Some(b) = view.byte_at(view_index) {
            [b, b, b, 255]
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub struct Category;

impl Style for Category {
    fn color_at_index(&self, view: &View, view_index: isize) -> Color {
        if let Some(b) = view.byte_at(view_index) {
            if b == 0x00 {
                [0, 0, 0, 255]
            } else if b == 0xFF {
                [255, 255, 255, 255]
            } else if b.is_ascii_graphic() {
                [60, 255, 96, 255]
            } else if b.is_ascii_whitespace() {
                [240, 240, 240, 255]
            } else if b.is_ascii() {
                [60, 178, 255, 255]
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
            let rgb = gradient.at((byte as f64) / 255.0f64);
            *color = [
                (rgb.r * 255.0) as u8,
                (rgb.g * 255.0) as u8,
                (rgb.b * 255.0) as u8,
                255,
            ];
        }

        ColorGradient { byte_color }
    }
}

impl Style for ColorGradient {
    fn color_at_index(&self, view: &View, view_index: isize) -> Color {
        if let Some(b) = view.byte_at(view_index) {
            self.byte_color[b as usize]
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub struct RGBA;

impl Style for RGBA {
    fn color_at_index(&self, view: &View, view_index: isize) -> Color {
        if let Some(int) = view.le_u32_at(view_index) {
            int.to_le_bytes()
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub struct ABGR;

impl Style for ABGR {
    fn color_at_index(&self, view: &View, view_index: isize) -> Color {
        if let Some(int) = view.le_u32_at(view_index) {
            int.to_be_bytes()
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub struct RGB;

impl Style for RGB {
    fn color_at_index(&self, view: &View, view_index: isize) -> Color {
        if let Some([r, g, b]) = view.rgb_at(view_index) {
            [r, g, b, 255]
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub struct BGR;

impl Style for BGR {
    fn color_at_index(&self, view: &View, view_index: isize) -> Color {
        if let Some([b, g, r]) = view.rgb_at(view_index) {
            [r, g, b, 255]
        } else {
            [0, 0, 0, 0]
        }
    }
}

pub struct Entropy {
    window_size: usize,
}

impl Entropy {
    pub fn with_window_size(window_size: usize) -> Entropy {
        Entropy { window_size }
    }
}

impl Style for Entropy {
    fn init(&mut self, _: &View) {}

    fn color_at_index(&self, view: &View, view_index: isize) -> Color {
        if let Some(bytes) = view.slice_at(view_index, self.window_size) {
            let mut counts: [i32; 256] = [0; 256];
            for byte in bytes.iter() {
                counts[*byte as usize] += 1;
            }

            let mut entropy = 0.0f64;
            for count in counts {
                if count > 0 {
                    let p = (count as f64) / (self.window_size as f64);
                    entropy -= p * p.log2();
                }
            }
            entropy *= 1.0f64 / 8.0f64;

            let color = colorgrad::magma().at(entropy);

            [
                (color.r * 255.0) as u8,
                (color.g * 255.0) as u8,
                (color.b * 255.0) as u8,
                255,
            ]
        } else {
            [0, 0, 0, 0]
        }
    }
}
