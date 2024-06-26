use bonfire::session::RequestKind;
use bonfire::{Result, SecureConnector, Session};

#[tokio::main]
async fn main() -> Result<()> {
    let host = "cf2.bonfire.moe";
    let connector = SecureConnector::new(host, (host, 443));
    let object = json::object! { J_REQUEST_NAME: "RProjectVersionGet" };

    let mut session = Session::builder()
        .kind(RequestKind::Bonfire)
        .connect(connector)
        .await?;
    let response = session.request("/", object).await?;
    println!("{}", response);

    Ok(())
}
