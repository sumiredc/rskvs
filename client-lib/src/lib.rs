use thiserror::Error;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Server returned an error: {0}")]
    Server(String),
}

type Result<T> = std::result::Result<T, ClientError>;

pub struct KvsClient {
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
}

impl KvsClient {
    pub async fn connect(addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        let (reader, writer) = stream.into_split();

        Ok(Self {
            reader: BufReader::new(reader),
            writer,
        })
    }

    pub async fn set(&mut self, key: &str, value: &str) -> Result<String> {
        let command = format!("set {} {}\n", key, value);
        self.writer.write_all(command.as_bytes()).await?;
        self.read_response().await
    }

    pub async fn get(&mut self, key: &str) -> Result<String> {
        let command = format!("get {}\n", key);
        self.writer.write_all(command.as_bytes()).await?;
        self.read_response().await
    }

    pub async fn delete(&mut self, key: &str) -> Result<String> {
        let command = format!("delete {}\n", key);
        self.writer.write_all(command.as_bytes()).await?;
        self.read_response().await
    }

    async fn read_response(&mut self) -> Result<String> {
        let mut response = String::new();
        self.reader.read_line(&mut response).await?;

        if response.starts_with("Error:") {
            Err(ClientError::Server(response.trim().to_string()))
        } else {
            Ok(response.trim().to_string())
        }
    }
}
