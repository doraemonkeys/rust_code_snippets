use color_eyre::{Result, eyre::eyre};
use tracing::{error, info, instrument};
use tracing_appender::rolling;
use tracing_error::ErrorLayer;

use tracing_subscriber::{
    Registry, filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

#[instrument]
fn return_err() -> Result<()> {
    Err(eyre!("Something went wrong"))
}

#[instrument]
fn call_return_err() {
    info!("going to log error");
    if let Err(err) = return_err() {
        // 推荐大家运行下，看看这里的输出效果
        error!(?err, "error");
    }
}
// 最后，再来看一个综合的例子，使用了 color-eyre 和 文件输出，前者用于为输出的日志加上更易读的颜色。
fn main() -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // 输出到控制台stderr中
    let formatting_layer = fmt::layer().pretty().with_writer(std::io::stderr);

    // 输出到文件中
    let file_name = chrono::Local::now()
        .format("app-%Y-%m-%d-%H-%M-%S.log")
        .to_string();
    println!("file_name: {}", file_name);
    let file_appender = rolling::never("logs", file_name);
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);

    // 注册
    Registry::default()
        .with(env_filter)
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        .with(ErrorLayer::default())
        .with(formatting_layer)
        .with(file_layer)
        .init();

    // 安裝 color-eyre 的 panic 处理句柄
    color_eyre::install()?;

    call_return_err();

    Ok(())
}

//TODO: https://course.rs/logs/tracing-logger.html
