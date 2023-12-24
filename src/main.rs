use rustirc::{Client, ClientErrors};

fn read_file(filename: &str) -> String { 
    let data = std::fs::read_to_string(filename).expect("could not read from file");
    data
}

fn run() -> Result<(), ClientErrors> {
    let password = read_file(".client");
    let mut client = Client::new("irc.twitch.tv", "6667", "cobbcoding")?;
    client.auth("cobbbot", &password)?;
    client.join()?;
    loop {
        if let Some(msg) = client.read_message()? {
            let commands: Vec<&str> = msg.message.split_whitespace().collect();
            match commands[0] {
                "!help" => {
                    client.private_message("helped")?;
                },
                "!say" => {
                    client.private_message(&commands[1..commands.len()].join(" "))?;
                },
                _ => {
                    if commands[0].chars().nth(0) == Some('!') {
                        client.private_message(&format!("Unknown command {}", commands[0]))?;
                    } 
                },
            }
        }
    }
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
