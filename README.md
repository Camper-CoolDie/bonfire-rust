# bonfire-rust

[![Crate](https://img.shields.io/crates/v/bonfire)](https://crates.io/crates/bonfire)
[![Documentation](https://img.shields.io/docsrs/bonfire)](https://docs.rs/bonfire)
[![Continuous Integration](https://github.com/Camper-CoolDie/bonfire-rust/workflows/Continuous%20Integration/badge.svg)](https://github.com/Camper-CoolDie/bonfire-rust/actions/workflows/ci.yml)

A modern, well-documented asynchronous Rust client library for the
[Bonfire API](https://github.com/timas130/bonfire).

## Features

- **Modern asynchronous design**: Built on [`tokio`](https://crates.io/crates/tokio) and
  [`hyper`](https://crates.io/crates/hyper) for non-blocking, high-performance network requests.
- **Ergonomic and fluent API**: Interact with API models like `Account` and `Fandom` directly (e.g.,
  `account.follows()`) rather than managing request structs manually.
- **Automatic session management**: Handles authentication token refreshing transparently. Log in
  once and the client manages the rest.

## Usage

### Quick Start: Logging in and printing user's email and ID

This simple example shows how to create a client, log in and fetch the profile of the currently
authenticated user:

```rust
use bonfire::prelude::*;

const EMAIL: &str = "user@example.com";
const PASSWORD: &str = "password";

#[tokio::main]
async fn main() -> ApiResult<()> {
    // Create a client with default settings and log in. Use `Client::builder` to edit these
    // settings
    let client = &Client::default();
    client.login(EMAIL, PASSWORD).await?;

    // Fetch the authenticated user's profile
    let profile = Profile::get(client).await?;
    println!("Logged in as {} (ID: {})", profile.name, profile.id);

    Ok(())
}
```

### Advanced usage: Searching accounts by name

Some methods like `Account::search` return a `Stream`, allowing you to asynchronously iterate over
accounts while the API loads them. This example also shows how to properly save and load
authentication tokens to reuse the same session:

```rust
use std::fs;
use std::fs::File;
use std::io::Write;

use anyhow::Result;
use bonfire::prelude::*;
use futures_util::TryStreamExt as _;

const EMAIL: &str = "user@example.com";
const PASSWORD: &str = "password";

async fn save_credentials(client: &Client) -> Result<()> {
    let data = serde_json::to_string(&client.auth().await?)?;
    let mut file = File::create("credentials.json")?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Build client & authenticate (either by using `credentials.json` or sending a login request)
    let auth_data = fs::read("credentials.json")
        .ok()
        .map(|data| serde_json::from_slice::<Auth>(&data))
        .transpose()?;
    let client = &match auth_data {
        Some(auth) => Client::builder().auth(auth).expect("invalid auth").build(),
        None => {
            let client = Client::default();
            client.login(EMAIL, PASSWORD).await?;
            save_credentials(&client).await?;
            client
        }
    };

    // Fetch accounts that have "Sus" in their name
    Account::search(client, Some("Sus"), 0)
        .try_for_each(|account| async move {
            println!("User {} (ID: {})", account.name, account.id);
            Ok(())
        })
        .await?;

    // Save authentication credentials and exit program
    save_credentials(client).await?;
    Ok(())
}
```

## License

bonfire-rust is provided under the MIT License. See [LICENSE](LICENSE).
