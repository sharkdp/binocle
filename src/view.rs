use std::ops::Index;

use crate::buffer::Buffer;
use crate::settings::Settings;

struct View<'a> {
    stride: isize,
    width: isize,
    offset: isize,

    data: &'a Buffer,
}

impl<'a> View<'a> {
    fn from_settings(data: &'a Buffer, settings: &Settings) -> Self {
        View {
            stride: settings.stride,
            width: settings.width,
            offset: settings.offset + settings.offset_fine,
            data,
        }
    }
}

struct ViewIterator<'a> {
    view: &'a View<'a>,
    index: isize,
}

impl<'a> Iterator for ViewIterator<'a> {
    type Item = [u8; 4];

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
