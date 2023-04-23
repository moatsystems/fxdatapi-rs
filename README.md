## Currensees

[![Crates.io][crates-badge]][crates-url]
[![Build Status][ci-badge]][ci-url]

[crates-badge]: https://img.shields.io/crates/v/currensees
[crates-url]: https://crates.io/crates/currensees
[ci-badge]: https://img.shields.io/github/actions/workflow/status/moatsystems/currensees-rs/ci.yml?branch=main
[ci-url]: https://github.com/moatsystems/currensees-rs/actions

A [Rust](https://www.rust-lang.org/) library to interact with the [Currency API](https://moatsystems.com/currency-api/).

### Usage

Add this library as a dependency in your `Cargo.toml`:

```toml
[dependencies]
currensees = "0.1.1"
```

#### Authentication

```rs
use currensees::auth;

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

#### Convert
##### Converting between two currencies

```rs
use currensees::convert;

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
##### Converting between two currencies

```rs
use currensees::convert_all;

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
use currensees::daily_average;

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
use currensees::weekly_average;

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

#### Margins and Spreads

```rs
use currensees::margins_spreads;

async fn example_margins_spreads() {
    let user_type = "member";
    let username = "your_username";
    let day = "19";
    let month = "04";
    let year = "2023";
    let uuid = Some("margins_spreads_uuid");

    let result = margins_spreads::margins_spreads(user_type, username, day, month, year, uuid).await;

    match result {
        Ok(response) => println!("Received response: {:?}", response),
        Err(error) => println!("Error: {:?}", error),
    }
}
```

### Setting up Currency API Account

Subscribe [here](https://moatsystems.com/currency-api/) for a user account.

### Using the Currency API

You can read the [API documentation](https://docs.currensees.com/) to understand what's possible with the Currency API. If you need further assistance, don't hesitate to [contact us](https://moatsystems.com/contact/).

### License

This project is licensed under the [BSD 3-Clause License](./LICENSE).

### Copyright

(c) 2020 - 2023 [Moat Systems Limited](https://moatsystems.com).