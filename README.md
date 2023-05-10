## FXDatapi

[![Crates.io][crates-badge]][crates-url]
[![Build Status][ci-badge]][ci-url]

[crates-badge]: https://img.shields.io/crates/v/fxdatapi
[crates-url]: https://crates.io/crates/fxdatapi
[ci-badge]: https://img.shields.io/github/actions/workflow/status/moatsystems/fxdatapi-rs/ci.yml?branch=main
[ci-url]: https://github.com/moatsystems/fxdatapi-rs/actions

A [Rust](https://www.rust-lang.org/) library to interact with the [Currency API](https://moatsystems.com/currency-api/).

### Usage

Add this library as a dependency in your `Cargo.toml`:

```toml
[dependencies]
fxdatapi = "0.1.4"
```

#### Authentication

```rs
use fxdatapi::auth;

// Currency API Authentication
async fn main() {
    let username = "your_username";
    let password = "your_password";

    let result = auth::login(username, password).await;

    match result {
        Ok(response) => println!("Received response: {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}
```

#### Currencies

```rs
use fxdatapi::currencies;

// Get All Currencies
async fn currencies_get_all() {
    let user_type = "member";
    let username = "your_username";
    let day = "19";
    let month = "04";
    let year = "2024";
    let uuid = None;

    let result = currencies(user_type, username, day, month, year, uuid).await;

    match result {
        Ok(response) => println!("Received response (Get All): {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}

// Get Currency By ID
async fn currencies_get_by_id() {
    let user_type = "member";
    let username = "your_username";
    let day = "19";
    let month = "04";
    let year = "2023";
    let uuid = Some("currency_uuid");

    let result = currencies(user_type, username, day, month, year, uuid).await;

    match result {
        Ok(response) => println!("Received response (Get By ID): {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}
```

#### Historical

```rs
use fxdatapi::historical;

// Get All Historical Data
async fn historical_get_all() {
    let user_type = "member";
    let username = "your_username";
    let date = "2023_04_02";
    let day = "19";
    let month = "04";
    let year = "2023";
    let uuid = None;

    let result = historical(user_type, username, date, day, month, year, uuid).await;

    match result {
        Ok(response) => println!("Received response (Get All): {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}

// Get Historical Data By ID
async fn historical_get_by_id() {
    let user_type = "member";
    let username = "your_username";
    let date = "2023_04_02";
    let day = "02";
    let month = "04";
    let year = "2023";
    let uuid = Some("historical_uuid");

    let result = historical(user_type, username, date, day, month, year, uuid).await;

    match result {
        Ok(response) => println!("Received response (Get By ID): {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}
```

#### Convert

```rs
use fxdatapi::convert;

// Converting between two currencies
async fn main() {
    let user_type = "member";
    let username = "your_username";
    let date = "2023_04_02";
    let base_currency = "GBP";
    let target_currency = "EUR";
    let amount = "500";

    let result = convert::convert(user_type, username, date, base_currency, target_currency, amount).await;

    match result {
        Ok(response) => println!("Received response: {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}
```

#### Convert All

```rs
use fxdatapi::convert_all;

// Converting between two currencies
async fn main() {
    let user_type = "member";
    let username = "your_username";
    let base_currency = "GBP";
    let amount = "500";
    let date = "2023_04_02";

    let result = convert_all::convert_all(user_type, username, base_currency, amount, date).await;

    match result {
        Ok(response) => println!("Received response: {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}
```

#### Daily Average

```rs
use fxdatapi::daily_average;

// Retrieve daily average data
async fn main() {
    let user_type = "member";
    let username = "your_username";
    let date = "2023_04_10";

    let result = daily_average::daily_average(user_type, username, date).await;

    match result {
        Ok(response) => println!("Received response: {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}
```

#### Weekly Average

```rs
use fxdatapi::weekly_average;

// Retrieve weekly average data
async fn main() {
    let user_type = "member";
    let username = "your_username";
    let from_date = "2023_04_03";
    let to_date = "2023_04_07";

    let result = weekly_average::weekly_average(user_type, username, from_date, to_date).await;

    match result {
        Ok(response) => println!("Received response: {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}
```

#### Monthly Average

```rs
use fxdatapi::monthly_average;

// Retrieve monthly average data
async fn main() {
    let user_type = "member";
    let username = "your_username";
    let year = "2023";
    let month = "04";

    let result = monthly_average::monthly_average(user_type, username, year, month).await;

    match result {
        Ok(response) => println!("Received response: {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}
```

#### Margins and Spreads

```rs
use fxdatapi::margins_spreads;

// Get All Margins and Spreads
pub async fn margins_spreads_get_all() {
    let user_type = "member";
    let username = "your_username";
    let day = "19";
    let month = "04";
    let year = "2023";
    let uuid = None;

    let result = margins_spreads(user_type, username, day, month, year, uuid).await;

    match result {
        Ok(response) => println!("Received response (Get All): {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}

// Get Margins and Spreads By ID
pub async fn margins_spreads_get_by_id() {
    let user_type = "member";
    let username = "your_username";
    let day = "19";
    let month = "04";
    let year = "2023";
    let uuid = Some("margins_spreads_uuid");

    let result = margins_spreads(user_type, username, day, month, year, uuid).await;

    match result {
        Ok(response) => println!("Received response (Get By ID): {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}
```

#### Performances

```rs
use fxdatapi::performances;

// Get All Performances
pub async fn performances_get_all() {
    let user_type = "member";
    let username = "your_username";
    let uuid = None;

    let result = performances(user_type, username, uuid).await;

    match result {
        Ok(response) => println!("Received response (Get All): {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}

// Get Performances By ID
pub async fn performances_get_by_id() {
    let user_type = "member";
    let username = "your_username";
    let uuid = Some("performances_uuid");

    let result = performances(user_type, username, uuid).await;

    match result {
        Ok(response) => println!("Received response (Get By ID): {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}
```

#### Signals

```rs
use fxdatapi::signals;

// Get All Signals
pub async fn signals_get_all() {
    let user_type = "member";
    let username = "your_username";
    let uuid = None;

    let result = signals(user_type, username, uuid).await;

    match result {
        Ok(response) => println!("Received response (Get All): {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}

// Get Signals By ID
pub async fn signals_get_by_id() {
    let user_type = "member";
    let username = "your_username";
    let uuid = Some("signals_uuid");

    let result = signals(user_type, username, uuid).await;

    match result {
        Ok(response) => println!("Received response (Get By ID): {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}
```

### Setting up Currency API Account

Subscribe [here](https://moatsystems.com/currency-api/) for a user account.

### Using the Currency API

You can read the [API documentation](https://docs.fxdatapi.com/) to understand what's possible with the Currency API. If you need further assistance, don't hesitate to [contact us](https://moatsystems.com/contact/).

### License

This project is licensed under the [BSD 3-Clause License](./LICENSE).

### Copyright

(c) 2020 - 2023 [Moat Systems Limited](https://moatsystems.com).