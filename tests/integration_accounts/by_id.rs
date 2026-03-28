use bonfire::models::Account;
use bonfire::{Error, RootError, UnavailableError};
use cmp::compare_structs;

use crate::common;

const ID: u64 = 207506;

#[cfg(feature = "serde")]
#[tokio::test]
async fn test_success() {
    let expected = common::load_ron_fixture::<Account>("account/expected.ron");
    let (mock, client) = common::setup_single("account/get_account.json");
    let result = Account::by_id(&client, ID).await;

    let account = result.unwrap();
    compare_structs!(account, expected);
    mock.assert();
}

#[tokio::test]
async fn test_not_found() {
    let (mock, client) = common::setup_single("error/not_found.json");
    let result = Account::by_id(&client, 999999999).await;

    assert!(matches!(
        result.unwrap_err(),
        Error::RootError(RootError::Unavailable(UnavailableError::NotFound))
    ));
    mock.assert();
}
