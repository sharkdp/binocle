#[derive(PartialEq)]
pub enum PixelStyle {
    Grayscale,
    Colorful,
    Category,
}

pub struct BinocleSettings {
    pub width: usize,
    pub offset: usize,
    pub offset_fine: usize,

    pub pixel_style: PixelStyle,

    pub buffer_length: usize,
    pub canvas_width: usize,
}
