use bonfire::session::RequestKind;
use bonfire::{Result, SecureConnector, Session};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let host = "cf2.bonfire.moe";
    let connector = SecureConnector::new(host, (host, 443));
    let json = json!({ "J_REQUEST_NAME": "RProjectVersionGet" });

    let mut session = Session::builder()
        .kind(RequestKind::Bonfire)
        .connect(connector)
        .await?;
    let response = session.request("/", json).await?;
    println!("{}", response);

    Ok(())
}
