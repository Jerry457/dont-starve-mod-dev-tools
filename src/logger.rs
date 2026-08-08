use std::path::PathBuf;

use flexi_logger::{Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming, WriteMode};

fn get_log_dir() -> PathBuf {
    let log_dir = dirs::data_local_dir() // 返回 Option<PathBuf>
        .unwrap_or_else(|| PathBuf::from(".")) // 降级安全处理
        .join(env!("CARGO_PKG_NAME"))
        .join("logs");

    std::fs::create_dir_all(&log_dir).ok();
    log_dir
}

pub fn start() -> anyhow::Result<()> {
    Logger::try_with_env_or_str("info")?
        .duplicate_to_stdout(Duplicate::All)
        .log_to_file(FileSpec::default().directory(get_log_dir()))
        .write_mode(WriteMode::BufferAndFlush)
        .rotate(
            Criterion::Size(10_000_000),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(5),
        )
        .start()?;
    Ok(())
}
