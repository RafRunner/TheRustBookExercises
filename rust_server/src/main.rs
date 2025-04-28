use std::{
    fs,
    io::BufReader,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rust_server::{
    http::{HttpMethod, HttpRequest, HttpResponse, HttpStatus},
    thread_pool::ThreadPool,
};

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    let pool = ThreadPool::new(4);

    println!("Listening on port 7878...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    println!("New connection established.");
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Error in connection attempt: {}", e);
            }
        }
    }
    println!("Server shutting down...");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);

    let request = HttpRequest::build(&mut buf_reader).unwrap();

    let (status, filename) = match (request.method, request.path.as_str()) {
        (HttpMethod::GET, "/") => (HttpStatus::Ok, "hello.html"),
        (HttpMethod::GET, "/sleep") => {
            thread::sleep(Duration::from_secs(5));
            (HttpStatus::Ok, "sleepy.html")
        },
        (_, "/echo") => {
            let body = request.raw_request;
            let mut response = HttpResponse::new(HttpStatus::Ok);
            response.str_entity(&body, "text/plain; charset=utf-8");
            response.write(stream);
            return;
        },
        _ => (HttpStatus::NotFound, "404.html"),
    };

    let contents = fs::read_to_string(format!("res/{filename}")).unwrap();
    let mut response = HttpResponse::new(status);
    response.str_entity(&contents, "text/html; charset=utf-8");

    response.write(stream);
}
