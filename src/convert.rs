#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use dotenv::dotenv;
use crate::auth;
use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub async fn convert(
    user_type: &str,
    username: &str,
    date: &str,
    base_currency: &str,
    target_currency: &str,
    amount: &str,
) -> Result<String, Box<dyn Error>> {
    let password = env::var("PASSWORD").expect("PASSWORD not set");
    let login_result = auth::login(username, &password).await?;
    let cookie_value = format!("{}={}", user_type, username);

    let client = Client::new();
    let url = "https://fxdatapi.com/v1/convert";
    let payload = json!({
        "username": username,
        "date": date,
        "base_currency": base_currency,
        "target_currency": target_currency,
        "amount": amount
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Cookie", cookie_value)
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;

    if status.is_success() && login_result == "Login successful" {
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
    async fn test_convert() {
        
        setup();

        let user_type = "member";
        let username = env::var("USERNAME").expect("USERNAME not set");
        let date = "2023_04_02";
        let base_currency = "GBP";
        let target_currency = "EUR";
        let amount = "500";

        let result = convert(user_type, &username, date, base_currency, target_currency, amount).await;

        match result {
            Ok(response) => println!("Received response: {:?}", response),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}
