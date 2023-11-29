use std::io::prelude::*;
use std::io::{BufWriter, BufReader};
use std::net::TcpStream;

struct Message {
    username: String,
    message: String,
}

fn read_file(filename: &str) -> String { 
    let data = std::fs::read_to_string(filename).unwrap();
    data
}

fn parse_message(mut buf: String) -> Message {
    let messagefirst: Vec<&str> = buf.split(":").collect();
    let messagesecond: Vec<&str> = messagefirst[1].split("!").collect();
    let username = messagesecond[0];
    let message = messagefirst[2];
    Message {
        username: username.to_string(),
        message: message.to_string(),
    }
}

fn main() {
    if let Ok(mut stream) = TcpStream::connect("irc.twitch.tv:6667") {
        let client = "PASS ".to_string() + &read_file(".client") + "\r\n";
        let mut writer = BufWriter::new(stream.try_clone().unwrap());
        let mut reader = BufReader::new(stream.try_clone().unwrap());

        stream.write(client.as_bytes()).unwrap();
        stream.flush().unwrap();
        stream.write(b"NICK cobbbot\r\n").unwrap();
        stream.flush().unwrap();
        stream.write(b"JOIN #cobbcoding\r\n").unwrap();
        stream.flush().unwrap();

        println!("Connected");
        loop {
            let mut buf = String::new();
            let got = reader.read_line(&mut buf).unwrap();
            if buf.contains("PRIVMSG") {
                let msg = parse_message(buf);
                if msg.message.chars().nth(0).unwrap() == '!' {
                    println!("{} send the command {}", msg.username, msg.message);
                }
            }
        }
    } else {
        println!("Could not connect");
    }
}
