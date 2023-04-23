#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use dotenv::dotenv;
use crate::auth;
use reqwest::Client;
use std::error::Error;

pub async fn margins_spreads(
    user_type: &str,
    username: &str,
    day: &str,
    month: &str,
    year: &str,
    uuid: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let password = env::var("PASSWORD").expect("PASSWORD not set");
    let login_result = auth::login(username, &password).await?;
    let cookie_value = format!("{}={}; {}={}", user_type, username, "username", username);

    let client = Client::new();
    let base_url = "https://currensees.com/v1/margins_spreads";
    let url = match uuid {
        Some(id) => format!("{}/{}/?username={}&day={}&month={}&year={}", base_url, id, username, day, month, year),
        None => format!("{}?username={}&day={}&month={}&year={}", base_url, username, day, month, year),
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
    dotenv().ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_margins_spreads() {
        
        setup();

        let user_type = "member";
        let username = env::var("USERNAME").expect("USERNAME not set");
        let day = "19";
        let month = "04";
        let year = "2023";
        let uuid = Some("margins_spreads_uuid");

        let result = margins_spreads(user_type, &username, day, month, year, uuid).await;

        match result {
            Ok(response) => println!("Received response: {:?}", response),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}
