use std::{
    error::Error,
    io::{Write, stdin, stdout},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TCP サーバーへ非同期で接続を試みる
    let mut stream = TcpStream::connect("127.0.0.1:8000").await?;
    println!("✅️ Connect to KVS server.");

    let (reader, mut writer) = stream.split();
    // バッファ付きリーダーを生成
    // 事前にある程度の量をまとめてメモリ上のバッファに読み込むことで、システムコールの回数を減らす
    let mut reader = BufReader::new(reader);

    // ループ外でメモリを確保
    let mut server_response = String::new();

    loop {
        print!("> ");
        // ☝️ の print!() は改行を含まないため直ぐに表示されない可能性があるので
        // flush() でバッファを強制的に空にして、"> " をすぐに画面出力させる
        stdout().flush()?;

        // ユーザー入力の受け取り
        // read_line() プログラムの実行を一時停止して、ユーザーが入力して Enter を押すのを待つ
        // Enter が押されたら、改行を含んだ文字列が user_input に格納される
        let mut user_input = String::new();
        stdin().read_line(&mut user_input)?;

        // 入力が改行のみの場合は、何もせずループの先頭に戻る
        if user_input.trim().is_empty() {
            continue;
        }

        if user_input.trim() == "exit" {
            println!("👋 Exiting.");
            // サーバーへ exit コマンドを送信して終了
            writer.write_all(user_input.as_bytes()).await?;
            break;
        }

        // ユーザー入力をサーバーに送信
        writer.write_all(user_input.as_bytes()).await?;

        // サーバーからの応答を読み取って表示
        server_response.clear();
        reader.read_line(&mut server_response).await?;

        // サーバーが応答なく接続を閉じた際に、0 が返ってくるのでループを抜ける
        if server_response.is_empty() {
            println!("\n 🔌 Connection closed by server.");
            break;
        }

        println!("{}", server_response);
    }

    Ok(())
}
