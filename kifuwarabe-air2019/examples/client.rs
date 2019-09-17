/*
# Current directory
# cd ./kifuwarabe-air2019
cd C:\Users\むずでょ\source\repos\kifuwarabe-air2019\kifuwarabe-air2019

cargo run --example client
 */
use std::net::{TcpStream, ToSocketAddrs};
use std::io::{BufRead, BufReader, BufWriter, Write};

/**
 * See: [Rust で TCP/IP ソケット通信をする際のモデル](https://qiita.com/7ma7X/items/479ad9025a3368c2348f)
 */
fn main(){
    let host = "localhost";
    let port = 9696;
    let url = format!("{}:{}", host, port);
    let mut addrs = url.to_socket_addrs().unwrap();

    // Change to ip v4.
    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
        // Address         | 127.0.0.1:3000
        println!("Address         | {}", addr);

        match TcpStream::connect(addr) {
            Err(_) => {
                println!("Info            | Connection NG.");
            }
            Ok(stream) => {
                println!("Info            | Connection Ok.");

                // Buffering.
                let mut reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);

                write_something(&mut writer, "hoge");

                read_something(&mut reader);

                write_something(&mut writer, "quit");
            }
        }
   } else {
        eprintln!("Invalid Host:Port Number");
    }
}

fn read_something (reader: &mut BufReader<&TcpStream>) {
  let mut msg = String::new();

  println!("Read            | ...");
  reader.read_line(&mut msg).expect("RECEIVE FAILURE!!!");

  // read_line は改行文字まで読む。
  // 他のread系のメソッドもある （https://doc.rust-lang.org/std/io/trait.BufRead.html）
  println!("{}", msg);
}

fn write_something (writer: &mut BufWriter<&TcpStream>, comment: &str) {
    // 改行までがメッセージ。
    let msg = format!("{}\n", comment);

    println!("Write           | {}", msg);
    writer.write(msg.as_bytes()).expect("SEND FAILURE!!!");
    writer.flush().unwrap();
}
