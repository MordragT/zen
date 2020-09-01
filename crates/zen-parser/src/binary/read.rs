use super::error::*;
use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
    mem,
};

impl BinaryRead for Cursor<&[u8]> {}

impl BinaryRead for Cursor<Vec<u8>> {}

impl BinaryRead for File {}

pub trait BinaryRead: Read + Seek {
    /// Peek for next byte without advancing the reader
    fn peek(&mut self) -> Result<u8> {
        let mut buf = [0_u8; mem::size_of::<u8>()];
        self.read_exact(&mut buf)?;
        self.seek(SeekFrom::Current(-(mem::size_of::<u8>() as i64)))?;
        Ok(u8::from_le_bytes(buf))
    }
    fn seek_whitespaces(&mut self) -> Result<()> {
        loop {
            let p = self.peek()?;
            if p == b' ' || p == b'\r' || p == b'\t' || p == b'\n' || p == 0 {
                self.seek(SeekFrom::Current(1))?;
            } else {
                break;
            }
        }
        Ok(())
    }
    fn bool(&mut self) -> Result<bool> {
        let val = self.u8()?;
        match val {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::ExpectedBool),
        }
    }
    fn char(&mut self) -> Result<char> {
        let val = self.u8()?;
        Ok(val as char)
    }
    fn u8(&mut self) -> Result<u8> {
        let mut buf = [0_u8; mem::size_of::<u8>()];
        self.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf))
    }
    fn u16(&mut self) -> Result<u16> {
        let mut buf = [0_u8; mem::size_of::<u16>()];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
    fn u32(&mut self) -> Result<u32> {
        let mut buf = [0_u8; mem::size_of::<u32>()];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
    fn u64(&mut self) -> Result<u64> {
        let mut buf = [0_u8; mem::size_of::<u64>()];
        self.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
    fn i8(&mut self) -> Result<i8> {
        let mut buf = [0_u8; mem::size_of::<i8>()];
        self.read_exact(&mut buf)?;
        Ok(i8::from_le_bytes(buf))
    }
    fn i16(&mut self) -> Result<i16> {
        let mut buf = [0_u8; mem::size_of::<i16>()];
        self.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }
    fn i32(&mut self) -> Result<i32> {
        let mut buf = [0_u8; mem::size_of::<i32>()];
        self.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }
    fn i64(&mut self) -> Result<i64> {
        let mut buf = [0_u8; mem::size_of::<i64>()];
        self.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }
    fn f32(&mut self) -> Result<f32> {
        let mut buf = [0_u8; mem::size_of::<f32>()];
        self.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }
    fn f64(&mut self) -> Result<f64> {
        let mut buf = [0_u8; mem::size_of::<f64>()];
        self.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }
    fn string(&mut self) -> Result<String> {
        let mut res = String::new();
        loop {
            match self.u8()? {
                0 => return Ok(res),
                x if x > 0 && x < 128 => res.push(x as char),
                x => return Err(Error::ExpectedAsciiChar(x)),
                //x => res.push(x as char),
            }
        }
    }
}
