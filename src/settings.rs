pub const WIDTH: u32 = 1366;
pub const HEIGHT: u32 = 1024;

#[derive(PartialEq)]
pub enum PixelStyle {
    Grayscale,
    Colorful,
    Category,
    Entropy,
    GradientMagma,
    GradientPlasma,
    GradientViridis,
    GradientRainbow,
    RGBA,
    ABGR,
    RGB,
    BGR,
    U16BE,
    F32BE,
    F32LE,
}

pub struct Settings {
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

    pub hex_view_visible: bool,
    pub hex_view: String,
    pub hex_ascii: String,

    pub gui_wants_keyboard: bool,
    pub gui_wants_mouse: bool,
}

impl Settings {
    pub fn zoom_factor(&self) -> isize {
        2isize.pow(self.zoom as u32)
    }

    pub fn offset_fine_max(&self) -> isize {
        3 * self.width * self.stride
    }
}
