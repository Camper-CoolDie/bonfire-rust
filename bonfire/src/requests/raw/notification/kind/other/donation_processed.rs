use serde::Deserialize;

use crate::models::notification::DonationProcessed;

#[derive(Deserialize)]
pub(crate) struct RawDonationProcessed {
    #[serde(rename = "sum")]
    pub amount: u64,
}

impl From<RawDonationProcessed> for DonationProcessed {
    fn from(value: RawDonationProcessed) -> Self {
        Self {
            amount: value.amount,
        }
    }
}
