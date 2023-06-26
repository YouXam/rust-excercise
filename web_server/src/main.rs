use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

use web_server::ThreadPool;

use std::sync::{Arc, Mutex};
use ctrlc;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
    let pool = Arc::new(Mutex::new(ThreadPool::new(4)));



    let pool2 = Arc::clone(&pool);
    ctrlc::set_handler(move || {
        println!("Ctrl+C received! Shutting down...");
        let mut pool = pool2.lock().unwrap(); 
        pool.terminate();
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    
    println!("Listening on {:#?}", listener.local_addr().unwrap());


    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!(
                    "Connection established from {:#?}.",
                    stream.peer_addr().unwrap()
                );
                pool.lock().unwrap().execute(|| handle_connection(stream));
            }
            Err(e) => {
                println!("Failed to establish connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (result, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/index.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(1));
            ("HTTP/1.1 200 OK", "src/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "src/404.html"),
    };
    let contents = fs::read_to_string(filename).expect("Failed to read file");
    let length = contents.len();
    let resposne = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        result, length, contents
    );
    stream
        .write_all(resposne.as_bytes())
        .expect("Failed to write to stream");
}
