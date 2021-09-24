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
    pub zoom: usize,
    pub width: usize,
    pub offset: usize,
    pub offset_fine: usize,
    pub stride: usize,

    pub pixel_style: PixelStyle,

    pub buffer_length: usize,
    pub canvas_width: usize,
}
