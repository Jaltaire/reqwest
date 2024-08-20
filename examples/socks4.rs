#![deny(warnings)]

// This is using the `tokio` runtime. You'll need the following dependency:
//
// `tokio = { version = "1", features = ["full"] }`
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Make sure you are running tor and this is your socks port
    let proxy = reqwest::Proxy::all("socks4://1.15.62.12:5678").expect("socks4 proxy should be there");
    let client = reqwest::Client::builder()
        .proxy(proxy)
        .build()
        .expect("should be able to build reqwest client");

    let req = client.get("https://httpbin.org/ip");
    let res = req.send().await?;
    println!("Status: {}", res.status());

    let text = res.text().await?;
    println!("text: {text}");

    Ok(())
}
