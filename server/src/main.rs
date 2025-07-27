mod command;
mod handler;

use handler::handle_connection;
use rskvs_core::KvsEngine;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("🚀 KVS server listening on 127.0.0.1:8000");

    let log_path = PathBuf::from("logs/append-only.log");
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
        let (stream, addr) = listener.accept().await.unwrap();
        println!("✅️ Accepted connection from: {}", addr);

        // 新しい接続ごとに、データベースへのポインタをクローンして渡す
        let db_clone = Arc::clone(&db);

        // 新しいタスクを並列起動して、接続を処理する
        tokio::spawn(async move {
            handle_connection(stream, db_clone, addr).await;
        });
    }
}
