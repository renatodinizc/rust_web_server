use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}