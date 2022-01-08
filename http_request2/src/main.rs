use async_std;
use surf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[async_std::main]
async fn main() -> Result<()> {
    let mut res = surf::get("https://httpbin.org/get").await?;
    dbg!(res.body_string().await?);

    Ok(())
}
