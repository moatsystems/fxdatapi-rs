#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use dotenv::dotenv;
use crate::auth;
use reqwest::Client;
use std::error::Error;

pub async fn monthly_average(
    user_type: &str,
    username: &str,
    year: &str,
    month: &str,
) -> Result<String, Box<dyn Error>> {
    let password = env::var("PASSWORD").expect("PASSWORD not set");
    let login_result = auth::login(username, &password).await?;
    let cookie_value = format!("{}={}; {}={}", user_type, username, "username", username);

    let client = Client::new();
    let url = format!("https://fxdatapi.com/v1/monthly_average/{}/{}", year, month);

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
    async fn test_monthly_average() {
        
        setup();

        let user_type = "member";
        let username = env::var("USERNAME").expect("USERNAME not set");
        let year = "2023";
        let month = "04";

        let result = monthly_average(user_type, &username, year, month).await;

        match result {
            Ok(response) => println!("Received response: {:?}", response),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}
