/*
# Current directory
# cd ./kifuwarabe-air2019
cd C:\Users\むずでょ\source\repos\kifuwarabe-air2019\kifuwarabe-air2019

cargo check
cargo run --example server
 */
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::io::{BufRead, BufReader, BufWriter, Error, Read, Write};
use std::thread;

/**
 * See: [Rustにっき/8日目・TCPサーバ](https://cha-shu00.hatenablog.com/entry/2019/03/02/174532)
 */
fn main(){
    let host = "localhost";
    let port = 9696;
    let url = format!("{}:{}", host, port);
    let mut addrs = url.to_socket_addrs().unwrap();

    // Change to ip v4.
    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
        // Address         | 127.0.0.1:9696
        println!("Address         | {}", addr);

        // Wait for connection.
        let listener = TcpListener::bind(addr).expect("Error. failed to bind.");

        // Server standup  | 127.0.0.1:9696
        println!("Server standup  | {}", addr);

        for streams in listener.incoming() {
            match streams {
                Err(e) => { eprintln!("error: {}", e)},
                Ok(stream) => {
                    println!("Create the thread.");
                    // Create the thread.
                    thread::spawn(move || {
                        handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                    });
                }
            }
        }
   } else {
        eprintln!("Invalid Host:Port Number");
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection from {}", stream.peer_addr()?);

    // Buffering.
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);


    let mut line = String::new();

    loop {
        println!("Read            | ...");
        if let Err(err) = reader.read_line(&mut line) {
            panic!("error during receive a line: {}", err);
        }
        println!("Read            | {}", line);

        if line == "quit" {
            return Ok(());
        }

        // 改行までがメッセージ。
        println!("Write           | piyo");
        writer.write("piyo\n".as_bytes())?;
        writer.flush()?;            
    }
}

