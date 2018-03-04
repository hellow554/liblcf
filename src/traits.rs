use std::borrow::Cow;
use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};
use failure::Error;

pub trait ReadDelphi: Read {
    fn read_u8(&mut self) -> Result<u8, Error> {
        unimplemented!()
    }

    fn read_u16(&mut self) -> Result<u16, Error> {
        unimplemented!()
    }

    fn read_u32(&mut self) -> Result<u32, Error> {
        unimplemented!()
    }

    fn read_string(&mut self) -> Result<Cow<str>, Error> {
        let len = self.read_u8()?;
        let mut buf = Vec::new();
        buf.resize(len as usize, 0);
        self.read_exact(&mut buf)?;
        Ok(Cow::Owned(String::from_utf8(buf)?))
    }
}

impl<R: ReadBytesExt + ?Sized> ReadDelphi for R {}

pub trait WriteDelphi : Write {
    fn write_u8(&mut self) -> Result<(), Error> {

    }
    fn write_string(&mut self) -> Result<(), Error> {

    }
}

impl<W: Write + ?Sized> WriteDelphi for W {}