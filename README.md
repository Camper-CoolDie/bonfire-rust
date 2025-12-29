# bonfire-rust

[![Crate](https://img.shields.io/crates/v/bonfire)](https://crates.io/crates/bonfire)
[![Documentation](https://img.shields.io/docsrs/bonfire)](https://docs.rs/bonfire)
[![Continuous Integration](https://github.com/Camper-CoolDie/bonfire-rust/workflows/Continuous%20Integration/badge.svg)](https://github.com/Camper-CoolDie/bonfire-rust/actions/workflows/ci.yml)

bonfire is an asynchronous Rust client library for the [Bonfire](https://github.com/timas130/bonfire) API.

- **Efficiency**. Fast asynchronous requests using [hyper](https://crates.io/crates/hyper)
- **Simplicity**. Well-documented request-sending methods like `Account::search()`
- **Debuggability**. Connection attempts, requests and errors are logged using [tracing](https://crates.io/crates/tracing)

## Example

```rust,no_run
use std::fs;
use std::fs::File;
use std::io::Write;

use bonfire::models::Auth;
use bonfire::{Client, Result};

const EMAIL: &str = "user@example.com";
const PASSWORD: &str = "password";

#[tokio::main]
async fn main() -> Result<()> {
    // Set up tracing
    tracing_subscriber::fmt::init();

    // Authenticate
    let mut client = Client::connect().await?;
    if let Ok(ref data) = fs::read("credential.json") {
        client.auth = Some(serde_json::from_slice(&data)?);
    } else {
        Auth::login(&mut client, EMAIL, PASSWORD).await?;
    }

    // Get information about the currently authenicated user
    println!("{:#?}", Auth::me(&mut client).await?);

    // Save auth tokens to "credential.json"
    if let Some(ref auth) = client.auth.as_ref() {
        let data = serde_json::to_string(auth)?;
        let mut file = File::create("credential.json").unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    Ok(())
}
```

Required dependencies in `Cargo.toml` for this example:

```toml
[dependencies]
bonfire = "1.0"
serde_json = "1.0"
tokio = { version = "1.48", features = ["macros", "rt-multi-thread"] }
tracing-subscriber = "0.3"
```
