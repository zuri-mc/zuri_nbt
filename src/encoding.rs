//! Contains all the standard NBT encodings.
//!
//! These include:
//!  - [BigEndian]
//!  - [LittleEndian]
//!  - [NetworkLittleEndian]
use crate::err::{ErrorPath, PathPart, ReadError, WriteError};
use crate::reader::Reader;
use crate::writer::Writer;
use crate::{reader, writer};
use std::io::{Read, Write};

/// An NBT encoding that encodes all basic types using big endian encoding.
///
/// This format is most commonly used in Minecraft: Java Edition.
#[derive(Debug, Default, Clone)]
pub struct BigEndian;

/// An NBT encoding that encodes all basic types using little endian encoding.
///
/// This format is most commonly used in Minecraft: Bedrock Edition, and more specifically in
/// Bedrock Edition world saves.
///
/// It is not to be confused with the [NetworkLittleEndian] encoding.
#[derive(Debug, Default, Clone)]
pub struct LittleEndian;

/// An NBT encoding that encodes certain integer types using variable-length encoding, while using
/// fixed-size little endian encoding for all other basic types.
///
/// This format is most commonly used for nbt sent in Minecraft: Bedrock Edition's protocol.
#[derive(Debug, Default, Clone)]
pub struct NetworkLittleEndian;

impl Reader for BigEndian {
    fn i16<R: Read>(&mut self, reader: &mut R) -> reader::Res<i16> {
        let mut buf = [0u8; size_of::<i16>()];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(i16::from_be_bytes(buf))
    }

    fn i32<R: Read>(&mut self, reader: &mut R) -> reader::Res<i32> {
        let mut buf = [0u8; size_of::<i32>()];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(i32::from_be_bytes(buf))
    }

    fn i64<R: Read>(&mut self, reader: &mut R) -> reader::Res<i64> {
        let mut buf = [0u8; size_of::<i64>()];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(i64::from_be_bytes(buf))
    }

    fn f32<R: Read>(&mut self, reader: &mut R) -> reader::Res<f32> {
        let mut buf = [0u8; size_of::<f32>()];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(f32::from_be_bytes(buf))
    }

    fn f64<R: Read>(&mut self, reader: &mut R) -> reader::Res<f64> {
        let mut buf = [0u8; size_of::<f64>()];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(f64::from_be_bytes(buf))
    }
}

impl Writer for BigEndian {
    fn write_i16<W: Write>(&mut self, buf: &mut W, x: i16) -> writer::Res {
        buf.write_all(&x.to_be_bytes())?;
        Ok(())
    }

    fn write_i32<W: Write>(&mut self, buf: &mut W, x: i32) -> writer::Res {
        buf.write_all(&x.to_be_bytes())?;
        Ok(())
    }

    fn write_i64<W: Write>(&mut self, buf: &mut W, x: i64) -> writer::Res {
        buf.write_all(&x.to_be_bytes())?;
        Ok(())
    }

    fn write_f32<W: Write>(&mut self, buf: &mut W, x: f32) -> writer::Res {
        buf.write_all(&x.to_be_bytes())?;
        Ok(())
    }

    fn write_f64<W: Write>(&mut self, buf: &mut W, x: f64) -> writer::Res {
        buf.write_all(&x.to_be_bytes())?;
        Ok(())
    }
}

impl Reader for LittleEndian {
    fn i16<R: Read>(&mut self, reader: &mut R) -> reader::Res<i16> {
        let mut buf = [0u8; size_of::<i16>()];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(i16::from_le_bytes(buf))
    }

    fn i32<R: Read>(&mut self, reader: &mut R) -> reader::Res<i32> {
        let mut buf = [0u8; size_of::<i32>()];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(i32::from_le_bytes(buf))
    }

    fn i64<R: Read>(&mut self, reader: &mut R) -> reader::Res<i64> {
        let mut buf = [0u8; size_of::<i64>()];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(i64::from_le_bytes(buf))
    }

    fn f32<R: Read>(&mut self, reader: &mut R) -> reader::Res<f32> {
        let mut buf = [0u8; size_of::<f32>()];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(f32::from_le_bytes(buf))
    }

    fn f64<R: Read>(&mut self, reader: &mut R) -> reader::Res<f64> {
        let mut buf = [0u8; size_of::<i64>()];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(f64::from_le_bytes(buf))
    }
}

impl Writer for LittleEndian {
    fn write_i16<W: Write>(&mut self, buf: &mut W, x: i16) -> writer::Res {
        buf.write_all(&x.to_le_bytes())?;
        Ok(())
    }

    fn write_i32<W: Write>(&mut self, buf: &mut W, x: i32) -> writer::Res {
        buf.write_all(&x.to_le_bytes())?;
        Ok(())
    }

    fn write_i64<W: Write>(&mut self, buf: &mut W, x: i64) -> writer::Res {
        buf.write_all(&x.to_le_bytes())?;
        Ok(())
    }

    fn write_f32<W: Write>(&mut self, buf: &mut W, x: f32) -> writer::Res {
        buf.write_all(&x.to_le_bytes())?;
        Ok(())
    }

    fn write_f64<W: Write>(&mut self, buf: &mut W, x: f64) -> writer::Res {
        buf.write_all(&x.to_le_bytes())?;
        Ok(())
    }
}

impl Reader for NetworkLittleEndian {
    fn i16<R: Read>(&mut self, reader: &mut R) -> reader::Res<i16> {
        let mut buf = [0u8; size_of::<i16>()];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(i16::from_le_bytes(buf))
    }

