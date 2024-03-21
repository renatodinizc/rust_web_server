use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:8080").expect("Could not bind to specified port");

    listener
        .incoming()
        .filter_map(|stream| match stream {
            Err(e) => {
                eprintln!("Couldn't get the connection from stream: {e}");
                None
            }
            Ok(stream) => Some(stream),
        })
        .for_each(|stream| {
            thread::spawn(move || {
                handle_connection(stream);
            });
        });

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .expect("Failed to read request stream");

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /hello_world HTTP/1.1" => ("HTTP/1.1 200 OK", "hello_world.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(format!("www/{filename}")).expect("Couldn't parse file");
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
