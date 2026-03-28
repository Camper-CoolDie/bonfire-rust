use std::mem;

use bonfire::models::Auth;
use bonfire::models::auth::LoginError;
use bonfire::{Client, Error, Result};
use cmp::compare_structs;

use crate::common;

const EMAIL: &str = "test@example.com";
const PASSWORD: &str = "password";

async fn assert_error(result: Result<&Client>, login_error: LoginError) {
    match result {
        Err(Error::RequestError(error_box)) => {
            let error = error_box.downcast::<LoginError>().unwrap();
            assert!(
                mem::discriminant(&*error) == mem::discriminant(&login_error),
                "expected {login_error:?}, got {error:?}"
            );
        }
        Err(error) => panic!("expected request error, got {error:?}"),
        Ok(client) => assert!(!client.is_auth().await),
    };
}

#[cfg(feature = "serde")]
#[tokio::test]
async fn test_success() {
    let expected = common::load_ron_fixture::<Auth>("auth/expected.ron");
    let (mock, client) = common::setup_single("auth/login_email/success.json");
    let result = client.login(EMAIL, PASSWORD).await;

    result.unwrap();
    let auth = client.auth().await.unwrap();
    compare_structs!(auth, expected);
    assert!(client.is_auth().await);
    mock.assert();
}

#[tokio::test]
async fn test_invalid_email() {
    let (mock, client) = common::setup_single("auth/login_email/invalid_email.json");
    let result = client.login("invalid email", PASSWORD).await;

    assert_error(result, LoginError::InvalidEmail).await;
    mock.assert();
}

#[tokio::test]
async fn test_wrong_email() {
    let (mock, client) = common::setup_single("auth/login_email/wrong_email.json");
    let result = client.login("inexistent@example.com", PASSWORD).await;

    assert_error(result, LoginError::WrongEmail).await;
    mock.assert();
}

#[tokio::test]
async fn test_wrong_password() {
    let (mock, client) = common::setup_single("auth/login_email/wrong_password.json");
    let result = client.login(EMAIL, "wrong password").await;

    assert_error(result, LoginError::WrongPassword).await;
    mock.assert();
}

#[tokio::test]
async fn test_hard_banned() {
    let (mock, client) = common::setup_single("auth/login_email/hard_banned.json");
    let result = client.login(EMAIL, PASSWORD).await;

    assert_error(result, LoginError::HardBanned).await;
    mock.assert();
}

#[cfg(feature = "serde")]
#[tokio::test]
async fn test_already_authenticated() {
    let expected = common::load_ron_fixture::<Auth>("auth/expected.ron");
    let (mock, client) = common::setup_none();
    client.set_auth(Some(expected)).await.unwrap();
    let result = client.login(EMAIL, PASSWORD).await;

    assert!(matches!(result.unwrap_err(), Error::AlreadyAuthenticated));
    assert!(client.is_auth().await); // Should still be authenticated
    mock.assert_calls(0);
}
