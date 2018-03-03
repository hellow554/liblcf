use std::borrow::{Borrow, Cow};
use std::fs::File;
use std::path::Path;
use std::io::{Error as IoErr, Read, Seek, SeekFrom, Write};

use failure::Error;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct LcfReader<'a> {
    reader: &'a mut Read,
}

impl<'a> LcfReader<'a> {
    pub fn new_from_read(reader: &'a mut impl Read) -> Result<LcfReader<'a>, Error> {
        Ok(LcfReader { reader })
    }
}

impl<'a> Seek for LcfReader<'a> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, IoErr> {
        unimplemented!()
    }
}

impl<'a> Read for LcfReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoErr> {
        self.reader.read(buf)
    }
}
