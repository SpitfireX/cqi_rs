use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::io::Result as IoResult;
use std::io::{Read, Write};
use byteorder::{ByteOrder, NetworkEndian, ReadBytesExt};

#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub mod cqi_consts;
mod tests;

pub type BOOL = bool;
pub type BYTE = u8;
pub type WORD = u16;
pub type INT = i32;
pub type STRING = String;
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

pub trait ReadCQiBytes {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<Box<Self>>;
}

impl ReadCQiBytes for BOOL {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<Box<BOOL>> {
        Ok(Box::new(stream.read_u8()? > 0))
    }
}

impl ReadCQiBytes for BYTE {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<Box<BYTE>> {
        Ok(Box::new(stream.read_u8()?))
    }
}

impl ReadCQiBytes for WORD {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<Box<WORD>> {
        Ok(Box::new(stream.read_u16::<NetworkEndian>()?))
    }
}

impl ReadCQiBytes for INT {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<Box<INT>> {
        Ok(Box::new(stream.read_i32::<NetworkEndian>()?))
    }
}

fn read_cqi_list<T: ReadCQiBytes>(stream: &mut TcpStream) -> IoResult<Box<Vec<T>>> {
    let len = stream.read_i32::<NetworkEndian>()?;
    let mut data: Vec<T> = Vec::with_capacity(len as usize);
    
    for value in &mut data {
        *value = *T::read_cqi_bytes(stream)?;
    }
    
    Ok(Box::new(data))
}

impl ReadCQiBytes for BOOL_LIST {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<Box<Vec<BOOL>>> {
        read_cqi_list::<BOOL>(stream)
    }
}

impl ReadCQiBytes for BYTE_LIST {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<Box<Vec<BYTE>>> {
        read_cqi_list::<BYTE>(stream)
    }
}

impl ReadCQiBytes for INT_LIST {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<Box<Vec<INT>>> {
        read_cqi_list::<INT>(stream)
    }
}

impl ReadCQiBytes for STRING {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<Box<STRING>> {
        let bytes = *BYTE_LIST::read_cqi_bytes(stream)?;
        let string = String::from_utf8(bytes).unwrap();
        Ok(Box::new(string))
    }
}

impl ReadCQiBytes for STRING_LIST {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<Box<Vec<STRING>>> {
        read_cqi_list::<STRING>(stream)
    }
}

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

fn write_cqi_list<T: WriteCQiBytes>(stream: &mut TcpStream, list: &[T]) -> IoResult<()> {
    stream.write_all(&(list.len() as INT).to_be_bytes())?;
    for elem in list {
        elem.write_cqi_bytes(stream)?;
    }
    Ok(())
}

impl WriteCQiBytes for BOOL_LIST {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        write_cqi_list(stream, self)
    }
}

impl WriteCQiBytes for BYTE_LIST {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        write_cqi_list(stream, self)
    }
}

impl WriteCQiBytes for INT_LIST {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        write_cqi_list(stream, self)
    }
}

impl WriteCQiBytes for STRING_LIST {
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        write_cqi_list(stream, self)
    }
}


pub struct CQiConnection {
    pub stream: TcpStream,
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