#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Read;
use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for result in listener.incoming() {
        match result {
            Ok(stream) => {
                println!("Connection established!");

                // Read the stream into a buffer
                let mut stream = stream;
                let mut request = [0; 1024];
                stream.read(&mut request).unwrap();

                println!("{}", String::from_utf8_lossy(&request));

                // Write the response
                let response = "+PONG\r\n";
                println!("{}", response);
                stream.write(response.as_bytes()).unwrap();

                println!("Connection closed!");
            }
            Err(e) => println!("couldn't accept client: {:?}", e),
        }
    }
}
