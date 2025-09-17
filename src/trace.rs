use std::{env, fs, panic, path::PathBuf, process, sync::LazyLock};

use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt};

use crate::term::Terminal;

static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    match env::var(format!(
        "{}_DATA_DIR",
        env!("CARGO_PKG_NAME").to_uppercase()
    )) {
        Ok(path) => PathBuf::from(path),
        Err(_) => directories::BaseDirs::new()
            .map(|bd| bd.data_dir().join(env!("CARGO_PKG_NAME")))
            .unwrap_or_else(|| PathBuf::from(".")),
    }
});
static LOG_FILE: LazyLock<PathBuf> =
    LazyLock::new(|| DATA_DIR.join(format!("{}.log", env!("CARGO_PKG_NAME"))));

pub fn setup_panic_hook() {
    let panic_hook = better_panic::Settings::default()
        .message(format!(
            "This may be a critical error. Consider reporting it at {}",
            env!("CARGO_PKG_REPOSITORY")
        ))
        .backtrace_first(false)
        .create_panic_handler();

    panic::set_hook(Box::new(move |info| {
        if let Err(err) = Terminal::build().map(|mut term| term.exit()) {
            tracing::error!("Error transitioning terminal to normal mode: {err}");
        }

        let payload = info.payload();
        let payload = payload
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| payload.downcast_ref::<&str>().cloned())
            .unwrap_or("<non string panic payload>");

        tracing::error!("Application crashed: {payload}");
        panic_hook(info);
        process::exit(1);
    }));
}

pub fn setup_tracing() -> anyhow::Result<()> {
    fs::create_dir_all(&*DATA_DIR)?;
    let file = fs::File::create(&*LOG_FILE)?;

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(ErrorLayer::default())
            .with(
                tracing_subscriber::fmt::layer()
                    .with_ansi(false)
                    .with_writer(file)
                    .with_file(true)
                    .with_line_number(false)
                    .with_target(false)
                    .with_filter(EnvFilter::from_default_env()),
            ),
    )?;

    Ok(())
}
