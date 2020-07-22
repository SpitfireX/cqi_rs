use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::io::Result as IoResult;
use std::io::{Read, Write};
use std::mem;
use byteorder::{ByteOrder, NetworkEndian, ReadBytesExt};

mod tests;

pub enum CQiData {
    BOOL(bool),
    BYTE(u8),
    WORD(u16),
    INT(i32),
    STRING(String),
    BOOL_LIST(Vec<bool>),
    BYTE_LIST(Vec<u8>),
    INT_LIST(Vec<i32>),
    STRING_LIST(Vec<String>),
}

pub trait ToCQiBytes {
    fn to_cqi_data(self) -> Box<CQiData>;
    fn to_bytes(self) -> Vec<u8>;
}

impl ToCQiBytes for bool {
    fn to_cqi_data(self) -> Box<CQiData> {
        Box::new(CQiData::BOOL(self))
    }

    fn to_bytes(self) -> std::vec::Vec<u8> {
        vec![self as u8]
    }
}

impl ToCQiBytes for u8 {
    fn to_cqi_data(self) -> Box<CQiData> {
        Box::new(CQiData::BYTE(self))
    }

    fn to_bytes(self) -> Vec<u8> {
        vec![self]
    }
}

impl ToCQiBytes for u16 {
    fn to_cqi_data(self) -> Box<CQiData> {
        Box::new(CQiData::WORD(self))
    }

    fn to_bytes(self) -> Vec<u8> {
        (self).to_be_bytes().to_vec()
    }
}

impl ToCQiBytes for i32 {
    fn to_cqi_data(self) -> Box<CQiData> {
        Box::new(CQiData::INT(self))
    }

    fn to_bytes(self) -> Vec<u8> {
        (self).to_be_bytes().to_vec()
    }
}

impl ToCQiBytes for &str {
    fn to_cqi_data(self) -> Box<CQiData> {
        Box::new(CQiData::STRING(self.to_string()))
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut ret = (self.len() as u16).to_be_bytes().to_vec();
        ret.extend_from_slice(self.as_bytes());
        ret
    }
}

pub trait ReadCQiBytes {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<Self>;
}

impl ReadCQiBytes for u16 {
    fn read_cqi_bytes(stream: &mut TcpStream) -> IoResult<u16> {
        stream.read_u16::<NetworkEndian>()
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
        A::read_cqi_bytes(&mut self.stream)
    }

    pub fn write<A: ToCQiBytes>(&mut self, data: A) -> IoResult<usize> {
        let bytes = data.to_bytes();
        let n = self.stream.write(&bytes)?;
        Ok(n)
    }

    pub fn write_word(&mut self, data: u16) -> IoResult<usize> {
        let bytes = data.to_be_bytes();
        let n = self.stream.write(&bytes)?;
        Ok(n)
    }

    pub fn write_string(&mut self, data: &str) -> IoResult<usize> {
        let mut bytes = (data.len() as u16).to_be_bytes().to_vec();
        bytes.extend_from_slice(data.as_bytes());
        let n = self.stream.write(&bytes)?;
        Ok(n)
    }
}