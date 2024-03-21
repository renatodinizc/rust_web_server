use rust_web_server::execute;
use std::net::TcpListener;

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:8080").expect("Could not bind to specified port");

    execute(listener);

    Ok(())
}
