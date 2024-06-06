use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("{:?}", http_request);
    let path = extarct_url_path(&http_request[0]);
    let mut response;

    if path == "/" {
        response = "HTTP/1.1 200 OK\r\n\r\n";
    } else {
        response = "HTTP/1.1 404 Not Found\r\n\r\n";
    }

    stream.write_all(response.as_bytes()).unwrap();
}

fn extarct_url_path(input: &str) -> &str {
    let request_line: Vec<&str> = input.split_whitespace().collect();

    request_line[1]
}
