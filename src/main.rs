use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;
use reqwest::Client;
use std::future::Future;

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;
    pub const URL_1: &str = "https://fasterthanli.me/articles/whats-in-the-box";

    info!("Building that fetch future...");
    let client = Client::new();
    let fut = fetch_thing(&client, URL_1);
    info!("Awaiting that fetch future...");
    fut.await?;
    info!("Done awaiting that fetch future");

    Ok(())
}
fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
fn fetch_thing<'a>(
    client: &'a Client,
    url: &'a str,
) -> impl Future<Output = Result<(), Report>> + 'a {
    async move {
        let res = client.get(url).send().await?.error_for_status()?;
        info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");
        Ok(())
    }
}