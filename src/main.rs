use std::{
    io::{BufRead, BufReader, Result, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
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

// maybe concurrency?
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = &http_request.first().unwrap();
    let user_agent = http_request
        .iter()
        .filter_map(|v| {
            if v.contains("User-Agent") {
                Some(v.trim_start_matches("User-Agent: ").to_string())
            } else {
                None
            }
        })
        .next();

    let (_, path) = parse_request_line(request_line).unwrap();

    handle_path(&path, &user_agent.unwrap_or("".to_string()), stream);
}

// add better parsing
// differentiate between methods like GET, POST, DELETE
fn parse_request_line(input: &str) -> Result<(String, String)> {
    let request_line: Vec<&str> = input.split_whitespace().collect();
    let method = request_line.first().unwrap();
    let path = request_line[1];

    Ok((method.to_string(), path.to_string()))
}

fn handle_path(path: &str, user_agent: &str, mut stream: TcpStream) {
    if path == "/" {
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    } else if path.contains("/echo/") {
        let input = path.strip_prefix("/echo/").unwrap();
        echo(input, stream);
    } else if path.contains("/user-agent") {
        echo(user_agent, stream);
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
}
// add path /echo/{str}
// in which it responds with `str` in the body
// additional headers like Content-Type and Content-Length
fn echo(input: &str, mut stream: TcpStream) {
    let content_type = "text/plain";
    let content_length = input.len();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}\r\n",
        content_type, content_length, input
    );
    stream.write_all(response.as_bytes()).unwrap();
}

// add path /read/{file} in which it reads a file on
// the server and returns it in the response body

// add path /upload/{file} to upload to the server
