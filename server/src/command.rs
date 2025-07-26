use std::str::FromStr;

// コマンド定義
#[derive(Debug)]
pub enum Command {
    Set(String, String),
    Get(String),
    Delete(String),
    Exit,
}

// パースエラー用のユニット型
#[derive(Debug)]
pub struct ParseError;

// 文字列スライスからコマンドへの変換実装
impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["set", key, value] => Ok(Command::Set(key.to_string(), value.to_string())),
            ["get", key] => Ok(Command::Get(key.to_string())),
            ["delete", key] => Ok(Command::Delete(key.to_string())),
            ["exit"] => Ok(Command::Exit),
            _ => Err(ParseError),
        }
    }
}
