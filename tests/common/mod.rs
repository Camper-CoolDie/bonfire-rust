use std::fs;
use std::sync::LazyLock;

use bonfire::Client;
use httpmock::{Mock, MockServer};
use nanoid::nanoid;
#[cfg(feature = "serde")]
use serde::Deserialize;

pub static MOCK_SERVER: LazyLock<MockServer> = LazyLock::new(|| {
    let server = MockServer::start();
    println!("mock server started at {}", server.address());
    server
});

fn setup() -> (Client, String) {
    // Generating random URI endpoints allows running tests in parallel
    let endpoint = "/".to_owned() + &nanoid!();
    let client = Client::builder()
        .root_uri(MOCK_SERVER.url(&endpoint))
        .melior_uri(MOCK_SERVER.url(&endpoint))
        .build();

    (client, endpoint)
}

pub fn setup_single(fixture_path: &'static str) -> (Mock<'static>, Client) {
    let (client, endpoint) = setup();
    let mock = MOCK_SERVER.mock(|when, then| {
        when.path(endpoint);
        then.body(load_fixture(fixture_path));
    });

    (mock, client)
}

// We could just use Client::default(), but such client would be able to connect to the real server
pub fn setup_none() -> (Mock<'static>, Client) {
    let (client, endpoint) = setup();
    let mock = MOCK_SERVER.mock(|when, then| {
        when.path(endpoint);
        then.status(503);
    });

    (mock, client)
}

pub fn load_fixture(path: &'static str) -> Vec<u8> {
    let path = "tests/fixtures/".to_owned() + path;
    fs::read(&path).unwrap_or_else(|error| panic!("failed to load fixture at {path}: {error}"))
}

#[cfg(feature = "serde")]
pub fn load_ron_fixture<T>(path: &'static str) -> T
where
    for<'de> T: Deserialize<'de>,
{
    let path = "tests/fixtures/".to_owned() + path;
    let content = fs::read(&path)
        .unwrap_or_else(|error| panic!("failed to load RON fixture at {path}: {error}"));
    ron::de::from_bytes(&content)
        .unwrap_or_else(|error| panic!("failed to deserialize RON fixture at {path}: {error}"))
}
