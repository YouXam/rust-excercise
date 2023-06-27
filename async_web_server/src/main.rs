use async_std::fs;
use async_std::io::BufReader;
use async_std::io::prelude::*;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::task;
use async_std;
use futures::StreamExt;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    /* 
    // This is the same as the while loop below， but a little bit slower. 
    listener
        .incoming()
        .for_each_concurrent(None, move |stream| async {
            if let Ok(stream) = stream {
                handle_connection(stream).await;
            }
        })
        .await;
    */
    loop {
        if let Some(Ok(stream)) = listener.incoming().next().await {
            task::spawn(async move {
                handle_connection(stream).await;
            });
        }
    }
}


async fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().await;
    let request_line = match request_line {
        Some(Ok(v)) => v,
        _ => return
    };
    let (result, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/index.html"),
        "GET /sleep HTTP/1.1" => {
            async_std::task::sleep(std::time::Duration::from_secs(1)).await;
            ("HTTP/1.1 200 OK", "src/index.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "src/404.html"),
    };
    let contents = fs::read_to_string(filename).await.expect("Failed to read file");
    let length = contents.len();
    let resposne = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        result, length, contents
    );
    stream
        .write_all(resposne.as_bytes())
        .await
        .expect("Failed to write to stream");
}