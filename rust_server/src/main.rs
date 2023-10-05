use rust_server::http::request::HttpRequest;
use rust_server::http::response::{HttpResponse, HttpStatus};
use rust_server::http::HttpMethod;
use std::{
    fs,
    io::BufReader,
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

    let (status, filename) = match (request.method, request.path.as_str()) {
        (HttpMethod::GET, "/") => (HttpStatus::Ok, "hello.html"),
        (HttpMethod::GET, "/sleep") => {
            thread::sleep(Duration::from_secs(5));
            (HttpStatus::Ok, "sleepy.html")
        }
        _ => (HttpStatus::NotFound, "404.html"),
    };

    let contents = fs::read_to_string(format!("res/{filename}")).unwrap();
    let mut response = HttpResponse::new(status);
    response.str_entity(&contents, "text/html");

    response.write(stream);
}
