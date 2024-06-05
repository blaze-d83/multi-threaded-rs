use multi_threaded::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // Bind the TCP Listener to the port address
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    // Listen for incoming connections
    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(_stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Store incoming request in a buffer
    let mut buffer = [0; 1024];
    // Read the incoming request from the client into the buffer
    stream.read(&mut buffer).unwrap();

    // Define byte array representing GET request for the root path
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Determine status line and filename based on the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // Attempt to read the contents of the file
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        // If the file cannot be read send an error
        Err(_) => {
            let response = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        }
    };

    // Construct appropriate HTTP response with status line, content length and contents
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // Write response to the TCP stream and flush it
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
