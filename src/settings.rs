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

    pub pixel_style: PixelStyle,

    pub buffer_length: isize,
    pub canvas_width: isize,
}
