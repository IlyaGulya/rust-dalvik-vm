use std::{fs, io};
use std::io::Read;

use buffered_reader::{BufferedReader, File};
use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};
use mutf8::MString;

#[derive(Debug, PartialEq)]
pub enum Endianness {
    Little,
    Big,
}

pub trait EndianAwareReader {
    type Endianess: ByteOrder;
    fn inner(&mut self) -> &mut dyn BufferedReader<()>;
    fn skip(&mut self, amount: usize) -> io::Result<()> {
        self.inner()
            .read_int128::<LittleEndian>(amount)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(())
    }
    fn read_u32(&mut self) -> io::Result<u32> {
        self.inner().read_u32::<Self::Endianess>()
    }
    fn read_u16(&mut self) -> io::Result<u16> {
        self.inner().read_u16::<Self::Endianess>()
    }
}

pub struct LittleEndianReader<R> {
    pub inner: R,
}

impl EndianAwareReader for LittleEndianReader<File<'_, ()>> {
    type Endianess = LittleEndian;

    fn inner(&mut self) -> &mut dyn BufferedReader<()> {
        &mut self.inner
    }
}

pub struct BigEndianReader<R> {
    pub inner: R,
}

impl EndianAwareReader for BigEndianReader<File<'_, ()>> {
    type Endianess = BigEndian;
    fn inner(&mut self) -> &mut dyn BufferedReader<()> {
        &mut self.inner
    }
}

pub enum EndianAwareFileReader<'a> {
    Little(LittleEndianReader<File<'a, ()>>),
    Big(BigEndianReader<File<'a, ()>>),
}

impl<'a> EndianAwareReader for EndianAwareFileReader<'a> {
    type Endianess = LittleEndian;

    fn inner(&mut self) -> &mut dyn BufferedReader<()> {
        match self {
            EndianAwareFileReader::Little(reader) => reader.inner(),
            EndianAwareFileReader::Big(reader) => reader.inner(),
        }
    }
}


pub fn create_reader<'a>(file: File<'a, ()>, endianess: &Endianness) -> EndianAwareFileReader<'a> {
    match endianess {
        Endianness::Little => EndianAwareFileReader::Little(LittleEndianReader { inner: file }),
        Endianness::Big => EndianAwareFileReader::Big(BigEndianReader { inner: file }),
    }
}

pub trait Leb128Ext: io::Read {
    fn read_sleb128(&mut self) -> Result<i64, leb128::read::Error>;
    fn read_uleb128(&mut self) -> Result<u64, leb128::read::Error>;

    fn read_uleb128p1(&mut self) -> Result<Option<u64>, leb128::read::Error> {
        let value = self.read_uleb128()?;
        return if value == 0 {
            Ok(None)
        } else {
            Ok(Some(value - 1))
        };
    }
}

impl Leb128Ext for fs::File {
    fn read_sleb128(&mut self) -> Result<i64, leb128::read::Error> {
        leb128::read::signed(self)
    }

    fn read_uleb128(&mut self) -> Result<u64, leb128::read::Error> {
        leb128::read::unsigned(self)
    }
}

pub trait MUtf8Ext: io::Read {
    fn read_mutf8(&mut self, length: u64) -> Result<String, io::Error>;
}

impl MUtf8Ext for fs::File {
    fn read_mutf8(&mut self, length: u64) -> Result<String, io::Error> {
        let mut buffer = vec![0; length as usize];
        self.read_exact(&mut buffer)?;
        MString::from_mutf8(buffer)
            .into_string()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}