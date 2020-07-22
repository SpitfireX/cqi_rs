use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::io::Result as IoResult;
use std::io::{Read, Write};
use std::mem;
use byteorder::{ByteOrder, NetworkEndian, ReadBytesExt};
use std::any::Any;

mod tests;

pub type BOOL = bool;
pub type BYTE = u8;
pub type WORD = u16;
pub type INT = i32;
pub type STRING = &'static str;
pub type BOOL_LIST = Vec<BOOL>;
pub type BYTE_LIST = Vec<BYTE>;
pub type INT_LIST = Vec<INT>;
pub type STRING_LIST = Vec<STRING>;

// pub type BOOL = bool;
// pub type BYTE = u8;
// pub type WORD = u16;
// pub type INT = i32;
// pub type STRING = &'static str;
// pub type LIST = Vec<dyn CQiBasicType>;

// trait CQiBasicType {}
// impl CQiBasicType for BOOL {}
// impl CQiBasicType for BYTE {}
// impl CQiBasicType for INT {}
// impl CQiBasicType for STRING {}

// impl WriteCQiBytes for LIST {

// }

pub trait WriteCQiBytes {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()>;
}

impl WriteCQiBytes for BOOL {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        Ok(stream.write_all(&[*self as BYTE])?)
    }
}

impl WriteCQiBytes for BYTE {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        Ok(stream.write_all(&[*self])?)
    }
}

impl WriteCQiBytes for WORD {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        Ok(stream.write_all(&(self.to_be_bytes()))?)
    }
}

impl WriteCQiBytes for INT {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        Ok(stream.write_all(&(self.to_be_bytes()))?)
    }
}

impl WriteCQiBytes for STRING {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        stream.write_all(&(self.len() as WORD).to_be_bytes())?;
        Ok(stream.write_all(&(self.as_bytes()))?)
    }
}

impl WriteCQiBytes for BOOL_LIST {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        stream.write_all(&(self.len() as WORD).to_be_bytes())?;
        for elem in self {
            elem.write_cqi_bytes(stream)?;
        }
        Ok(())
    }
}

impl WriteCQiBytes for BYTE_LIST {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        stream.write_all(&(self.len() as WORD).to_be_bytes())?;
        for elem in self {
            elem.write_cqi_bytes(stream)?;
        }
        Ok(())
    }
}

impl WriteCQiBytes for INT_LIST {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        stream.write_all(&(self.len() as WORD).to_be_bytes())?;
        for elem in self {
            elem.write_cqi_bytes(stream)?;
        }
        Ok(())
    }
}

impl WriteCQiBytes for STRING_LIST {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        stream.write_all(&(self.len() as WORD).to_be_bytes())?;
        for elem in self {
            elem.write_cqi_bytes(stream)?;
        }
        Ok(())
    }
}

pub trait ReadCQiBytes {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<Box<Self>>;
}

impl ReadCQiBytes for WORD {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<Box<WORD>> {
        Ok(Box::new(stream.read_u16::<NetworkEndian>()?))
    }
}

pub struct CQiConnection {
    stream: TcpStream,
}

impl CQiConnection {

    pub fn new<A: ToSocketAddrs>(address: A) -> IoResult<CQiConnection> {
        let stream = TcpStream::connect(&address)?;

        Ok(CQiConnection { stream: stream })
    }

    pub fn read<A: ReadCQiBytes>(&mut self) -> IoResult<A> {
        let result = A::read_cqi_bytes(&mut self.stream)?;
        Ok(*result)
    }

    pub fn write<A: WriteCQiBytes>(&mut self, data: A) -> IoResult<()> {
        data.write_cqi_bytes(&mut self.stream)
    }
    
}