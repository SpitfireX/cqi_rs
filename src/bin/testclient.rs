use cqi_rs::*;
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
    let mut cqi_data = vec!();

    for token in line.split_ascii_whitespace() {
        match parse_token(token) {
            Some(data) => cqi_data.push(data),
            None => {
                println!("Could not parse token \"{}\"", token);
                return Ok(());
            }
        }
    }

    if cqi_data.len() > 0 {
        println!("Sending {} CQi data object(s): {:?}", cqi_data.len(), cqi_data);
        
        for data in cqi_data {
            connection.write_boxed(data)?;
        }

        let mut buf: Vec<u8> = vec![0; 2];
        connection.stream.read_exact(&mut buf)?;
        print!("Response: ");
        let mut res_bytes = [0; 2];
        res_bytes.copy_from_slice(&buf[..2]);
        
        let datatype = parse_response(u16::from_be_bytes(res_bytes));

        match datatype {
            Some(datatype) => {
                let data: Box<dyn CQiData> = match datatype {
                    DATA::BYTE => Box::new(connection.read::<BYTE>()?),
                    DATA::BOOL => Box::new(connection.read::<BOOL>()?),
                    DATA::INT => Box::new(connection.read::<INT>()?),
                    DATA::STRING => Box::new(connection.read::<STRING>()?),
                    DATA::BYTE_LIST => Box::new(connection.read::<BYTE_LIST>()?),
                    DATA::BOOL_LIST => Box::new(connection.read::<BOOL_LIST>()?),
                    DATA::INT_LIST => Box::new(connection.read::<INT_LIST>()?),
                    DATA::STRING_LIST => Box::new(connection.read::<STRING_LIST>()?),
                    _ => panic!("The fuck is this?"),
                };
                println!(" {:?}", data);
            },
            None => println!(),
        }

        // kinda not working right now
        // let mut f = File::create("response.bin")?;
        // f.write_all(&buf)?;
        
    }

    Ok(())
}

fn parse_response(r: WORD) -> Option<DATA>{
    match ResponseType::from_u8((r >> 8) as u8) {
        Some(ResponseType::STATUS) => {
            print!("STATUS {:?}", STATUS::from_u16(r).unwrap());
            None
        },
        Some(ResponseType::ERROR) => {
            print!("ERROR {:?}", ERROR::from_u16(r).unwrap());
            None
        },
        Some(ResponseType::DATA) => {
            match DATA::from_u16(r) {
                Some(datatype) => {
                    print!("DATA {:?}", datatype);
                    Some(datatype)
                },
                None => {
                    print!("DATA UNKNOWN_TYPE");
                    None
                }
            }

        },
        Some(ResponseType::CL_ERROR) =>{
            print!("CL_ERROR {:?}", CL_ERROR::from_u16(r).unwrap());
            None
        },
        Some(ResponseType::CQP_ERROR) => {
            print!("CQP_ERROR {:?}", CQP_ERROR::from_u16(r).unwrap());
            None
        },
        None => {
            print!("0x{:X} Not a valid response type!", (r >> 8) as u8);
            None
        },
    }
}

fn parse_token(token: &str) -> Option<Box<dyn WriteCQiBytes>> {
    if let Some(data) = parse_command(token) { return Some(data); }
    if let Some(data) = parse_num_type(token) { return Some(data); }
    if let Some(data) = parse_string(token) { return Some(data); }
    None
}

fn parse_command(token: &str) -> Option<Box<WORD>> {
    let command = token.parse::<COMMANDS>();
    match command {
        Ok(x) => Some(Box::new(x as WORD)),
        Err(_) => None,
    }
}

fn parse_num_type(token: &str) -> Option<Box<dyn WriteCQiBytes>> {
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

fn parse_byte(token: &str, radix: u32) -> Option<Box<dyn WriteCQiBytes>> {
    if let Ok(value) = u8::from_str_radix(token, radix) {
        return Some(Box::new(value as BYTE))
    }
    None
}

fn parse_word(token: &str, radix: u32) -> Option<Box<dyn WriteCQiBytes>> {
    if let Ok(value) = u16::from_str_radix(token, radix) {
        return Some(Box::new(value as WORD))
    }
    None
}

fn parse_int(token: &str, radix: u32) -> Option<Box<dyn WriteCQiBytes>> {
    if let Ok(value) = i32::from_str_radix(token, radix) {
        return Some(Box::new(value as INT))
    }
    None
}

fn parse_string(token: &str) -> Option<Box<STRING>> {
    if token.starts_with("\"") && token.starts_with("\"") {
        let len = token.len();
        Some(Box::new(token[1..len-1].to_owned() as STRING))
    } else {
        None
    }
}
