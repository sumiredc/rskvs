use core::KvsEngine;
use std::{
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

// コマンド定義
#[derive(Debug)]
enum Command {
    Set(String, String),
    Get(String),
    Exit,
}

// パースエラー用のユニット型
#[derive(Debug)]
struct ParseError;

// 文字列スライスからコマンドへの変換実装
impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["set", key, value] => Ok(Command::Set(key.to_string(), value.to_string())),
            ["get", key] => Ok(Command::Get(key.to_string())),
            ["exit"] => Ok(Command::Exit),
            _ => Err(ParseError),
        }
    }
}

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("🚀 KVS server listening on 127.0.0.1:8000");

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
    let db = Arc::new(Mutex::new(KvsEngine::new()));

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

async fn handle_connection(mut stream: TcpStream, db: Arc<Mutex<KvsEngine>>, addr: SocketAddr) {
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
                    Ok(Command::Set(key, value)) => {
                        // 排他制御してデータストアにアクセス
                        let mut db_lock = db.lock().unwrap();
                        // データを格納
                        db_lock.set(key.to_string(), value.to_string());
                        "OK\n".to_string()
                    }
                    // データの取得
                    Ok(Command::Get(key)) => {
                        // 排他制御してデータストアにアクセス
                        let db_lock = db.lock().unwrap();
                        // データを取得
                        match db_lock.get(key.to_string()) {
                            Some(value) => format!("{}\n", value),
                            None => "Key not found\n".to_string(),
                        }
                    }
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
