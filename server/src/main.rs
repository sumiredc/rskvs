mod command;
mod handler;

use dotenvy::dotenv;
use handler::handle_connection;
use rskvs_core::KvsEngine;
use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .env を読み込み
    dotenv().ok();

    let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| String::new());
    let log_path_str = env::var("LOG_PATH").unwrap_or_else(|_| String::new());

    println!("... Try to listening on {}", listen_addr);
    let listener = TcpListener::bind(&listen_addr).await?;
    println!("🚀 KVS server listening on {}", listen_addr);

    println!("📝 Append Only Logs loading to {}", log_path_str);
    let log_path = PathBuf::from(log_path_str);
    let engine = KvsEngine::new(log_path)?;

    // Arc:
    //  あるデータへの所有権を複数の場所で共有できるようにするスマートポインタ
    //  Arc::clone() すると自動的に参照カウントして、スコープを抜けるとカウントが減る
    //  カウントが 0 になると、メモリから開放される
    //
    // Mutex:
    //  相互排他の機構、データへのアクセスを保護するロック
    //  アクセス前に lock() を呼び出し、鍵を取得（他のタスクがロック中なら待機）
    //  ロック成功時に MutexGuard が返ってきて、安全にデータへアクセスできる
    //  MutexGuard がスコープを抜けるとロックが自動的に開放される
    let db = Arc::new(Mutex::new(engine));

    loop {
        // 新しいクライアントからの TCP 接続を待ち受ける処理
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("✅️ Accepted connection from: {}", addr);

                // 新しい接続ごとに、データベースへのポインタをクローンして渡す
                let db_clone = Arc::clone(&db);

                // 新しいタスクを並列起動して、接続を処理する
                tokio::spawn(async move {
                    handle_connection(stream, db_clone, addr).await;
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
