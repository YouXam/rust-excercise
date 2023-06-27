use bytes::{self, Bytes};

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
    value: bytes::Bytes
}

impl Set {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &Bytes {
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
                    [Frame::Bulk(cmd), Frame::Bulk(key)] => {
                        if String::from_utf8(cmd.to_vec()).unwrap() == "get" {
                            Command::Get(Get {
                                key: String::from_utf8(key.to_vec()).unwrap()
                            })
                        } else {
                            Command::Other
                        }
                    }
                    [Frame::Bulk(cmd), Frame::Bulk(key), Frame::Bulk(value)] => {
                        if String::from_utf8(cmd.to_vec()).unwrap() == "set" {
                            Command::Set(Set {
                                key: String::from_utf8(key.to_vec()).unwrap(),
                                value: value.clone()
                            })
                        } else {
                            Command::Other
                        }
                    },
                    _ => Command::Other
                }
            },
            _ => Command::Other
        }
    }
}

pub enum Frame {
    Simple(String),
    Bulk(Bytes),
    Array(Vec<Frame>),
    Null
}


pub struct Connection {

}

impl Connection {
    pub fn new(stream: tokio::net::TcpStream) -> Self {
        Connection {}
    }
    pub async fn read_frame(&mut self) -> Result<Option<Frame>, ()> {
        Ok(None)
    }
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<(), ()> {
        Ok(())
    }
}