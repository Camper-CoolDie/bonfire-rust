use std::fs;

use anyhow::Result;
use bonfire::fcm::prelude::*;
use bonfire::prelude::*;
use futures::StreamExt as _;
use serde::{Deserialize, Serialize};
use tokio::signal;
use tokio_util::sync::CancellationToken;

const EMAIL: &str = "user@example.com";
const PASSWORD: &str = "password";

// A collection of all used credentials
#[derive(Deserialize, Serialize)]
struct Credentials {
    auth: Auth,
    // FCM credentials are primarily used for creating and removing subscriptions
    fcm: FcmCredentials,
    // Subscriptions store `push_token` (the only useful field) and other data regarding
    // notifications
    subscription: Option<Subscription>,
}

async fn save_credentials(
    client: &Client,
    fcm_client: &FcmClient,
    subscription: Option<Subscription>,
) -> Result<()> {
    let credentials = Credentials {
        auth: client.auth().await?,
        fcm: fcm_client.credentials().await?,
        subscription,
    };

    let data = serde_json::to_string(&credentials)?;
    fs::write("credentials.json", data)?;
    Ok(())
}

async fn load_credentials() -> Result<(Client, FcmClient, Option<Subscription>)> {
    let data = fs::read("credentials.json")
        .ok()
        .map(|data| serde_json::from_slice::<Credentials>(&data))
        .transpose()?;

    if let Some(credentials) = data {
        // If credentials are present, reuse them to build new clients
        let client = Client::builder()
            .auth(credentials.auth)
            .expect("invalid auth")
            .build();
        let fcm_client = FcmClient::builder()
            .credentials(credentials.fcm)
            .expect("invalid credentials")
            .build();

        Ok((client, fcm_client, credentials.subscription))
    } else {
        // Send a login request and create a default FCM client (it doesn't need any input to create
        // credentials unlike normal client, so it creates them automatically)
        let client = Client::default();
        client.login(EMAIL, PASSWORD).await?;
        let fcm_client = FcmClient::default();
        save_credentials(&client, &fcm_client, None).await?;

        Ok((client, fcm_client, None))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (client, fcm_client, subscription) = load_credentials().await?;

    // Subscribe to FCM or reuse existing subscription
    let subscription = if let Some(subscription) = subscription {
        subscription
    } else {
        let profile = Profile::get(&client).await?;
        fcm_client.subscribe(profile.id).await?
    };

    // Link subscription with the authentication session by sending its token. The server can unlink
    // subscription in specific cases, so it should be linked every time
    client.set_push_token(&subscription.push_token).await?;

    // Cancellation token is used to gracefully stop the listener
    let cancellation_token = CancellationToken::new();

    // Parser additionally returns a `oneshot` receiver containing subscription + error (if the
    // listener has been stopped by an error)
    let (parser, mut subscription_receiver) = NotificationParser::new();

    // Connect to FCM and start listening for notifications (`4` is the stream buffer size, change
    // it to your needs)
    let mut stream = fcm_client
        .listen(parser, subscription, cancellation_token.clone(), 4)
        .await?;

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                // Catch CTRL+C and stop the listener
                cancellation_token.cancel();
            }
            result = &mut subscription_receiver => {
                let (subscription, error) = result.expect("parser has been dropped");

                // Save updated subscription and propagate the error, if any
                save_credentials(&client, &fcm_client, Some(subscription)).await?;
                if let Some(error) = error {
                    return Err(error.into());
                }

                break;
            }
            notification = stream.next() => {
                // Write your logic here
                println!("{notification:#?}");
            }
        }
    }

    Ok(())
}
