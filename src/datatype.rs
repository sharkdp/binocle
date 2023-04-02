use std::convert::TryInto;

#[derive(Clone, Copy, PartialEq)]
pub enum Endianness {
    Big,
    Little,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Signedness {
    Unsigned,
    Signed,
}

#[derive(Clone)]
pub enum Datatype {
    Integer8(Signedness),
    Integer16(Signedness),
    Integer32(Signedness),
    Integer64(Signedness),
    Float32,
    Float64,
}

impl Datatype {
    pub fn size(&self) -> usize {
        match self {
            Self::Integer8(_) => 1,
            Self::Integer16(_) => 2,
            Self::Integer32(_) => 4,
            Self::Integer64(_) => 8,
            Self::Float32 => 4,
            Self::Float64 => 8,
        }
    }

    pub fn read_as_float_from(&self, slice: &[u8], endianness: Endianness) -> Option<f32> {
        // The 'slice.try_into().ok().map(…)' repetition can not easily be removed, as we would
        // need const generics because try_into() returns a '[u8, N]', depending on the size of
        // the data type.
        match (self, endianness) {
            (Datatype::Integer8(Signedness::Unsigned), _) => slice
                .try_into()
                .ok()
                .map(|bytes| u8::from_le_bytes(bytes) as f32),
            (Datatype::Integer8(Signedness::Signed), _) => slice
                .try_into()
                .ok()
                .map(|bytes| i8::from_le_bytes(bytes) as f32),
            (Datatype::Integer16(Signedness::Unsigned), Endianness::Little) => slice
                .try_into()
                .ok()
                .map(|bytes| u16::from_le_bytes(bytes) as f32),
            (Datatype::Integer16(Signedness::Signed), Endianness::Little) => slice
                .try_into()
                .ok()
                .map(|bytes| i16::from_le_bytes(bytes) as f32),
            (Datatype::Integer16(Signedness::Unsigned), Endianness::Big) => slice
                .try_into()
                .ok()
                .map(|bytes| u16::from_be_bytes(bytes) as f32),
            (Datatype::Integer16(Signedness::Signed), Endianness::Big) => slice
                .try_into()
                .ok()
                .map(|bytes| i16::from_be_bytes(bytes) as f32),
            (Datatype::Integer32(Signedness::Unsigned), Endianness::Little) => slice
                .try_into()
                .ok()
                .map(|bytes| u32::from_le_bytes(bytes) as f32),
            (Datatype::Integer32(Signedness::Signed), Endianness::Little) => slice
                .try_into()
                .ok()
                .map(|bytes| i32::from_le_bytes(bytes) as f32),
            (Datatype::Integer32(Signedness::Unsigned), Endianness::Big) => slice
                .try_into()
                .ok()
                .map(|bytes| u32::from_be_bytes(bytes) as f32),
            (Datatype::Integer32(Signedness::Signed), Endianness::Big) => slice
                .try_into()
                .ok()
                .map(|bytes| i32::from_be_bytes(bytes) as f32),

            (Datatype::Integer64(Signedness::Unsigned), Endianness::Little) => slice
                .try_into()
                .ok()
                .map(|bytes| u64::from_le_bytes(bytes) as f32),
            (Datatype::Integer64(Signedness::Signed), Endianness::Little) => slice
                .try_into()
                .ok()
                .map(|bytes| i64::from_le_bytes(bytes) as f32),
            (Datatype::Integer64(Signedness::Unsigned), Endianness::Big) => slice
                .try_into()
                .ok()
                .map(|bytes| u64::from_be_bytes(bytes) as f32),
            (Datatype::Integer64(Signedness::Signed), Endianness::Big) => slice
                .try_into()
                .ok()
                .map(|bytes| i64::from_be_bytes(bytes) as f32),

            (Datatype::Float32, Endianness::Little) => {
                slice.try_into().ok().map(f32::from_le_bytes)
            }
            (Datatype::Float32, Endianness::Big) => slice.try_into().ok().map(f32::from_be_bytes),

            (Datatype::Float64, Endianness::Little) => slice
                .try_into()
                .ok()
                .map(|bytes| f64::from_le_bytes(bytes) as f32),
            (Datatype::Float64, Endianness::Big) => slice
                .try_into()
                .ok()
                .map(|bytes| f64::from_be_bytes(bytes) as f32),
        }
    }
}
