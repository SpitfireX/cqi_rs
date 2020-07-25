use cqi_rs::{CQiConnection, WriteCQiBytes, ReadCQiBytes};
use cqi_rs::{BOOL, BYTE, WORD, INT, STRING};
use std::io::Result as IoResult;
use cqi_rs::cqi_consts::*;
use num_traits::FromPrimitive;
use std::io::{Read, Write};
use std::fs::File;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() -> IoResult<()> {
    let mut connection = CQiConnection::new("localhost:4877")?;

    // automatically login in the beginning
    connection.write(COMMANDS::CTRL_CONNECT as WORD)?;
    connection.write("test".to_string())?; // user
    connection.write("ficken23".to_string())?; // password

    let r = connection.read::<WORD>()?;
    parse_response(r);

    //REPL
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("CQi $ ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                process_line(&mut connection, &line)?;
            },
            Err(ReadlineError::Interrupted) => {
                println!("Received CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("Received EOF");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();

    println!("Closing CQi connection...");
    connection.write(COMMANDS::CTRL_BYE as WORD)?;
    let r = connection.read::<WORD>()?;
    parse_response(r);
    // connection will be closed when value is dropped

    Ok(())
}

fn process_line(connection: &mut CQiConnection, line: &String) -> IoResult<()>{    
    let mut bytes: Vec<u8> = vec!();

    for token in line.split_ascii_whitespace() {
        match parse_token(token) {
            Some(x) => bytes.extend(x.iter()),
            None => {
                println!("Could not parse token {}", token);
                break;
            }
        }
    }

    if bytes.len() > 0 {
        println!("Sending {} bytes: {:X?}", &bytes.len(), &bytes);
        connection.stream.write(&bytes)?;

        let mut buf = [0; 1000];
        connection.stream.read(&mut buf)?;
        println!("Response: {:X?}", &buf[..]);

        let mut f = File::create("response.bin")?;
        f.write_all(&buf)?;
    }

    Ok(())
}

fn parse_response(r: WORD) {
    match ResponseType::from_u8((r >> 8) as u8) {
        Some(ResponseType::STATUS) => println!("STATUS {:?}", STATUS::from_u16(r).unwrap()),
        Some(ResponseType::ERROR) => println!("ERROR {:?}", ERROR::from_u16(r).unwrap()),
        Some(ResponseType::DATA) => println!("DATA {:?}", DATA::from_u16(r).unwrap()),
        Some(ResponseType::CL_ERROR) => println!("CL_ERROR {:?}", CL_ERROR::from_u16(r).unwrap()),
        Some(ResponseType::CQP_ERROR) => println!("CQP_ERROR {:?}", CQP_ERROR::from_u16(r).unwrap()),
        None => println!("0x{:X} Not a valid response type!", (r >> 8) as u8),
    }
}

fn parse_token(token: &str) -> Option<Vec<u8>> {
    println!("frag: {}", token);
    if let Some(bytes) = parse_command(token) { return Some(bytes); }
    if let Some(bytes) = parse_num_type(token) { return Some(bytes); }
    if let Some(bytes) = parse_string(token) { return Some(bytes); }
    None
}

fn parse_command(token: &str) -> Option<Vec<u8>> {
    let command = token.parse::<COMMANDS>();
    match command {
        Ok(x) => Some(Vec::from((x as WORD).to_be_bytes())),
        Err(_) => None,
    }
}

fn parse_num_type(token: &str) -> Option<Vec<u8>> {
    let frags: Vec<&str> = token.split(":").collect();

    if frags.len() >= 1 {
        let mut num = frags[0];
        let ntype = if frags.len() >= 2 { frags[1] } else { "byte" };
        let mut radix = 10;
        if num.starts_with("0x") {
            radix = 16;
            num = &num[2..];
        }

        return match ntype {
            "byte" => parse_byte(num, radix),
            "word" => parse_word(num, radix),
            "int" => parse_int(num, radix),
            _ => None
        }
    }
    None
}

fn parse_byte(token: &str, radix: u32) -> Option<Vec<u8>> {
    if let Ok(value) = u8::from_str_radix(token, radix) {
        return Some(vec!(value))
    }
    None
}

fn parse_word(token: &str, radix: u32) -> Option<Vec<u8>> {
    if let Ok(value) = u16::from_str_radix(token, radix) {
        return Some(value.to_be_bytes().to_vec())
    }
    None
}

fn parse_int(token: &str, radix: u32) -> Option<Vec<u8>> {
    if let Ok(value) = i32::from_str_radix(token, radix) {
        return Some(value.to_be_bytes().to_vec())
    }
    None
}

fn parse_string(token: &str) -> Option<Vec<u8>> {
    if token.starts_with("\"") && token.starts_with("\"") {
        let mut bytes = vec!();
        let len = token.len();
        bytes.extend((token[1..len-1].len() as u16).to_be_bytes().iter());
        bytes.extend(token[1..len-1].as_bytes());
        Some(bytes)
    } else {
        None
    }
}
