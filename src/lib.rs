use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::io::Result as IoResult;
use std::io::{Read, Write};
use byteorder::{ByteOrder, NetworkEndian, ReadBytesExt};
use core::fmt::Debug;
use cqi_consts::*;
use num_traits::FromPrimitive;
use std::time::Duration;

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


pub trait CQiData {
    fn repr(&self) -> String;
    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()>;
}

impl Debug for dyn CQiData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.repr())
    }
}

impl CQiData for BOOL {
    fn repr(&self) -> String {
        format!("{}", &self)
    }

    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        Ok(stream.write_all(&[*self as BYTE])?)
    }
}

impl CQiData for BYTE {
    fn repr(&self) -> String {
        format!("0x{:X} [= {}]", &self, &self)
    }

    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        Ok(stream.write_all(&[*self])?)
    }
}

impl CQiData for WORD {
    fn repr(&self) -> String {
        format!("0x{:X} [= {}]", &self, &self)
    }

    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        Ok(stream.write_all(&(self.to_be_bytes()))?)
    }
}

impl CQiData for INT {
    fn repr(&self) -> String {
        format!("{}", &self)
    }

    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        Ok(stream.write_all(&(self.to_be_bytes()))?)
    }
}

impl CQiData for STRING {
    fn repr(&self) -> String {
        format!("\"{}\"", &self)
    }

    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        stream.write_all(&(self.len() as WORD).to_be_bytes())?;
        Ok(stream.write_all(&(self.as_bytes()))?)
    }
}

impl CQiData for &str {
    fn repr(&self) -> String {
        format!("\"{}\"", &self)
    }

    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        stream.write_all(&(self.len() as WORD).to_be_bytes())?;
        Ok(stream.write_all(&(self.as_bytes()))?)
    }
}

impl CQiData for BOOL_LIST {
    fn repr(&self) -> String {
        format!("{:?}", &self)
    }

    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        write_cqi_list(stream, self)
    }
}

impl CQiData for BYTE_LIST {
    fn repr(&self) -> String {
        format!("{:?}.len({})", &self, &self.len())
    }

    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        write_cqi_list(stream, self)
    }
}

impl CQiData for INT_LIST {
    fn repr(&self) -> String {
        format!("{:?}.len({})", &self, &self.len())
    }

    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        write_cqi_list(stream, self)
    }
}

impl CQiData for STRING_LIST {
    fn repr(&self) -> String {
        format!("{:?}.len({})", &self, &self.len())
    }

    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        write_cqi_list(stream, self)
    }
}

fn write_cqi_list<T: CQiData>(stream: &mut TcpStream, list: &[T]) -> IoResult<()> {
    stream.write_all(&(list.len() as INT).to_be_bytes())?;
    for elem in list {
        elem.write_cqi_bytes(stream)?;
    }
    Ok(())
}


pub trait ReadCQiBytes<T> {
    fn read(&mut self) -> IoResult<T>;
}

macro_rules! read_cqi_list {
    ($type:ident, $con:ident) => (
        {
            let len = $con.stream.read_i32::<NetworkEndian>()?;

            let mut data: Vec<$type> = Vec::with_capacity(len as usize);
            
            for _ in 0..len {
                let value: $type = $con.read()?;
                data.push(value);
            }
        
            Ok(data)
        }
    );
}

impl ReadCQiBytes<BOOL> for CQiConnection {
    fn read(&mut self) -> IoResult<BOOL> {
        Ok(self.stream.read_u8()? > 0)
    }
}

impl ReadCQiBytes<BYTE> for CQiConnection {
    fn read(&mut self) -> IoResult<BYTE> {
        Ok(self.stream.read_u8()?)
    }
}

impl ReadCQiBytes<WORD> for CQiConnection {
    fn read(&mut self) -> IoResult<WORD> {
        Ok(self.stream.read_u16::<NetworkEndian>()?)
    }
}

impl ReadCQiBytes<INT> for CQiConnection {
    fn read(&mut self) -> IoResult<INT> {
        Ok(self.stream.read_i32::<NetworkEndian>()?)
    }
}

impl ReadCQiBytes<STRING> for CQiConnection {
    fn read(&mut self) -> IoResult<STRING> {
        let len = self.stream.read_u16::<NetworkEndian>()?;

        let mut data: Vec<u8> = Vec::with_capacity(len as usize);
        
        for _ in 0..len {
            let value: BYTE = self.read()?;
            data.push(value);
        }

        let string = String::from_utf8(data).unwrap();
        Ok(string)
    }
}

impl ReadCQiBytes<BOOL_LIST> for CQiConnection {
    fn read(&mut self) -> IoResult<BOOL_LIST> {
        read_cqi_list!(BOOL, self)
    }
}

impl ReadCQiBytes<BYTE_LIST> for CQiConnection {
    fn read(&mut self) -> IoResult<BYTE_LIST> {
        read_cqi_list!(BYTE, self)
    }
}

impl ReadCQiBytes<INT_LIST> for CQiConnection {
    fn read(&mut self) -> IoResult<INT_LIST> {
        read_cqi_list!(INT, self)
    }
}

impl ReadCQiBytes<STRING_LIST> for CQiConnection {
    fn read(&mut self) -> IoResult<STRING_LIST> {
        read_cqi_list!(STRING, self)
    }
}


pub struct CQiConnection {
    pub stream: TcpStream,
}

// Struct methods
impl CQiConnection {

    pub fn new<A: ToSocketAddrs>(address: A) -> IoResult<CQiConnection> {
        let stream = TcpStream::connect(&address)?;

        let dur = Duration::from_secs(2);
        stream.set_read_timeout(Some(dur))?;
        stream.set_write_timeout(Some(dur))?;

        Ok(CQiConnection { stream: stream })
    }

    pub fn write<A: CQiData>(&mut self, data: A) -> IoResult<()> {
        data.write_cqi_bytes(&mut self.stream)
    }

    pub fn write_boxed(&mut self, data: Box<dyn CQiData>) -> IoResult<()> {
        (*data).write_cqi_bytes(&mut self.stream)
    }
}

macro_rules! send_cqi_data {
    ( $con:ident, $command:path$(, $( $x:expr ),*)? ) => (
        {
            $con.write($command as WORD)?;
            $(
                $(
                    $con.write($x)?;
                )*
            )?
            IoResult::Ok(())
        }
    );
}

macro_rules! receive_cqi_response {
    ( $con:ident, $msg_type:ident ) => (
        {
            let r: WORD = $con.read()?;
            IoResult::Ok($msg_type::from_u16(r).expect("Malformed response."))
        }
    );
}

// CQi commands
impl CQiConnection {

    pub fn ctr_connect(&mut self, user: &str, password: &str) -> IoResult<STATUS> {
        // self.write(COMMANDS::CTRL_CONNECT as WORD)?;
        // self.write(user)?;
        // self.write(password)?;
        send_cqi_data!(self,
            COMMANDS::CTRL_CONNECT,
            user,
            password
        )?;
        receive_cqi_response!(self, STATUS)
    }

    pub fn ctrl_ping(&mut self) -> IoResult<STATUS> {
        send_cqi_data!(self,
            COMMANDS::CTRL_PING
        )?;
        receive_cqi_response!(self, STATUS)
    }
}