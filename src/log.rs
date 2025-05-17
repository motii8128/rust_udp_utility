use std::time::Instant;
use colored::Colorize;

// デバッグログ機能を追加する
pub struct Logger
{
    timer : Instant
}

impl Logger {
    /// 初期化関数
    pub fn new()->Self
    {
        Self { timer: Instant::now() }
    }

    /// 良い状態を報告する
    /// * `name` - 名前
    /// * `message` - 内容
    pub fn log_info(&self, name : &str, message : &str)
    {
        let out = format!("[{}][{}][INFO] {}", self.timer.elapsed().as_secs().to_string(), name, message);

        println!("{}", out.green());
    }

    /// 壊れはしないが、忠告したいときの報告する
    /// * `name` - 名前
    /// * `message` - 内容
    pub fn log_warn(&self, name : &str, message : &str)
    {
        let out = format!("[{}][{}][INFO] {}", self.timer.elapsed().as_secs().to_string(), name, message);

        println!("{}", out.yellow());
    }

    /// エラーを報告する
    /// * `name` - 名前
    /// * `message` - 内容
    pub fn log_error(&self, name : &str, message : &str)
    {
        let out = format!("[{}][{}][INFO] {}", self.timer.elapsed().as_secs().to_string(), name, message);

        println!("{}", out.red());
    }
}