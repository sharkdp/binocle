use std::convert::TryInto;

pub struct View<'a> {
    stride: isize,
    start: isize,

    data: &'a [u8],
}

impl<'a> View<'a> {
    pub fn new(data: &'a [u8], start: isize, stride: isize) -> Self {
        assert!(stride >= 1);

        View {
            start,
            stride,
            data,
        }
    }

    pub fn len(&self) -> isize {
        // the length of the view is (len - start)/stride, but rounded towards
        // infinity. that's what the "+ stride - 1" part is for.
        (self.data.len() as isize - self.start + self.stride - 1) / self.stride
    }

    fn data_index(&self, view_index: isize) -> usize {
        (self.start + view_index * self.stride)
            .try_into()
            .expect("positive index")
    }

    pub fn byte_at(&self, view_index: isize) -> Option<u8> {
        self.data.get(self.data_index(view_index)).copied()
    }

    pub fn le_u32_at(&self, view_index: isize) -> Option<u32> {
        let data_index = self.data_index(view_index);
        self.data
            .get(data_index..(data_index + 4))
            .and_then(|slice| slice.try_into().ok())
            .map(u32::from_le_bytes)
    }

    pub fn be_u16_at(&self, view_index: isize) -> Option<u16> {
        let data_index = self.data_index(view_index);
        self.data
            .get(data_index..(data_index + 2))
            .and_then(|slice| slice.try_into().ok())
            .map(u16::from_le_bytes)
    }

    pub fn rgb_at(&self, view_index: isize) -> Option<[u8; 3]> {
        let data_index = self.data_index(view_index);
        self.data
            .get(data_index..(data_index + 3))
            .and_then(|slice| slice.try_into().ok())
    }
}

#[test]
fn view_basic_access() {
    let data: Vec<u8> = vec![0, 1, 2];
    let view = View::new(&data, 0, 1);

    assert_eq!(view.byte_at(0), Some(0));
    assert_eq!(view.byte_at(1), Some(1));
    assert_eq!(view.byte_at(2), Some(2));
    assert_eq!(view.byte_at(3), None);
    assert_eq!(view.byte_at(4), None);
}

#[test]
fn view_access_with_offset() {
    let data: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
    {
        let view = View::new(&data, 2, 1);

        assert_eq!(view.byte_at(0), Some(2));
        assert_eq!(view.byte_at(1), Some(3));
        assert_eq!(view.byte_at(2), Some(4));
        assert_eq!(view.byte_at(3), Some(5));
        assert_eq!(view.byte_at(4), None);
        assert_eq!(view.byte_at(5), None);
    }
    {
        let view = View::new(&data, 5, 1);

        assert_eq!(view.byte_at(0), Some(5));
        assert_eq!(view.byte_at(1), None);
        assert_eq!(view.byte_at(2), None);
    }
    {
        let view = View::new(&data, 6, 1);

        assert_eq!(view.byte_at(0), None);
        assert_eq!(view.byte_at(1), None);
    }
}

#[test]
fn view_access_with_stride() {
    let data: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    {
        let view = View::new(&data, 2, 3);

        assert_eq!(view.byte_at(0), Some(2));
        assert_eq!(view.byte_at(1), Some(5));
        assert_eq!(view.byte_at(2), Some(8));
        assert_eq!(view.byte_at(3), None);
        assert_eq!(view.byte_at(4), None);
    }
    {
        let view = View::new(&data, 0, 2);

        assert_eq!(view.byte_at(0), Some(0));
        assert_eq!(view.byte_at(1), Some(2));
        assert_eq!(view.byte_at(2), Some(4));
        assert_eq!(view.byte_at(3), Some(6));
        assert_eq!(view.byte_at(4), Some(8));
        assert_eq!(view.byte_at(5), None);
        assert_eq!(view.byte_at(6), None);
    }
    {
        let view = View::new(&data, 0, 3);

        assert_eq!(view.byte_at(0), Some(0));
        assert_eq!(view.byte_at(1), Some(3));
        assert_eq!(view.byte_at(2), Some(6));
        assert_eq!(view.byte_at(3), Some(9));
        assert_eq!(view.byte_at(4), None);
        assert_eq!(view.byte_at(5), None);
    }
    {
        let view = View::new(&data, 0, 9);

        assert_eq!(view.byte_at(0), Some(0));
        assert_eq!(view.byte_at(1), Some(9));
        assert_eq!(view.byte_at(2), None);
    }
    {
        let view = View::new(&data, 0, 10);

        assert_eq!(view.byte_at(0), Some(0));
        assert_eq!(view.byte_at(1), None);
        assert_eq!(view.byte_at(2), None);
    }
}

#[test]
fn view_access_u32() {
    let data: Vec<u8> = vec![0x12, 0x34, 0x56, 0x78, 0xaa, 0xbb, 0xcc, 0xdd];
    let view = View::new(&data, 0, 4);

    assert_eq!(view.byte_at(0), Some(0x12));
    assert_eq!(view.byte_at(1), Some(0xaa));

    assert_eq!(view.le_u32_at(0), Some(0x78563412));
    assert_eq!(view.le_u32_at(1), Some(0xddccbbaa));
    assert_eq!(view.le_u32_at(2), None);
}
