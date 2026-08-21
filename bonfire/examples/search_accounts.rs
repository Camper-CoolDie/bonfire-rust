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
