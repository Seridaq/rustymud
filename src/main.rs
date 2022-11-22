use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

mod protocol;
use protocol::Telnet;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Connection Established: {}", stream.local_addr()?);
    let msg = [Telnet::IAC, Telnet::DO, Telnet::LINE_MODE];
    stream.write_all(&msg)?;

    loop {
        let mut buf = [0u8; 4028];
        let n = stream.read(&mut buf[..])?;

        if n > 0 {
            if buf[0] == Telnet::IAC {
                println!("Interpret As Command Recieved!");

                let mut command_string: String = String::new();
                for bit in buf[..n].iter() {
                    match *bit {
                        Telnet::IAC => command_string += "IAC ",
                        _ => command_string += &format!("\'{:?}\' ", bit),
                    };
                }

                println!("{}", command_string);
            } else {
                //let s = str::from_utf8(&buf[..n]).expect("Invalid UTF8 format.").trim_end();

                //println!("Buffer: {}", s);
                println!("Buffer: {:?}", &buf[..n]);
            }
        } else {
            println!("Connection Closed: {}", stream.local_addr()?);
            break;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
