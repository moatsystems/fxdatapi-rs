#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use dotenv::dotenv;
use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub async fn login(username: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let url = "https://fxdatapi.com/v1/login";
    let payload = json!({
        "username": username,
        "password": password
    });

    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;

    if status.is_success() {
        Ok(text)
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, text)))
    }
}

#[cfg(test)]
fn setup() {
    dotenv().ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_login() {
        setup();

        let username = env::var("USERNAME").expect("USERNAME not set");
        let password = env::var("PASSWORD").expect("PASSWORD not set");

        let result = login(&username, &password).await;

        match result {
            Ok(response) => println!("Received response: {:?}", response),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}