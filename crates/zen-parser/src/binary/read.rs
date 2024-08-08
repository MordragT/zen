use std::io::{self, SeekFrom};

mod private {
    pub trait Sealed {}
}

pub trait BinaryRead: private::Sealed {
    /// Peek for next byte if present without advancing the reader
    fn peek(&mut self) -> io::Result<Option<u8>>;
    /// Consumes the next byte if present and returns its value
    fn next(&mut self) -> io::Result<Option<u8>>;
    /// Consumes the next chunk if present and returns its value
    fn next_chunk<const N: usize>(&mut self) -> io::Result<Option<[u8; N]>>;
    /// Fills the provided buffer with bytes
    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()>;
    /// Returns the current position
    fn position(&mut self) -> io::Result<u64>;
    /// Sets the position
    fn set_position(&mut self, pos: u64) -> io::Result<()>;
    /// Changes the current position by the given amount
    fn offset_position(&mut self, n: i64) -> io::Result<()>;
}

pub struct BinaryIoReader<R>
where
    R: io::BufRead + io::Seek,
{
    reader: R,
}

impl<R> BinaryIoReader<R>
where
    R: io::BufRead + io::Seek,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R> private::Sealed for BinaryIoReader<R> where R: io::BufRead + io::Seek {}

impl<R> BinaryRead for BinaryIoReader<R>
where
    R: io::BufRead + io::Seek,
{
    fn peek(&mut self) -> io::Result<Option<u8>> {
        let mut buf = [0; 1];

        match self.reader.read(&mut buf)? {
            0 => Ok(None),
            _ => {
                self.reader.seek_relative(-1)?;
                Ok(Some(u8::from_le_bytes(buf)))
            }
        }
    }

    fn next(&mut self) -> io::Result<Option<u8>> {
        let mut buf = [0; 1];

        match self.reader.read(&mut buf)? {
            0 => Ok(None),
            _ => Ok(Some(u8::from_le_bytes(buf))),
        }
    }

    fn next_chunk<const N: usize>(&mut self) -> io::Result<Option<[u8; N]>> {
        let mut buf = [0; N];

        match self.reader.read_exact(&mut buf) {
            Ok(()) => Ok(Some(buf)),
            Err(e) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    Ok(None)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.reader.read_exact(buf)?;
        Ok(())
    }

    fn position(&mut self) -> io::Result<u64> {
        let pos = self.reader.stream_position()?;
        Ok(pos)
    }

    fn set_position(&mut self, pos: u64) -> io::Result<()> {
        self.reader.seek(SeekFrom::Start(pos))?;
        Ok(())
    }

    fn offset_position(&mut self, n: i64) -> io::Result<()> {
        self.reader.seek_relative(n)?;
        Ok(())
    }
}

pub struct BinarySliceReader<'a> {
    slice: &'a [u8],
    position: usize,
}

impl<'a> BinarySliceReader<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self { slice, position: 0 }
    }
}

impl<'a> private::Sealed for BinarySliceReader<'a> {}

impl<'a> BinaryRead for BinarySliceReader<'a> {
    fn peek(&mut self) -> io::Result<Option<u8>> {
        Ok(if self.position < self.slice.len() {
            Some(self.slice[self.position])
        } else {
            None
        })
    }

    fn next(&mut self) -> io::Result<Option<u8>> {
        Ok(if self.position < self.slice.len() {
            self.position += 1;
            Some(self.slice[self.position])
        } else {
            None
        })
    }

    fn next_chunk<const N: usize>(&mut self) -> io::Result<Option<[u8; N]>> {
        let end = self.position + N;

        Ok(if end <= self.slice.len() {
            let chunk = self.slice[self.position..end].try_into().unwrap();
            self.position += N;
            Some(chunk)
        } else {
            None
        })
    }

    fn read_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        let end = self.position + buf.len();

        if end <= self.slice.len() {
            let chunk = &self.slice[self.position..end];
            self.position += buf.len();
            buf.copy_from_slice(chunk);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Unable to fill the provided buffer",
            ))
        }
    }

    fn position(&mut self) -> io::Result<u64> {
        Ok(self.position as u64)
    }

    fn set_position(&mut self, pos: u64) -> io::Result<()> {
        self.position = pos as usize;
        Ok(())
    }

    fn offset_position(&mut self, n: i64) -> io::Result<()> {
        self.position += n as usize;
        Ok(())
    }
}
