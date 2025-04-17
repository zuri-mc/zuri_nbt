//! See [Writer].
use std::io::Write;

use crate::err::{NBTError, PathPart, WriteError};

/// A short notation for the result type used in the [Writer].
pub type Res = Result<(), NBTError<WriteError>>;

/// A trait that can be implemented to alter how basic NBT types are written.
///
/// All the implemented methods must not panic.
pub trait Writer {
    /// Writes an 8-bit unsigned integer.
    fn write_u8<W: Write>(&mut self, writer: &mut W, x: u8) -> Res {
        writer.write_all(&x.to_le_bytes())?;
        Ok(())
    }

    /// Writes a 16-bit signed integer.
    fn write_i16<W: Write>(&mut self, writer: &mut W, x: i16) -> Res;

    /// Writes a 32-bit signed integer.
    fn write_i32<W: Write>(&mut self, writer: &mut W, x: i32) -> Res;

    /// Writes a 64-bit signed integer.
    fn write_i64<W: Write>(&mut self, writer: &mut W, x: i64) -> Res;

    /// Writes a 32-bit floating point number.
    fn write_f32<W: Write>(&mut self, writer: &mut W, x: f32) -> Res;

    /// Writes a 64-bit floating point number.
    fn write_f64<W: Write>(&mut self, writer: &mut W, x: f64) -> Res;

    /// Writes the NBT `end` tag, which indicates the end of a compound tag.
    fn write_end<W: Write>(&mut self, writer: &mut W) -> Res {
        self.write_u8(writer, 0)
    }

    /// Writes a variable-length string.
    fn write_string<W: Write>(&mut self, writer: &mut W, x: &str) -> Res {
        if x.len() > i16::MAX as usize {
            return Err(NBTError::new(WriteError::SeqLengthViolation(
                i16::MAX as usize,
                x.len(),
            )));
        }

        self.write_i16(writer, x.len() as i16)?;
        for (i, b) in x.as_bytes().iter().enumerate() {
            self.write_u8(writer, *b)
                .map_err(|err| err.prepend(PathPart::Element(i)))?;
        }
        Ok(())
    }

    /// Writes variable-length array of 8-bit unsigned integers.
    fn write_u8_vec<W: Write>(&mut self, writer: &mut W, x: &[u8]) -> Res {
        if x.len() > i32::MAX as usize {
            return Err(NBTError::new(WriteError::SeqLengthViolation(
                i32::MAX as usize,
                x.len(),
            )));
        }
        self.write_i32(writer, x.len() as i32)?;
        for (i, v) in x.iter().enumerate() {
            self.write_u8(writer, *v)
                .map_err(|err| err.prepend(PathPart::Element(i)))?;
        }
        Ok(())
    }

    /// Writes variable-length array of 32-bit signed integers.
    fn write_i32_vec<W: Write>(&mut self, writer: &mut W, x: &[i32]) -> Res {
        if x.len() > i32::MAX as usize {
            return Err(NBTError::new(WriteError::SeqLengthViolation(
                i32::MAX as usize,
                x.len(),
            )));
        }
        self.write_i32(writer, x.len() as i32)?;
        for (i, v) in x.iter().enumerate() {
            self.write_i32(writer, *v)
                .map_err(|err| err.prepend(PathPart::Element(i)))?;
        }
        Ok(())
    }

    /// Writes variable-length array of 64-bit signed integers.
    fn write_i64_vec<W: Write>(&mut self, writer: &mut W, x: &[i64]) -> Res {
        if x.len() > i32::MAX as usize {
            return Err(NBTError::new(WriteError::SeqLengthViolation(
                i32::MAX as usize,
                x.len(),
            )));
        }
        self.write_i32(writer, x.len() as i32)?;
        for (i, v) in x.iter().enumerate() {
            self.write_i64(writer, *v)
                .map_err(|err| err.prepend(PathPart::Element(i)))?;
        }
        Ok(())
    }
}

/// Allow mutable references to writers to be used as a writer.
impl<T: Writer> Writer for &mut T {
    fn write_u8<W: Write>(&mut self, writer: &mut W, x: u8) -> Res {
        (**self).write_u8(writer, x)
    }

    fn write_i16<W: Write>(&mut self, writer: &mut W, x: i16) -> Res {
        (**self).write_i16(writer, x)
    }

    fn write_i32<W: Write>(&mut self, writer: &mut W, x: i32) -> Res {
        (**self).write_i32(writer, x)
    }

    fn write_i64<W: Write>(&mut self, writer: &mut W, x: i64) -> Res {
        (**self).write_i64(writer, x)
    }

    fn write_f32<W: Write>(&mut self, writer: &mut W, x: f32) -> Res {
        (**self).write_f32(writer, x)
    }

    fn write_f64<W: Write>(&mut self, writer: &mut W, x: f64) -> Res {
        (**self).write_f64(writer, x)
    }

    fn write_end<W: Write>(&mut self, writer: &mut W) -> Res {
        (**self).write_end(writer)
    }

    fn write_string<W: Write>(&mut self, writer: &mut W, x: &str) -> Res {
        (**self).write_string(writer, x)
    }

    fn write_u8_vec<W: Write>(&mut self, writer: &mut W, x: &[u8]) -> Res {
        (**self).write_u8_vec(writer, x)
    }

    fn write_i32_vec<W: Write>(&mut self, writer: &mut W, x: &[i32]) -> Res {
        (**self).write_i32_vec(writer, x)
    }

    fn write_i64_vec<W: Write>(&mut self, writer: &mut W, x: &[i64]) -> Res {
        (**self).write_i64_vec(writer, x)
    }
}
