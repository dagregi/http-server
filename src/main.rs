use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Result, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let port = 4221;
    println!("Serving on port: {}", port);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                thread::spawn(|| handle_connection(_stream));
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

    handle_path(&path, &user_agent.unwrap_or("".to_string()), stream).unwrap();
}

// add better parsing
// differentiate between methods like GET, POST, DELETE
fn parse_request_line(input: &str) -> Result<(String, String)> {
    let request_line: Vec<&str> = input.split_whitespace().collect();
    let method = request_line.first().unwrap();
    let path = request_line[1];

    Ok((method.to_string(), path.to_string()))
}

fn handle_path(path: &str, user_agent: &str, mut stream: TcpStream) -> io::Result<()> {
    if path == "/" {
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
    if path.contains("/echo/") {
        let input = path.strip_prefix("/echo/").unwrap();
        echo(input, &stream)?
    }
    if path.contains("/user-agent") {
        echo(user_agent, &stream)?
    }
    if path.contains("/files/") {
        let input = path.strip_prefix("/files/").unwrap();
        read(input, &stream)
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write_all(response.as_bytes())
    }
}

fn echo(input: &str, mut stream: &TcpStream) -> io::Result<()> {
    let content_type = "text/plain";
    let content_length = input.len();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}\r\n",
        content_type, content_length, input
    );

    stream.write_all(response.as_bytes())
}

fn read(input: &str, mut stream: &TcpStream) -> io::Result<()> {
    let envs: Vec<String> = env::args().collect();
    let dir = envs[2].clone();
    let path = format!("{}/{}", dir, input);
    if let Ok(f) = File::open(path) {
        let mut buffer = String::new();
        let mut reader = BufReader::new(f);

        reader.read_line(&mut buffer)?;

        let content_type = "application/octet-stream";
        let content_length = buffer.len();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}\r\n",
            content_type, content_length, buffer
        );

        stream.write_all(response.as_bytes())
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write_all(response.as_bytes())
    }
}

// add path /upload/{file} to upload to the server
