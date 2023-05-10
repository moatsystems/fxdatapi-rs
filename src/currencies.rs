#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use dotenv::dotenv;
use crate::auth;
use reqwest::Client;
#[allow(unused_imports)]
use serde_json::json;
use std::error::Error;

pub async fn currencies(
    user_type: &str,
    username: &str,
    day: &str,
    month: &str,
    year: &str,
    uuid: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let password = std::env::var("PASSWORD").expect("PASSWORD not set");
    let login_result = auth::login(username, &password).await?;
    let cookie_value = format!("{}={}", user_type, username);

    let client = Client::new();
    let base_url = "https://fxdatapi.com/v1/currencies";

    let url = match uuid {
        Some(id) => format!("{}/{}?username={}&day={}&month={}&year={}", base_url, id, username, day, month, year),
        None => format!("{}?username={}&day={}&month={}&year={}", base_url, username, day, month, year),
    };

    let response = client
        .get(&url)
        .header("Accept", "application/json")
        .header("Cookie", cookie_value)
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
    dotenv::dotenv().ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_currencies_get_all() {
        setup();

        let user_type = "member";
        let username = std::env::var("USERNAME").expect("USERNAME not set");
        let day = "19";
        let month = "04";
        let year = "2024";
        let uuid = None;

        let result = currencies(user_type, &username, day, month, year, uuid).await;

        match result {
            Ok(response) => println!("Received response (Get All): {:?}", response),
            Err(error) => println!("Error: {:?}", error),
        }
    }

    #[tokio::test]
    async fn test_currencies_get_by_id() {
        setup();

        let user_type = "member";
        let username = std::env::var("USERNAME").expect("USERNAME not set");
        let day = "19";
        let month = "04";
        let year = "2023";
        let uuid = Some("594bffc4-d095-11ed-9e30-acde48001122");

        let result = currencies(user_type, &username, day, month, year, uuid).await;

        match result {
            Ok(response) => println!("Received response (Get By ID): {:?}", response),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}
