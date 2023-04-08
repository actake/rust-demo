use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // println!("request: {}", String::from_utf8_lossy(&buffer[..]));

    let (status_line, filename) = if buffer.starts_with(b"GET / HTTP/1.1") {
        ("HTTP/1.1 200 OK", "resources/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "resources/404.html")
    };

    let html_content = fs::read_to_string(filename).unwrap();
    let res = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        html_content.len(),
        html_content
    );

    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}