    fn i32<R: Read>(&mut self, reader: &mut R) -> reader::Res<i32> {
        let mut v: u32 = 0;
        for i in (0..35).step_by(7) {
            let b = self.u8(reader)?;

            v |= ((b & 0x7f) as u32) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i32;
                return Ok(if v & 1 != 0 { -x } else { x });
            }
        }
        Err(ErrorPath::new(ReadError::Custom(
            "varint overflows integer".to_string(),
        )))
    }

    fn i64<R: Read>(&mut self, reader: &mut R) -> reader::Res<i64> {
        let mut v: u64 = 0;
        for i in (0..70).step_by(7) {
            let b = self.u8(reader)?;

            v |= ((b & 0x7f) as u64) << i;
            if b & 0x80 == 0 {
                let x = (v >> 1) as i64;
                return Ok(if v & 1 != 0 { -x } else { x });
            }
        }
        Err(ErrorPath::new(ReadError::Custom(
            "varint overflows integer".to_string(),
        )))
    }

    fn f32<R: Read>(&mut self, reader: &mut R) -> reader::Res<f32> {
        let mut buf = [0u8; size_of::<f32>()];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(f32::from_le_bytes(buf))
    }

    fn f64<R: Read>(&mut self, reader: &mut R) -> reader::Res<f64> {
        let mut buf = [0u8; size_of::<i64>()];
        reader.read_exact(buf.as_mut_slice())?;

        Ok(f64::from_le_bytes(buf))
    }

    fn string<R: Read>(&mut self, reader: &mut R) -> reader::Res<String> {
        let len = 'var_len: {
            let mut v: u32 = 0;
            for i in (0..35).step_by(7) {
                let b = self.u8(reader)?;

                v |= ((b & 0x7f) as u32) << i;
                if b & 0x80 == 0 {
                    break 'var_len v;
                }
            }
            return Err(ErrorPath::new(ReadError::Custom(
                "varint overflows integer".to_string(),
            )));
        };

        let mut str_reader = Vec::with_capacity(len.min(1024) as usize);
        for i in 0..len {
            str_reader.push(
                self.u8(reader)
                    .map_err(|err| err.prepend(PathPart::Element(i as usize)))?,
            );
        }

        String::from_utf8(str_reader).map_err(|err| ErrorPath::new(ReadError::from(err)))
    }
}

impl Writer for NetworkLittleEndian {
    fn write_u8<W: Write>(&mut self, buf: &mut W, x: u8) -> writer::Res {
        buf.write_all(&x.to_le_bytes())?;
        Ok(())
    }

    fn write_i16<W: Write>(&mut self, buf: &mut W, x: i16) -> writer::Res {
        buf.write_all(&x.to_le_bytes())?;
        Ok(())
    }

    fn write_i32<W: Write>(&mut self, buf: &mut W, x: i32) -> writer::Res {
        let mut u = (x as u32) << 1;
        if x < 0 {
            u = !u;
        }
        while u >= 0x80 {
            self.write_u8(buf, u as u8 | 0x80)?;
            u >>= 7;
        }
        self.write_u8(buf, u as u8)?;
        Ok(())
    }

    fn write_i64<W: Write>(&mut self, buf: &mut W, x: i64) -> writer::Res {
        let mut u = (x as u64) << 1;
        if x < 0 {
            u = !u;
        }
        while u >= 0x80 {
            self.write_u8(buf, u as u8 | 0x80)?;
            u >>= 7;
        }
        self.write_u8(buf, u as u8)?;
        Ok(())
    }

    fn write_f32<W: Write>(&mut self, buf: &mut W, x: f32) -> writer::Res {
        buf.write_all(&x.to_le_bytes())?;
        Ok(())
    }

    fn write_f64<W: Write>(&mut self, buf: &mut W, x: f64) -> writer::Res {
        buf.write_all(&x.to_le_bytes())?;
        Ok(())
    }

    fn write_string<W: Write>(&mut self, buf: &mut W, x: &str) -> writer::Res {
        if x.len() > i16::MAX as usize {
            return Err(ErrorPath::new(WriteError::SeqLengthViolation(
                i16::MAX as usize,
                x.len(),
            )));
        }

        let mut l = x.len() as u32;
        while l >= 0x80 {
            self.write_u8(buf, l as u8 | 0x80)?;
            l >>= 7;
        }
        self.write_u8(buf, l as u8)?;
        for b in x.as_bytes() {
            self.write_u8(buf, *b)?;
        }
        Ok(())
    }
}

/// Test all encodings with various data.
#[cfg(test)]
mod tests {
    use crate::encoding::{BigEndian, LittleEndian, NetworkLittleEndian};
    use crate::reader::Reader;
    use crate::writer::Writer;
    use crate::{tag, NBTTag};

    #[test]
    fn test_big_endian() {
        test::<BigEndian>();
    }

    #[test]
    fn test_little_endian() {
        test::<LittleEndian>();
    }

    #[test]
    fn test_network_little_endian() {
        test::<NetworkLittleEndian>();
    }

    fn test<T: Reader + Writer + Sized + Default>() {
        let nbt = tag::Compound::builder()
            .with_long("test", 10)
            .with_byte("test1", 100)
            .with_short("test2", 1)
            .with_list(
                "test3",
                vec![tag::ByteArray(vec![1, 2, 3]), tag::ByteArray(vec![4, 5, 6])],
            )
            .with_list("test4", vec![tag::Byte(1), tag::Byte(3)])
            .with("test5", tag::Compound::default());
        let nbt = NBTTag::Compound(nbt.build());
        let mut buf_writer = Vec::new();
        nbt.write(&mut buf_writer, T::default()).unwrap();

        let buf: Vec<u8> = buf_writer.into();
        assert_eq!(
            NBTTag::read(&mut buf.as_slice(), T::default()).unwrap(),
            nbt
        );
    }
}
