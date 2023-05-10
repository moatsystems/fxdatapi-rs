#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use dotenv::dotenv;
use crate::auth;
use reqwest::Client;
use std::error::Error;

pub async fn weekly_average(
    user_type: &str,
    username: &str,
    from_date: &str,
    to_date: &str,
) -> Result<String, Box<dyn Error>> {
    let password = env::var("PASSWORD").expect("PASSWORD not set");
    let login_result = auth::login(username, &password).await?;
    let cookie_value = format!("{}={}; {}={}", user_type, username, "username", username);

    let client = Client::new();
    let url = format!("https://fxdatapi.com/v1/weekly_average/{}/{}", from_date, to_date);

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
    async fn test_weekly_average() {
        
        setup();

        let user_type = "member";
        let username = env::var("USERNAME").expect("USERNAME not set");
        let from_date = "2023_04_03";
        let to_date = "2023_04_07";

        let result = weekly_average(user_type, &username, from_date, to_date).await;

        match result {
            Ok(response) => println!("Received response: {:?}", response),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}
