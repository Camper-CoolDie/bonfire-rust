#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::Profile;
use crate::requests::account::profile::SetGenderRequest;
use crate::{Client, Result};

/// Represents the declared gender of an account.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Gender {
    /// Male gender
    #[default]
    Male,
    /// Female gender
    Female,
    /// Non-binary gender
    Other,
}

impl Profile {
    /// Sets the account's declared gender.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn set_gender(client: &Client, gender: Gender) -> Result<()> {
        SetGenderRequest::new(gender).send_request(client).await?;
        Ok(())
    }
}
