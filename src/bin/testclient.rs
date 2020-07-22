use cqi_rs::{CQiConnection, ToCQiBytes, ReadCQiBytes};
use std::io::Result as IoResult;

fn main() {
    let mut connection = CQiConnection::new("localhost:4877").unwrap();
    // ohne magie
    // connection.write_word(0x1101);
    // connection.write_string("test");
    // connection.write_string("ficken23");
    // mit magie
    connection.write(0x1101 as u16);
    connection.write("test");
    connection.write("ficken23");

    let w: IoResult<u16> = connection.read();
}
