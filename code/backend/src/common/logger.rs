//! ログ出力の共通ラッパー（必要に応じて拡張可能）


/// ロガー初期化（ファイルとコンソール両方に出力）
pub fn init_logger() {
    use flexi_logger::{Logger, Duplicate, FileSpec};
    use std::path::Path;
    // logディレクトリを作成（存在しない場合）
    let log_dir = Path::new("log");
    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir).expect("Failed to create log directory");
    }
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    Logger::try_with_str(&log_level)
        .unwrap()
        .log_to_file(FileSpec::default().directory(log_dir))
        .duplicate_to_stdout(Duplicate::All)
        .format(|w, now, record| {
            write!(w, "[{}] {}", record.level(), &record.args())
        })
        .start()
        .unwrap();
}
