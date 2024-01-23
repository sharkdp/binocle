use crate::datatype::{Endianness, Signedness};

pub const WIDTH: u32 = 1366;
pub const HEIGHT: u32 = 800;

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
    GradientTurbo,
    GradientCubehelix,
    Rgba,
    Abgr,
    Rgb,
    Bgr,
    Datatype,
}

#[derive(Clone, PartialEq)]
pub enum GuiDatatype {
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Float32,
    Float64,
}

pub struct DatatypeSettings {
    pub datatype: GuiDatatype,
    pub signedness: Signedness,
    pub endianness: Endianness,
}

pub struct Settings {
    pub zoom: isize,
    pub zoom_range: (isize, isize),

    pub width: isize,
    pub offset: isize,
    pub offset_fine: isize,

    pub stride: isize,
    pub max_stride: isize,

    pub pixel_style: PixelStyle,
    pub datatype_settings: DatatypeSettings,

    pub buffer_length: isize,
    pub canvas_width: isize,

    pub value_range: (f32, f32),

    pub hex_view_visible: bool,
    pub hex_view: String,
    pub hex_ascii: String,

    pub gui_wants_keyboard: bool,
    pub gui_wants_mouse: bool,
}

impl Settings {
    pub fn zoom_factor(&self) -> isize {
        2isize.pow((self.zoom - 1) as u32)
    }

    pub fn max_offset_fine(&self) -> isize {
        3 * self.width * self.stride
    }

    pub fn max_width(&self) -> isize {
        2 * (WIDTH as isize)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            zoom: 1,
            zoom_range: (1, 7),
            width: 1024,
            offset: 0,
            offset_fine: 0,
            stride: 1,
            max_stride: 128,
            pixel_style: PixelStyle::Colorful,
            datatype_settings: DatatypeSettings {
                datatype: GuiDatatype::Integer16,
                signedness: Signedness::Unsigned,
                endianness: Endianness::Little,
            },
            buffer_length: 0,
            canvas_width: WIDTH as isize,
            value_range: (0.0, 100.0),
            hex_view_visible: false,
            hex_view: "".into(),
            hex_ascii: "".into(),
            gui_wants_keyboard: false,
            gui_wants_mouse: false,
        }
    }
}
