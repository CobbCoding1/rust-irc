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

pub enum ClientErrors {
    ConnectionError,
    NoMessageError,
    WriteError,
    ReadError,
    FlushError,
    ConversionError,
    NoError,
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

    pub fn auth(&mut self, nickname: &str, password: &str) {
        match self.say(&format!("PASS {}\r\n", password)) {
            ClientErrors::NoError => {
                let nick = format!("NICK {}\r\n", nickname);
                match self.say(&nick) {
                    ClientErrors::NoError => ClientErrors::NoError,
                    e => e,
                }
            },
            e => e,
        };
    }

    pub fn read_message(&mut self) -> Result<Message, ClientErrors> {
        let mut buf = [0 as u8; 128];
        match self.stream.read(&mut buf) {
            Ok(_) => {
                if let Ok(str) = std::str::from_utf8(&buf) {
                    if str.contains("PRIVMSG") {
                        let msg = self.parse_message(str.to_string());
                        return Ok(msg);
                    }
                    Err(ClientErrors::NoMessageError)
                } else {
                    Err(ClientErrors::ConversionError)
                }
            },
            Err(_) => Err(ClientErrors::ReadError)
        }
    }

    pub fn join(&mut self) -> ClientErrors {
        match self.say(&format!("JOIN #{}", self.server)) {
            ClientErrors::NoError => ClientErrors::NoError,
            e => e,
        }
    }

    pub fn say(&mut self, message: &str) -> ClientErrors {
        match self.stream.write(format!("{}\r\n", message).as_bytes()) {
            Ok(_) => {
                match self.stream.flush() {
                    Ok(_) => return ClientErrors::NoError,
                    Err(_) => return ClientErrors::FlushError,
                }
            },
            Err(_) => return ClientErrors::WriteError,
        };
    }


    pub fn private_message(&mut self, message: &str) -> ClientErrors {
        match self.say(&format!("PRIVMSG #{} :{}", self.server, message)) {
            ClientErrors::NoError => ClientErrors::NoError,
            e => e,
        }
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

