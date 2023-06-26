use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::task::spawn;
use tokio::fs;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    
    while let Ok((socket, _)) = listener.accept().await {
        spawn(async move {
            handle_connection(socket).await;
        });
    }
}


async fn handle_connection(mut stream: tokio::net::TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    
    if buf_reader.read_line(&mut request_line).await.is_err() {
        return;
    }
    
    let (result, filename) = match request_line.trim() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/index.html"),
        "GET /sleep HTTP/1.1" => {
            time::sleep(Duration::from_secs(1)).await;
            ("HTTP/1.1 200 OK", "src/index.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "src/404.html"),
    };
    let contents = fs::read_to_string(filename).await.expect("Failed to read file");
    let length = contents.len();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        result, length, contents
    );
    stream.write_all(response.as_bytes()).await.expect("Fail to write to stream");
    stream.flush().await.expect("Fail to flush stream");
}
