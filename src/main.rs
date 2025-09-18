mod app;
mod coder;
mod term;
mod trace;
mod tui;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    trace::setup_tracing()?;
    trace::setup_panic_hook();
    app::App::build()?.run().await
}
