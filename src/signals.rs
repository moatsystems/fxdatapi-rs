#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use dotenv::dotenv;
use crate::auth;
use reqwest::Client;
use std::error::Error;

pub async fn signals(
    user_type: &str,
    username: &str,
    uuid: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let password = std::env::var("PASSWORD").expect("PASSWORD not set");
    let login_result = auth::login(username, &password).await?;
    let cookie_value = format!("{}={}", user_type, username);

    let client = Client::new();
    let base_url = "https://fxdatapi.com/v1/signals";

    let url = match uuid {
        Some(id) => format!("{}/{}?username={}", base_url, id, username),
        None => format!("{}?username={}", base_url, username),
    };

    let response = client
        .get(&url)
        .header("Content-Type", "application/json")
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
    async fn test_signals_get_all() {
        setup();

        let user_type = "member";
        let username = std::env::var("USERNAME").expect("USERNAME not set");
        let uuid = None;

        let result = signals(user_type, &username, uuid).await;

        match result {
            Ok(response) => println!("Received response (Get All): {:?}", response),
            Err(error) => println!("Error: {:?}", error),
        }
    }

    #[tokio::test]
    async fn test_signals_get_by_id() {
        setup();

        let user_type = "member";
        let username = std::env::var("USERNAME").expect("USERNAME not set");
        let uuid = Some("d46cc05a-e3c6-11ed-8570-acde48001122");

        let result = signals(user_type, &username, uuid).await;

        match result {
            Ok(response) => println!("Received response (Get By ID): {:?}", response),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}
