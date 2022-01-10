#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/get")
        .await?
        .text()
        .await?;
    println!("{:#?}", resp);

    let client = reqwest::Client::new();
    let res = client
        .post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await?;
    println!("{:#?}", res);

    Ok(())
}
