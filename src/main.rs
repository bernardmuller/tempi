use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://wttr.in/Cape+Town?format=1";

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("{}", body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}