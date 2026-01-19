# bonfire-rust

[![Crate](https://img.shields.io/crates/v/bonfire)](https://crates.io/crates/bonfire)
[![Documentation](https://img.shields.io/docsrs/bonfire)](https://docs.rs/bonfire)
[![Continuous Integration](https://github.com/Camper-CoolDie/bonfire-rust/workflows/Continuous%20Integration/badge.svg)](https://github.com/Camper-CoolDie/bonfire-rust/actions/workflows/ci.yml)

bonfire is an asynchronous Rust client library for the [Bonfire](https://github.com/timas130/bonfire) API.

- **Efficiency**. Fast asynchronous requests thanks to [hyper](https://crates.io/crates/hyper)
- **Simplicity**. Well-documented request-sending methods like `Account::search()`
- **Debuggability**. Requesting and errors are logged using [tracing](https://crates.io/crates/tracing)

## Example

```rust
use std::fs;
use std::fs::File;
use std::io::Write;

use anyhow::Result;
use bonfire::models::Auth;
use bonfire::Client;

const EMAIL: &str = "user@example.com";
const PASSWORD: &str = "password";

async fn save_credentials(client: &Client) -> Result<()> {
    if let Some(ref auth) = client.auth().await? {
        let data = serde_json::to_string(&auth)?;
        let mut file = File::create("credentials.json")?;
        file.write_all(data.as_bytes())?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up tracing
    tracing_subscriber::fmt::init();

    // Build client & authenticate (either by using `credentials.json` or sending a log-in request)
    let auth = fs::read("credentials.json")
        .ok()
        .map(|data| serde_json::from_slice::<Auth>(&data))
        .transpose()?;
    let client = match auth {
        Some(auth) => Client::builder().auth(auth).expect("invalid auth").build(),
        None => {
            let client = Client::default();
            client.login(EMAIL, PASSWORD).await?;
            save_credentials(&client).await?;
            client
        }
    };

    // Get information about the currently authenicated user
    println!("{:#?}", Auth::me(&client).await?);

    // Save authentication credentials and exit program
    save_credentials(&client).await?;
    Ok(())
}
```

The following dependencies are required for this example to work:

```toml
[dependencies]
anyhow = "1.0"
bonfire = "1.0"
serde_json = "1.0"
tokio = { version = "1.49", features = ["macros", "rt-multi-thread"] }
tracing-subscriber = "0.3"
```
