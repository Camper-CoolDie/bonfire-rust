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
