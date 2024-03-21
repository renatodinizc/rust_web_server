# Rust Web Server

A basic, lightweight web server built with Rust. This server listens on localhost at port 8080 and serves static HTML files in response to GET requests. It's designed as a simple example of handling HTTP requests in Rust.

## Features

- **Simple Routing:** Handles basic routing to serve different HTML pages based on the request URL.
- **Multi-threaded:** Utilizes Rust's `thread` module to handle connections concurrently.
- **Error Handling:** Implements basic error handling for connection issues and file reading errors.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your system. You can download Rust and find installation instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:

```sh
git clone git@github.com:renatodinizc/rust_web_server.git
```

2. Navigate into the project directory:

```sh
cd rust_web_server
```

3. Build and run the server:

```sh
cargo run
```

The server will start listening on `localhost:8080`. You can access it using a web browser or a tool like `curl` to make requests.

### Directory Structure

- **www/**: This directory contains the static HTML files served by the web server. Add your HTML files here.
  - `index.html`: The default page served at `/`.
  - `hello_world.html`: Example page served at `/hello_world`.
  - `404.html`: Error page displayed when a request does not match existing routes.

## Usage

With the server running, you can access the following URLs from a web browser:

- `http://localhost:8080/` - Displays the content of `index.html`.
- `http://localhost:8080/hello_world` - Displays the content of `hello_world.html`.
- Any other path will result in a 404 page, showing the content of `404.html`.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the server.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.