pub const WIDTH: u32 = 1386;
pub const HEIGHT: u32 = 1024;

#[derive(PartialEq)]
pub enum PixelStyle {
    Grayscale,
    Colorful,
    Category,
    GradientMagma,
    GradientPlasma,
    GradientViridis,
    GradientRainbow,
}

pub struct BinocleSettings {
    pub zoom: isize,
    pub max_zoom: isize,

    pub width: isize,
    pub offset: isize,
    pub offset_fine: isize,

    pub stride: isize,
    pub max_stride: isize,

    pub pixel_style: PixelStyle,

    pub buffer_length: isize,
    pub canvas_width: isize,

    pub hex_view: String,
    pub hex_ascii: String,
}
