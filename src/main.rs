use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:?}", http_request);

    // Send the file "img.png"
    let file = std::fs::read("src/img.png").unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
    // Slowly send the file over a span of some time
    let time_in_seconds = 30;
    let size = file.len();

    let chunk_size = size / time_in_seconds;
    let chunk_size = ceil(chunk_size);

    let chunks = size / chunk_size;
    let chunks = ceil(chunks);

    println!("Sending {} chunks of size {}", chunks, chunk_size);

    for chunk in file.chunks(chunk_size) {
        match stream.write_all(chunk) {
            Ok(_) => {}
            Err(e) => {
                println!("Error writing chunk: {}", e);
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("File sent!");
}

fn ceil(x: usize) -> usize {
    if x % 2 != 0 {
        x + 1
    } else {
        x
    }
}
