use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

mod protocol;
use protocol::Telnet;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Connection Established: {}", stream.local_addr()?);
    let msg = [Telnet::IAC, Telnet::DO, Telnet::ECHO];
    stream.write_all(&msg)?;

    loop {
        let mut buf = [0u8; 4028];
        let n = stream.read(&mut buf[..])?;

        if n > 0 {
            if buf[0] == Telnet::IAC {
                println!("Interpret As Command Recieved!");

                let mut command_string: String = String::new();

                for bit in buf[..n].iter() {
                    command_string.push_str(&Telnet::from_u8(*bit));
                }

                println!("{}", command_string);
            } else {
                match std::str::from_utf8(&buf[..n]) {
                    Ok(s) => {
                        // This is good, print it out!
                        println!("Buffer: {}", s);
                    }
                    Err(_) => {
                        // Ignore the input
                        // Send an error message to the sender

                        //let buf: [u8] = "Test" as [u8];
                    }
                };

                //println!("Buffer: {:?}", &buf[..n]);
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
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Connection failed to establish: {}", e);
            }
        }
    }

    Ok(())
}
