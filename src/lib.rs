mod log;

use std::net::UdpSocket;
use std::time::{Duration, Instant};

use log::Logger;

/// UDP通信をサポートする
/// * `name` - ログ用の名前
/// * `socket` - UDPソケット。Optionなのでソケットが作られてないなら`None`が入る
/// * `enagle_debug_log` - trueならデバッグログを出力する
/// * `logger` - ログ出力をする
/// * `timer` - 時間を管理する
/// * `destination_addr` - 送信先のアドレスを登録する
/// * `recv_destination_addr` - 受信した際に送ってきた相手のアドレスを格納する。受信してない場合は`None`
pub struct UdpHandler
{
    name : String,
    socket : Option<UdpSocket>,
    enable_debug_log : bool,
    logger : log::Logger,
    timer : Instant,
    period : Duration,
    destination_addr : String,
    recv_destination_addr : Option<String>
}

impl UdpHandler {
    /// 初期化
    /// * `enable_debug` - trueならデバッグログを有効にする
    pub fn new(name : &str, enable_debug : bool)->Self
    {
        Self {
            name : name.to_string(), 
            socket: None, 
            enable_debug_log: enable_debug, 
            logger: Logger::new(), 
            timer: Instant::now(), 
            period : Duration::from_millis(1),
            destination_addr: String::from("None"),
            recv_destination_addr : None
        }
    }

    /// ローカルホストでソケットを作成
    /// * `port_num` - ポート番号を指定する 0~65535
    /// * `period` - 受信のタイムアウトを設定する。例えば`1`にすると受信待機してから１ｍｓ受信できないとエラーを吐くようにする
    pub fn open_localhost(&mut self, port_num : u16, period : u64)
    {
        if self.socket.is_some()
        {
            self.logger.log_warn(&self.name, "socket have already opened.");
            return;
        }

        let addr = format!("127.0.0.1:{}", port_num);

        match UdpSocket::bind(addr.as_str())
        {
            Ok(socket)=>{
                if self.enable_debug_log 
                {
                    let msg = format!("Open new socket on {} .", addr);
                    self.logger.log_info(&self.name, msg.as_str());
                }
                socket.set_read_timeout(Some(Duration::from_millis(period))).unwrap();

                self.socket = Some(socket);
            }
            Err(e)=>{
                self.logger.log_error(&self.name, "Failed to create new socket on localhost");
                self.logger.log_error(&self.name, e.to_string().as_str());
            }
        }
    }

    /// 自動でアドレスを割り当ててソケットを作成
    /// * `period` - 受信のタイムアウトを設定する。例えば`1`にすると受信待機してから１ｍｓ受信できないとエラーを吐くようにする
    pub fn open_auto_address(&mut self, period : u64)
    {
        if self.socket.is_some()
        {
            self.logger.log_warn(&self.name, "socket have already opened.");
            return;
        }

        match UdpSocket::bind("0.0.0.0:0") {
            Ok(socket)=>{
                socket.connect("8.8.8.8:80").unwrap();
                if self.enable_debug_log
                {
                    let msg = format!("Open new socket on {}", socket.local_addr().unwrap());
                    self.logger.log_info(&self.name, msg.as_str());
                }

                socket.set_read_timeout(Some(Duration::from_millis(period))).unwrap();
                self.socket = Some(socket)
            }
            Err(e)=>{
                self.logger.log_error(&self.name, "Failed to open socket");
                self.logger.log_error(&self.name, e.to_string().as_str());
            }
        }
    }

    /// 設定したアドレスでソケットを作成する
    /// * `addr` - ポートもまとめたアドレス。例えば「192.168.0.50:64202」と入力する
    /// * `period` - 受信のタイムアウトを設定する。例えば`1`にすると受信待機してから１ｍｓ受信できないとエラーを吐くようにする
    pub fn open_set_address(&mut self, addr : &str, period : u64)
    {
        if self.socket.is_some()
        {
            self.logger.log_warn(&self.name, "socket have already opened.");
            return;
        }

        match UdpSocket::bind(addr) {
            Ok(socket)=>{
                if self.enable_debug_log
                {
                    let msg = format!("Open new socket on {} ", addr);
                    self.logger.log_info(&self.name, msg.as_str());
                }
                socket.set_read_timeout(Some(Duration::from_millis(period))).unwrap();
                self.socket = Some(socket)
            }
            Err(e)=>{
                self.logger.log_error(&self.name, "Failed to open socket");
                self.logger.log_error(&self.name, e.to_string().as_str());
            }
        }
    }

    /// 送信相手のアドレスを登録する
    /// * `addr` - 相手のアドレス。例えば「192.168.0.50:64201」
    pub fn set_destination(&mut self, addr : &str)
    {
        if self.enable_debug_log
        {
            let msg = format!("Set destination address : {}", addr);
            self.logger.log_info(&self.name, msg.as_str());
        }
        self.destination_addr = addr.to_string();
    }

    /// 送信周期を決める
    /// * `period` - ミリ秒で設定する。例えば1msごとに送信したい場合は`1`を入れる
    pub fn set_send_period(&mut self, period : u64)
    {
        if self.enable_debug_log
        {
            let msg = format!("Set send period : {}ms", period);
            self.logger.log_info(&self.name, msg.as_str());
        }
        self.period = Duration::from_millis(period);
    }


    /// 登録したアドレスに値を送信する
    /// * `buf` - 8bitの正の整数の配列のポインタ
    pub fn send(&mut self, buf : &[u8])
    {
        if self.timer.elapsed() >= self.period
        {
            match &self.socket {
                Some(sock)=>{
                    match sock.send_to(buf, self.destination_addr.as_str()) {
                        Ok(_s)=>{
                            if self.enable_debug_log
                            {
                                let msg = format!("Send buffer : {} ", String::from_utf8_lossy(&buf).to_string());
                                self.logger.log_info(&self.name, msg.as_str());
                            }
                        }
                        Err(e)=>{
                            self.logger.log_error(&self.name, "Failed to send buffer.");
                            self.logger.log_error(&self.name, e.to_string().as_str());
                        }
                    }
                }
                None=>{
                    self.logger.log_warn(&self.name, "Socket is not opened.");
                }
            }

            self.timer = Instant::now();
        }
    }

    pub fn recv(&mut self)->String
    {
        let mut buf = [0_u8; 1024];
        match &self.socket {
            Some(socket)=>{
                match socket.recv_from(&mut buf) {
                    Ok((size, dest_addr))=>{
                        let get_data = &buf[..size];
                        
                        self.recv_destination_addr = Some(dest_addr.to_string());

                        let str = String::from_utf8_lossy(&get_data).to_string();

                        if self.enable_debug_log
                        {
                            let msg = format!("Receive : {}", str.clone());
                            self.logger.log_info(&self.name, msg.as_str());
                        }

                        str
                    }
                    Err(e)=>{
                        self.logger.log_error(&self.name, "Failed to recv value.");
                        self.logger.log_error(&self.name, e.to_string().as_str());

                        String::from("None")
                    }
                }
            }
            None=>{
                self.logger.log_warn(&self.name, "Socket is not opened.");

                String::from("None")
            }
        }
    }

    /// データを送ってきた相手のアドレスを取得する。
    /// 受信後でないと`None`が返ってくる
    pub fn who(&self)->String
    {
        if self.recv_destination_addr.is_some()
        {
            self.recv_destination_addr.clone().unwrap()
        }
        else {
            String::from("None")
        }
    }
}