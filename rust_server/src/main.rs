use rust_server::http::{HttpMethod, HttpRequest};
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rust_server::thread_pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Error in connection attempt: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);

    let request = HttpRequest::build(&mut buf_reader).unwrap();

    dbg!(&request);
    dbg!(&request.body_as_string());

    let (status_line, filename) = match (request.method, request.path.as_str()) {
        (HttpMethod::GET, "/") => ("HTTP/1.1 200 OK", "hello.html"),
        (HttpMethod::GET, "/sleep") => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sleepy.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(format!("res/{filename}")).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
