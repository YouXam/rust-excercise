use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::task;
use std;
use std::sync::Arc;
use mredis::{Command::{self, Get, Set}, Frame, Connection};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.expect("Failed to bind to port 6379");
    let db = std::collections::HashMap::new(); 
    let db = Arc::new(Mutex::new(db));
    loop {
        if let Ok((stream, _)) = listener.accept().await {
            let db = db.clone();
            task::spawn(async {
                process(stream, db).await;
            });
        }       
    }
}

async fn process(stream: TcpStream, db: Arc<Mutex<std::collections::HashMap<String, String>>>) {

    let mut connection = Connection::new(stream);
    while let Ok(frame) = connection.read_frame().await {
        connection.write_frame(&(match Command::from_frame(frame) {
            Get(cmd) => {
                if let Some(value) = db.lock().await.get(cmd.key()) {
                    Frame::Simple(value.clone())
                } else {
                    Frame::Null
                }
            },
            Set(cmd) => {
                db.lock().await.insert(cmd.key().into(), cmd.value().clone());
                Frame::Simple("OK".into())
            },
            _ => continue
        })).await.expect("Failed to write frame to socket");
    }
}
