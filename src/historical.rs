#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use dotenv::dotenv;
use crate::auth;
use reqwest::Client;
#[allow(unused_imports)]
use serde_json::json;
use std::error::Error;

pub async fn historical(
    user_type: &str,
    username: &str,
    date: &str,
    day: &str,
    month: &str,
    year: &str,
    uuid: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let password = std::env::var("PASSWORD").expect("PASSWORD not set");
    let login_result = auth::login(username, &password).await?;
    let cookie_value = format!("{}={}", user_type, username);

    let client = Client::new();
    let base_url = "https://currensees.com/v1/historical";

    let url = match uuid {
        Some(id) => format!("{}/{}?username={}&date={}&day={}&month={}&year={}", base_url, id, username, date, day, month, year),
        None => format!("{}?username={}&date={}&day={}&month={}&year={}", base_url, username, date, day, month, year),
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
    async fn test_historical_get_all() {
        setup();

        let user_type = "member";
        let username = std::env::var("USERNAME").expect("USERNAME not set");
        let date = "2023_04_02";
        let day = "19";
        let month = "04";
        let year = "2023";
        let uuid = None;

        let result = historical(user_type, &username, date, day, month, year, uuid).await;

        match result {
            Ok(response) => println!("Received response (Get All): {:?}", response),
            Err(error) => println!("Error: {:?}", error),
        }
    }

    #[tokio::test]
    async fn test_historical_get_by_id() {
        setup();

        let user_type = "member";
        let username = std::env::var("USERNAME").expect("USERNAME not set");
        let date = "2023_04_02";
        let day = "02";
        let month = "04";
        let year = "2023";
        let uuid = Some("fe1ee1c4-d162-11ed-a2dc-acde48001122");

        let result = historical(user_type, &username, date, day, month, year, uuid).await;

        match result {
            Ok(response) => println!("Received response (Get By ID): {:?}", response),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}
