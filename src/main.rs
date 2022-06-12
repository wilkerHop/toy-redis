#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Read;
use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 1024];
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            println!("{}", String::from_utf8_lossy(&data[..size]));

            // return PONG
            match stream.write(b"+PONG\r\n") {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for result in listener.incoming() {
        match result {
            Ok(stream) => {
                println!("Connection established!");

                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => println!("couldn't accept client: {:?}", e),
        }
    }
    // close the socket server
    drop(listener);
}
