# rust_udp_utility

|test|status|
|:--:|:--:|
|Rust|[![Rust](https://github.com/motii8128/rust_udp_utility/actions/workflows/rust.yml/badge.svg)](https://github.com/motii8128/rust_udp_utility/actions/workflows/rust.yml)|

# 使用方法
Cargo.tomlに以下のように追記する
```toml
[dependencies]
rust_udp_utility = {git = "https://github.com/motii8128/rust_udp_utility"}
```

# 送信側の例
UDPによって値を送信する例を以下に示す
```rs
// 本ライブラリを使う宣言
use rust_udp_utility::UdpHandler;

fn main() {
    // 初期化する
    // ひとつめの引数にデバッグログ用の名前を入れる
    // ふたつめの引数はデバッグログを有効にするか無効にするか選ぶ
    // エラーログに関しては有無に関わらず強制的に出力する
    let mut udp = UdpHandler::new("UdpHandler", true);

    // 自動アドレス取得モードで起動する
    // 引数には受信時のタイムアウトの秒数をミリ秒で設定する
    udp.open_auto_address(1000);

    // すでに起動しているのにもう一度起動しようとしているためログを出したあと、内部で処理をスルーする
    udp.open_auto_address(1000);

    // 送信周期をミリ秒で決める。この場合は1.0秒ごとに送信する設定
    udp.set_send_period(1000);

    // 送信相手のアドレスを登録する
    udp.set_destination("192.168.11.65:64201");

    loop {
        // Helloという文字を送信する
        // 周期を守って送信するが、delayやthread::sleepのような他の処理も止めてしまうような処理は使ってない
        udp.send("Hello".as_bytes());
    }
}
```
このコードの場合以下のようにログが出る。左から実行時間、名前、状態と言ったあとに内容が出力される
```
[0][UdpHandler][INFO] Open new socket on 192.168.11.65:34329
[0][UdpHandler][WARN] socket have already opened.
[0][UdpHandler][INFO] Set send period : 1000ms
[0][UdpHandler][INFO] Set destination address : 192.168.11.65:64201
[1][UdpHandler][INFO] Send buffer : Hello 
[2][UdpHandler][INFO] Send buffer : Hello 
[3][UdpHandler][INFO] Send buffer : Hello 
[4][UdpHandler][INFO] Send buffer : Hello 
```
##### 初期化するときのtrueをfalseにすると「INFO」の部分は出力されなくなる

# 受信側の例
```rs
// 本ライブラリを使う宣言
use rust_udp_utility::UdpHandler;

fn main() {
    // 初期化
    let mut udp = UdpHandler::new("UdpReceiver", true);

    //　手動アドレスモード
    // ひとつめの引数にアドレス
    // ふたつめの引数に受信時のタイムアウトを設定
    //　受信する際に１秒までは他の処理に行かずに待つ
    udp.open_set_address("192.168.11.65:64201", 1000);

    loop {
        // 受信した内容が文字列でrecv_dataに入る
        let recv_data = udp.recv();

        //　データを送ってきた相手のアドレスを取得してfromに入れてる
        let from = udp.who();
    }
}
```
