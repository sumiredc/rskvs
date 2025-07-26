mod delete;
mod get;
mod set;

use super::command::Command;
use core::KvsEngine;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

pub async fn handle_connection(mut stream: TcpStream, db: Arc<Mutex<KvsEngine>>, addr: SocketAddr) {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);

    // ループ外でメモリを確保
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line).await {
            Ok(0) => break, // 接続が切れた
            Ok(_) => {
                // コマンドを解釈・実行
                let response = match line.parse::<Command>() {
                    // データの保存
                    Ok(Command::Set(key, value)) => set::handle(db.clone(), key, value),
                    // データの取得
                    Ok(Command::Get(key)) => get::handle(db.clone(), key),
                    // データの削除
                    Ok(Command::Delete(key)) => delete::handle(db.clone(), key),
                    // 終了
                    Ok(Command::Exit) => {
                        println!(" Connection closed by client: {}", addr);
                        break;
                    }
                    _ => "Invalid command. Use: set <key> <value> OR get <key>\n".to_string(),
                };

                // レスポンスをクライアントに送信
                if writer.write_all(response.as_bytes()).await.is_err() {
                    break;
                }

                // ループの最後で文字列を空にして、確保しているメモリをそのままに再利用する
                line.clear();
            }
            Err(_) => break, // エラー発生
        }
    }
}
