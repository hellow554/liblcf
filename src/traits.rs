use std::borrow::Cow;
use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};
use failure::Error;

pub trait ReadPascalString : Read {
    fn read_string(&mut self) -> Result<Cow<str>, Error> {
        let len = self.read_u8()?;
        let mut buf = Vec::new();
        buf.resize(len as usize, 0);
        self.read_exact(&mut buf)?;
        Ok(Cow::Owned(String::from_utf8(buf)?))
    }
}

impl<R: ReadBytesExt + ?Sized> ReadPascalString for R {}

pub trait WritePascalString : Write {
    fn write_string(&mut self) -> Result<(), Error> {
        unimplemented!()
    }
}

impl<W: WriteBytesExt + ?Sized> WritePascalString for W {}

pub trait Serialize {
    fn serialize(&self, writer: &mut impl WritePascalString) -> Result<(), Error>;
}

pub trait Deserialize {
    type Out;
    fn deserialize(reader: &mut impl ReadPascalString) -> Result<Self::Out, Error>;
}