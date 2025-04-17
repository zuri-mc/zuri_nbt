//! See [Reader].
use crate::err::{NBTError, PathPart, ReadError};
use std::{i16, io::Read};

/// A short notation for the result type used in the [Reader].
pub type Res<T> = Result<T, NBTError<ReadError>>;

/// A trait that can be implemented to alter how basic NBT types are read.
///
/// All the implemented methods must not panic.
pub trait Reader {
    /// Reads an 8-bit unsigned integer.
    fn u8<R: Read>(&mut self, reader: &mut R) -> Res<u8> {
        let mut buf = [0u8];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(buf[0])
    }

    /// Reads a 16-bit signed integer.
    fn i16<R: Read>(&mut self, reader: &mut R) -> Res<i16>;

    /// Reads a 32-bit signed integer.
    fn i32<R: Read>(&mut self, reader: &mut R) -> Res<i32>;

    /// Reads a 64-bit signed integer.
    fn i64<R: Read>(&mut self, reader: &mut R) -> Res<i64>;

    /// Reads a 32-bit floating point number.
    fn f32<R: Read>(&mut self, reader: &mut R) -> Res<f32>;

    /// Reads a 64-bit floating point number.
    fn f64<R: Read>(&mut self, reader: &mut R) -> Res<f64>;

    /// Reads the NBT `end` tag, which indicates the end of a compound tag.
    fn end<R: Read>(&mut self, reader: &mut R) -> Res<()> {
        let t = self.u8(reader)?;
        if t != 0 {
            return Err(NBTError::new(ReadError::UnexpectedTag(
                "END (0x00)".to_string(),
                format!("{t:#04x}"),
            )));
        }
        Ok(())
    }

    /// Reads a variable-length string.
    fn string<R: Read>(&mut self, reader: &mut R) -> Res<String> {
        let len = self.i16(reader)?;
        let len: usize = len.try_into().map_err(|_| {
            NBTError::new(ReadError::SeqLengthViolation(i16::MAX as usize, len as i32))
        })?;

        let mut str_buf = Vec::with_capacity(len.min(1024));
        for i in 0..len {
            str_buf.push(
                self.u8(reader)
                    .map_err(|err| err.prepend(PathPart::Element(i as usize)))?,
            );
        }

        String::from_utf8(str_buf).map_err(|err| NBTError::new(ReadError::from(err)))
    }

    /// Reads variable-length array of 8-bit unsigned integers.
    fn u8_vec<R: Read>(&mut self, reader: &mut R) -> Res<Vec<u8>> {
        let len = self.i32(reader)?;
        let len: usize = len.try_into().map_err(|_| {
            NBTError::new(ReadError::SeqLengthViolation(
                // i32 has a lower limit on 32 bit machines.
                usize::MAX.min(i32::MAX as usize),
                len,
            ))
        })?;

        let mut vec_buf = Vec::with_capacity(len.min(1024));
        for i in 0..len {
            vec_buf.push(
                self.u8(reader)
                    .map_err(|err| err.prepend(PathPart::Element(i as usize)))?,
            );
        }

        Ok(vec_buf)
    }

    /// Reads variable-length array of 32-bit signed integers.
    fn i32_vec<R: Read>(&mut self, reader: &mut R) -> Res<Vec<i32>> {
        let len = self.i32(reader)?;
        let len: usize = len.try_into().map_err(|_| {
            NBTError::new(ReadError::SeqLengthViolation(
                // i32 has a lower limit on 32 bit machines.
                usize::MAX.min(i32::MAX as usize),
                len,
            ))
        })?;

        let mut vec_buf = Vec::with_capacity(len.min(1024 / size_of::<i32>()));
        for i in 0..len {
            vec_buf.push(
                self.i32(reader)
                    .map_err(|err| err.prepend(PathPart::Element(i as usize)))?,
            );
        }

        Ok(vec_buf)
    }

    /// Reads variable-length array of 64-bit signed integers.
    fn i64_vec<R: Read>(&mut self, reader: &mut R) -> Res<Vec<i64>> {
        let len = self.i32(reader)?;
        let len: usize = len.try_into().map_err(|_| {
            NBTError::new(ReadError::SeqLengthViolation(
                // i32 has a lower limit on 32 bit machines.
                usize::MAX.min(i32::MAX as usize),
                len,
            ))
        })?;

        let mut vec_buf = Vec::with_capacity(len.min(1024 / size_of::<i64>()));
        for i in 0..len {
            vec_buf.push(
                self.i64(reader)
                    .map_err(|err| err.prepend(PathPart::Element(i as usize)))?,
            );
        }

        Ok(vec_buf)
    }
}

/// Allow for mutable references to readers to be used as a reader.
impl<T: Reader> Reader for &mut T {
    fn u8<R: Read>(&mut self, reader: &mut R) -> Res<u8> {
        (**self).u8(reader)
    }

    fn i16<R: Read>(&mut self, reader: &mut R) -> Res<i16> {
        (**self).i16(reader)
    }

    fn i32<R: Read>(&mut self, reader: &mut R) -> Res<i32> {
        (**self).i32(reader)
    }

    fn i64<R: Read>(&mut self, reader: &mut R) -> Res<i64> {
        (**self).i64(reader)
    }

    fn f32<R: Read>(&mut self, reader: &mut R) -> Res<f32> {
        (**self).f32(reader)
    }

    fn f64<R: Read>(&mut self, reader: &mut R) -> Res<f64> {
        (**self).f64(reader)
    }

    fn end<R: Read>(&mut self, reader: &mut R) -> Res<()> {
        (**self).end(reader)
    }

    fn string<R: Read>(&mut self, reader: &mut R) -> Res<String> {
        (**self).string(reader)
    }

    fn u8_vec<R: Read>(&mut self, reader: &mut R) -> Res<Vec<u8>> {
        (**self).u8_vec(reader)
    }

    fn i32_vec<R: Read>(&mut self, reader: &mut R) -> Res<Vec<i32>> {
        (**self).i32_vec(reader)
    }

    fn i64_vec<R: Read>(&mut self, reader: &mut R) -> Res<Vec<i64>> {
        (**self).i64_vec(reader)
    }
}
