use std::{
    cell::RefMut,
    fmt,
    io::{self, Read, Seek, SeekFrom},
};
use zen_parser::prelude::BinaryRead;

/// Vdfs Entry
pub struct Entry<'a> {
    // The index in the archive it was read from
    index: u32,
    name: String,
    pos: u64,
    offset: u64,
    size: u64,
    kind: u32,
    attr: u32,
    io: RefMut<'a, dyn BinaryRead>,
}

impl<'a> Entry<'a> {
    pub(super) fn new(
        index: u32,
        name: String,
        offset: u64,
        size: u64,
        kind: u32,
        attr: u32,
        io: RefMut<'a, dyn BinaryRead>,
    ) -> Self {
        Self {
            index,
            name,
            pos: offset,
            offset,
            size,
            kind,
            attr,
            io,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

impl<'a> fmt::Display for Entry<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vdfs Entry:\n\tName: {}\n\tOffset: {}\n\tSize: {}\n\tKind: {}\n\tAttr: {}",
            self.name, self.offset, self.size, self.kind, self.attr
        )
    }
}

impl<'a> Read for Entry<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos - self.offset >= self.size {
            log::warn!(
                "Entry was read outside of its size:\n\tPosition: {}\n\tOffset: {}\n\tSize: {}",
                self.pos,
                self.offset,
                self.size
            );
            return Ok(0);
        }

        self.io.seek(SeekFrom::Start(self.pos))?;

        let delta = self.io.read(buf)?;
        self.pos += delta as u64;
        Ok(delta)
    }
}

impl<'a> Seek for Entry<'a> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(pos) => {
                self.pos = self.offset + pos;
                return Ok(self.pos);
            }
            SeekFrom::End(pos) => self.size.checked_add_signed(pos),
            SeekFrom::Current(pos) => (self.pos - self.offset).checked_add_signed(pos),
        };

        match new_pos {
            Some(n) => {
                self.pos = n + self.offset;
                Ok(n)
            }
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid seek to a negative or overflowing position",
            )),
        }
    }
}
