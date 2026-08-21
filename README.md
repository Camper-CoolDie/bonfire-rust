# bonfire-rust

[![Crate](https://img.shields.io/crates/v/bonfire)](https://crates.io/crates/bonfire)
[![Documentation](https://img.shields.io/docsrs/bonfire)](https://docs.rs/bonfire)
[![CI Status](https://github.com/Camper-CoolDie/bonfire-rust/workflows/CI/badge.svg)](https://github.com/Camper-CoolDie/bonfire-rust/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/bonfire.svg)](#license)

bonfire-rust is a client library for the [Bonfire API](https://github.com/timas130/bonfire). It
exposes asynchronous methods for sending API requests and receiving FCM notifications (with `fcm`
feature). The initial goal was to create a desktop Bonfire client on top of an API library, but
because of a full backend rework coming soon bonfire-rust has shifted its goals towards bot
development.

Other features include `serde`, which implements Serialize and Deserialize on almost every API
object. You will probably want to enable this for caching authentication tokens at least, see
[example](#searching-accounts-by-name). `http1` and `http2` enable corresponding HTTP versions, at
least one must be present (the latter is enabled by default).

## Examples

You can use the following code as a base for your future program.

### Logging in and printing user's email and ID

The shortest example, which shows how to create a client, log in and fetch the profile of the
currently authenticated user:

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

### Searching accounts by name

Some methods like `Account::search` return a `Stream`, allowing you to asynchronously iterate over
accounts while the API loads them (the library manages pagination internally). This example also
shows how to properly save and load authentication tokens to reuse the same session:

```rust
use std::fs;

use anyhow::Result;
use bonfire::prelude::*;
use futures_util::TryStreamExt as _;

const EMAIL: &str = "user@example.com";
const PASSWORD: &str = "password";

async fn save_credentials(client: &Client) -> Result<()> {
    let data = serde_json::to_string(&client.auth().await?)?;
    fs::write("credentials.json", data)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Build client & authenticate (either from `credentials.json` or by sending a login request)
    let auth_data = fs::read("credentials.json")
        .ok()
        .map(|data| serde_json::from_slice::<Auth>(&data))
        .transpose()?;
    let client = &if let Some(auth) = auth_data {
        Client::builder().auth(auth).expect("invalid auth").build()
    } else {
        let client = Client::default();
        client.login(EMAIL, PASSWORD).await?;
        save_credentials(&client).await?;
        client
    };

    // Fetch accounts that have "Sus" in their name
    Account::search(client, Some("Sus"), 0)
        .try_for_each(|account| async move {
            println!("User {} (ID: {})", account.name, account.id);
            Ok(())
        })
        .await?;

    // Save tokens and exit program
    save_credentials(client).await?;
    Ok(())
}
```

### Debugging

The library supports [`tracing`](https://crates.io/crates/tracing). One can easily enable logging by
calling `tracing_subscriber::fmt().init()`. Log output includes errors and outgoing requests by
default and even more info if the maximum log level is set to `DEBUG`.

## License

This project is licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE.txt](LICENSE-APACHE.txt))
- [MIT License](https://opensource.org/licenses/MIT) ([LICENSE-MIT.txt](LICENSE-MIT.txt))

at your option.

## Contribution

To get involved, take a look at [CONTRIBUTING.md](CONTRIBUTING.md).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
`bonfire-rust` by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.
