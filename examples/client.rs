use bonfire::session::RequestKind;
use bonfire::{Result, SecureConnector, Session};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(116, 202, 162, 215)), 443);
    let host = "cf2.bonfire.moe";
    let connector = SecureConnector::new(host, addr);
    let object = json::object! { J_REQUEST_NAME: "RProjectVersionGet" };

    let mut session = Session::builder()
        .kind(RequestKind::Bonfire)
        .connect(connector)
        .await?;
    let response = session.request("/", object).await?;
    println!("{}", response);

    Ok(())
}
