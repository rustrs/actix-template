use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use tracing::Level;
use tracing_subscriber::{
    fmt::{format::FmtSpan, time::ChronoLocal}, layer::SubscriberExt, EnvFilter, Layer, Registry
};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use once_cell::sync::Lazy;
use parking_lot::Mutex;

#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct LogConfig {
    pub level: String,
    pub dir: String,
    pub file: String,
    pub rotation: String, // "hourly" or "daily"
    pub max_days: usize,
    pub console: bool, // 是否在控制台输出日志
}
static LOGGER_GUARD: Lazy<Mutex<Option<tracing_appender::non_blocking::WorkerGuard>>> =
    Lazy::new(|| Mutex::new(None));

/// 初始化日志模块
pub fn init_logger(cfg: &LogConfig) {
    let level = cfg.level.parse::<Level>().unwrap_or(Level::INFO);
    let filter_file = EnvFilter::from_default_env().add_directive(level.into());
    let filter_console = EnvFilter::from_default_env().add_directive(level.into());
    // 创建日志目录
    if !Path::new(&cfg.dir).exists() {
        fs::create_dir_all(&cfg.dir).expect("Failed to create log directory");
    }

    // 滚动策略
    let rotation = match cfg.rotation.as_str() {
        "hourly" => Rotation::HOURLY,
        "daily" => Rotation::DAILY,
        _ => Rotation::DAILY,
    };

    let file_appender = RollingFileAppender::new(rotation, &cfg.dir, &cfg.file);
    //let (non_blocking_file, _guard_file) = tracing_appender::non_blocking(file_appender);
    let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);
    *LOGGER_GUARD.lock() = Some(guard); // 防止日志线程退出

    let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string());

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking_file)
        .with_timer(timer)
        .json()
        .with_span_events(FmtSpan::CLOSE)
        .with_filter(filter_file);

    // 控制台输出（可选）
    let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string());

    if cfg.console {
        let console_layer = tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_timer(timer)
            .pretty()
            .with_filter(filter_console);

        let subscriber = Registry::default()
            .with(file_layer)
            .with(console_layer);
        
        tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");
    } else {
        let subscriber = Registry::default().with(file_layer);
        tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");
    }
}