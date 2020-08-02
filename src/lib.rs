use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::io::Result as IoResult;
use std::io::{Read, Write};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
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
pub type INT_INT = [INT; 2];
pub type INT_INT_INT_INT = [INT; 4];
pub type INT_TABLE = Vec<Vec<INT>>;



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

impl CQiData for INT_INT {
    fn repr(&self) -> String {
        format!("{:?}", &self)
    }

    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        write_cqi_list(stream, self)
    }
}

impl CQiData for INT_INT_INT_INT {
    fn repr(&self) -> String {
        format!("{:?}", &self)
    }

    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        write_cqi_list(stream, self)
    }
}

impl CQiData for INT_TABLE {
    fn repr(&self) -> String {
        let rows = self.len();
        let mut cols = 0;

        if rows > 0 {
            cols = self[0].len();
        }

        format!("{:?}.rows({}).cols({})", &self, rows, cols)
    }

    fn write_cqi_bytes(&self, stream: &mut TcpStream) -> IoResult<()> {
        let rows = self.len();
        let mut cols = 0;

        if rows > 0 {
            cols = self[0].len();
        }

        stream.write_all(&(rows as INT).to_be_bytes())?;
        stream.write_all(&(cols as INT).to_be_bytes())?;

        for row in 0..rows {
            write_cqi_multiple(stream, &self[row])?;
        }

        Ok(())
    }
}

fn write_cqi_list<T: CQiData>(stream: &mut TcpStream, list: &[T]) -> IoResult<()> {
    stream.write_all(&(list.len() as INT).to_be_bytes())?;
    write_cqi_multiple(stream, list)
}

fn write_cqi_multiple<T: CQiData>(stream: &mut TcpStream, list: &[T]) -> IoResult<()> {
    for elem in list {
        elem.write_cqi_bytes(stream)?;
    }
    Ok(())
}


pub struct CQiConnection {
    pub stream: TcpStream,
}

macro_rules! read_cqi_multiple {
    ($con:ident, $readfun:ident, $num:expr) => (
        {
            let len = $num;

            let mut data = Vec::with_capacity(len as usize);
            
            for _ in 0..len {
                let value = $con.$readfun()?;
                data.push(value);
            }
        
            IoResult::Ok(data)
        }
    );
}

macro_rules! read_cqi_list {
    ($con:ident, $readfun:ident) => (
        {
            let len = $con.read_int()?;

            read_cqi_multiple!($con, $readfun, len)
        }
    );
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

    pub fn read_bool(&mut self) -> IoResult<BOOL> {
        Ok(self.stream.read_u8()? > 0)
    }

    pub fn read_byte(&mut self) -> IoResult<BYTE> {
        Ok(self.stream.read_u8()?)
    }

    pub fn read_word(&mut self) -> IoResult<WORD> {
        Ok(self.stream.read_u16::<NetworkEndian>()?)
    }

    pub fn read_int(&mut self) -> IoResult<INT> {
        Ok(self.stream.read_i32::<NetworkEndian>()?)
    }

    pub fn read_string(&mut self) -> IoResult<STRING> {
        let len = self.read_word()?;

        let data = read_cqi_multiple!(self, read_byte, len)?;

        match String::from_utf8(data) {
            Ok(str) => Ok(str),
            Err(_) => Err(IoError::new(IoErrorKind::InvalidData, "Received string bytes are not utf8")),
        }
    }

    pub fn read_bool_list(&mut self) -> IoResult<BOOL_LIST> {
        read_cqi_list!(self, read_bool)
    }

    pub fn read_byte_list(&mut self) -> IoResult<BYTE_LIST> {
        read_cqi_list!(self, read_byte)
    }

    pub fn read_int_list(&mut self) -> IoResult<INT_LIST> {
        read_cqi_list!(self, read_int)
    }

    pub fn read_string_list(&mut self) -> IoResult<STRING_LIST> {
        read_cqi_list!(self, read_string)
    }

    pub fn read_int_int(&mut self) -> IoResult<INT_INT> {
        Ok(
            [
                self.read_int()?,
                self.read_int()?,
            ]
        )
    }

    pub fn read_int_int_int_int(&mut self) -> IoResult<INT_INT_INT_INT> {
        Ok(
            [
                self.read_int()?,
                self.read_int()?,
                self.read_int()?,
                self.read_int()?,
            ]
        )
    }

    pub fn read_int_table(&mut self) -> IoResult<INT_TABLE> {
        let rows = self.read_int()?;
        let cols = self.read_int()?;

        let mut data: INT_TABLE = Vec::with_capacity(rows as usize);

        for _ in 0..rows {
            data.push(read_cqi_multiple!(self, read_int, cols)?);
        }
        
        Ok(data)
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
            let r = $con.read_word()?;
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