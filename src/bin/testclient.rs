use cqi_rs::{CQiConnection, WriteCQiBytes, ReadCQiBytes};
use cqi_rs::{BOOL, BYTE, WORD, INT, STRING};
use std::io::Result as IoResult;

fn main() -> IoResult<()> {
    let mut connection = CQiConnection::new("localhost:4877").unwrap();
    // ohne magie
    // connection.write_word(0x1101);
    // connection.write_string("test");
    // connection.write_string("ficken23");
    // mit magie
    connection.write(0x1101 as WORD);
    connection.write("test");
    connection.write("ficken23");

    let w = connection.read::<WORD>()?;
    print!("{}", w);
    Ok(())
}
