#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use dotenv::dotenv;
use crate::auth;
use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub async fn convert_all(
    user_type: &str,
    username: &str,
    base_currency: &str,
    amount: &str,
    date: &str,
) -> Result<String, Box<dyn Error>> {
    let password = env::var("PASSWORD").expect("PASSWORD not set");
    let login_result = auth::login(username, &password).await?;
    let cookie_value = format!("{}={}", user_type, username);

    let client = Client::new();
    let url = "https://currensees.com/v1/convert_all";
    let payload = json!({
        "username": username,
        "base_currency": base_currency,
        "amount": amount,
        "date": date,
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
    async fn test_convert_all() {

        setup();
        
        let user_type = "member";
        let username = env::var("USERNAME").expect("USERNAME not set");
        let base_currency = "GBP";
        let amount = "500";
        let date = "2023_04_02";

        let result = convert_all(user_type, &username, base_currency, amount, date).await;

        match result {
            Ok(response) => println!("Received response: {:?}", response),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}