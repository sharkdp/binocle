use std::convert::TryInto;
use std::ops::Index;

use crate::buffer::Buffer;

pub struct View<'a> {
    stride: isize,
    start: isize,

    data: &'a Buffer,
}

impl<'a> View<'a> {
    pub fn new(data: &'a Buffer, start: isize, stride: isize) -> Self {
        View {
            start,
            stride,
            data,
        }
    }

    pub fn len(&self) -> isize {
        (self.data.len() as isize - self.start) / self.stride
    }

    pub fn iter(&'a self) -> ViewIterator<'a> {
        ViewIterator {
            view: self,
            index: -1,
            length: self.len(),
        }
    }
}

impl<'a> Index<isize> for View<'a> {
    type Output = u8;

    fn index(&self, iter_index: isize) -> &Self::Output {
        let data_index = self.start + iter_index * self.stride;
        &self.data[data_index.try_into().expect("positive index")]
    }
}

pub struct ViewIterator<'a> {
    view: &'a View<'a>,
    index: isize,
    length: isize,
}

impl<'a> Iterator for ViewIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;

        if self.index >= self.length {
            None
        } else {
            Some(self.view[self.index])
        }
    }
}
