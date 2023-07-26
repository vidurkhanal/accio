#[derive(Debug, Clone)]
pub enum Command {
    Ping,
    Get(String),
    Del(String),
    Set(String, String),
    Invalid,
}

impl Command {
    pub fn from_buffer(words: Vec<String>) -> Self {
        // println!("[debug] Received words: {:?}", words);
        match words.len() {
            1 => match words[0].as_str() {
                "ping" => Command::Ping,
                _ => Command::Invalid,
            },
            2 => match words[0].as_str() {
                "del" => Command::Del(words[1].clone()),
                "get" => Command::Get(words[1].clone()),
                _ => Command::Invalid,
            },
            3 => Command::Set(words[1].clone(), words[2].clone()),
            _ => Command::Invalid,
        }
    }
}
