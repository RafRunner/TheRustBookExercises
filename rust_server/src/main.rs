use std::{
    collections::HashMap,
    fs,
    io::{prelude::*, BufReader, self},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rust_server::ThreadPool;

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

    let request_line = read_utf8_line(&mut buf_reader).unwrap();

    println!("{request_line}");

    let mut headers = HashMap::new();

    loop {
        let header_line = read_utf8_line(&mut buf_reader).unwrap();
        let header_line = header_line.trim();

        if header_line.is_empty() {
            break;
        }

        if let Some((key, value)) = header_line.split_once(':') {
            headers
                .entry(key.to_ascii_lowercase())
                .or_insert(Vec::new())
                .push(value.trim().to_owned());
        }
    }

    dbg!(&headers);

    let body = headers
        .get("content-length")
        .and_then(|content_length| content_length[0].parse::<usize>().ok())
        .filter(|content_length| *content_length > 0)
        .and_then(|content_length| {
            let mut body = vec![0u8; content_length];
            match buf_reader.read_exact(&mut body) {
                Ok(()) => {
                    Some(body)
                }
                Err(_) => None,
            }
        });

    dbg!(body.as_ref().map(|vec| String::from_utf8_lossy(&vec)));

    let (status_line, filename) = match request_line.trim() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
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

fn read_utf8_line(buf_reader: &mut dyn BufRead) -> Result<String, io::Error> {
    let mut request_line = Vec::new();
    buf_reader.read_until(b'\n', &mut request_line)?;

    String::from_utf8(request_line)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
