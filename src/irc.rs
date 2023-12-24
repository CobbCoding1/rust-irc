use std::io::prelude::*;
use std::net::TcpStream;

pub struct Message {
    pub username: String,
    pub message: String,
}

pub struct Client {
    stream: TcpStream,
    server: String,
}

#[derive(Debug)]
pub enum ClientErrors {
    ConnectionError,
    WriteError,
    ReadError,
    FlushError,
    ConversionError,
}

impl std::fmt::Display for ClientErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::ConnectionError => "Could not connect",
            Self::WriteError      => "Could not write to stream",
            Self::ReadError       => "Could not read from stream",
            Self::FlushError      => "Could not flush stream",
            Self::ConversionError => "Could not convert message",
        })
    }
}

impl Client {
    pub fn new(address: &str, port: &str, server: &str) -> Result<Client, ClientErrors> {
        if let Ok(stream) = TcpStream::connect(format!("{}:{}", address, port)) {
            Ok(Client {
                stream: stream,
                server: server.to_string(),
            })
        } else {
            Err(ClientErrors::ConnectionError)
        }
    }

    pub fn auth(&mut self, nickname: &str, password: &str) -> Result<(), ClientErrors> {
        self.say(&format!("PASS {}\r\n", password))?;
        self.say(&format!("NICK {}\r\n", nickname))
    }

    pub fn handle_ping(&mut self, str: &str) -> Result<(), ClientErrors> {
        let output = str.strip_prefix("PING ").ok_or(ClientErrors::ConversionError)?;
        self.say(&format!("PONG {}", output))?;
        println!("PONG {output}");
        Ok(())
    }

    pub fn read_message(&mut self) -> Result<Option<Message>, ClientErrors> {
        let mut buf = [0 as u8; 128];
        self.stream.read(&mut buf).map_err(|_| ClientErrors::ReadError)?;
        let str = std::str::from_utf8(&buf).map_err(|_| ClientErrors::ConversionError)?;
        if str.contains("PRIVMSG") {
            let msg = self.parse_message(str.to_string());
            return Ok(Some(msg));
        } else if str.starts_with("PING") {
            self.handle_ping(str)?;
        }
        Ok(None)
    }

    pub fn join(&mut self) -> Result<(), ClientErrors> {
        self.say(&format!("JOIN #{}", self.server))
    }

    pub fn say(&mut self, message: &str) -> Result<(), ClientErrors> {
        self.stream.write(format!("{}\r\n", message).as_bytes()).map_err(|_| ClientErrors::WriteError)?;
        self.stream.flush().map_err(|_| ClientErrors::WriteError)?;
        Ok(())
    }


    pub fn private_message(&mut self, message: &str) -> Result<(), ClientErrors> {
        self.say(&format!("PRIVMSG #{} :{}", self.server, message))
    }

    fn parse_message(&mut self, buf: String) -> Message {
        let messagefirst: Vec<&str> = buf.split(":").collect();
        let messagesecond: Vec<&str> = messagefirst[1].split("!").collect();
        let username = messagesecond[0];
        let message = messagefirst[2];
        Message {
            username: username.to_string(),
            message: message.to_string(),
        }
    }
}

