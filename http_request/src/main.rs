use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

// https://github.com/hyperium/hyper
#[tokio::main]
async fn main() -> Result<()> {
    sample_get_request().await
}

async fn sample_get_request() -> Result<()> {
    let client = Client::new();

    // Parse an `http::Uri`...
    let uri = "http://example.com".parse()?;

    // Await the response...
    let mut res = client.get(uri).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    Ok(())
}
