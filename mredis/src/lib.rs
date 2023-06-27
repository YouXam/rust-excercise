

use bytes::{self, Bytes};
use tokio::{io::{BufReader, AsyncBufReadExt, AsyncWriteExt}, net::TcpStream};

#[derive(Debug)]
pub struct Get {
    key: String,
}

impl Get {
    pub fn key(&self) -> &str {
        &self.key
    }
}

#[derive(Debug)]
pub struct Set {
    key: String,
    value: String,
}

impl Set {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}


#[derive(Debug)]
pub enum Command {
    Get(Get),
    Set(Set),
    Other
}

impl Command {
    pub fn from_frame(frame: Frame) -> Self {
        match frame {
            Frame::Array(frames) => {
                match &frames[..] {
                    [Frame::Simple(cmd), Frame::Simple(key)] => {
                        Some(cmd.clone())
                            .filter(|x| x == "get")
                            .map(|_| Command::Get(Get { 
                                key: key.clone()
                            }))
                            .unwrap_or(Command::Other)
                    }
                    [Frame::Simple(cmd), Frame::Simple(key), Frame::Simple(value)] => {
                        Some(cmd.clone())
                            .filter(|x| x == "set")
                            .map(|_| Command::Set(Set {
                                key: key.clone(),
                                value: value.clone()
                            }))
                            .unwrap_or(Command::Other)
                    },
                    _ => Command::Other
                }
            },
            _ => Command::Other
        }
    }
}

#[derive(Debug)]
pub enum Frame {
    Simple(String),
    Bulk(Bytes),
    Array(Vec<Frame>),
    Null
}


pub struct Connection {
    reader: BufReader<tokio::net::tcp::OwnedReadHalf>,
    writer: tokio::net::tcp::OwnedWriteHalf,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        let (reader, writer) = stream.into_split();
        Connection {
            reader: BufReader::new(reader),
            writer,
        }
    }
    
    async fn read_line(&mut self) -> Result<String, std::io::Error> {
        let mut buf = String::new();
        self.reader.read_line(&mut buf).await?;
        Ok(buf)
    }

    pub async fn read_frame(&mut self) -> Result<Frame, std::io::Error> {
        let length = self.read_line().await?;
        if !length.starts_with("*") {
            return Ok(Frame::Null);
        }
        let length = length.trim_start_matches("*").trim_end().parse::<usize>();
        if length.is_err() {
            return Ok(Frame::Null);
        }
        let length = length.unwrap();
        let mut frames = Vec::with_capacity(length);
        for i in 0..length*2 {
            if i % 2 == 0 {
                self.read_line().await?;
            } else {
                frames.push(Frame::Simple(self.read_line().await?.trim().to_string()));
            }
        }
        Ok(Frame::Array(frames))
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> Result<(), ()> {
        match frame {
            Frame::Simple(s) => {
                self.writer.write_all(format!("+{}\r\n", s).as_bytes()).await.map_err(|_| ())?;
            },
            Frame::Null => {
                self.writer.write_all(b"$-1\r\n").await.map_err(|_| ())?;
            },
            _ => return Err(())
        }
        self.writer.flush().await.map_err(|_| ())?;
        Ok(())
    }
}