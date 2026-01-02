use serde_json::json;

use crate::models::raw::RawGender;
use crate::models::{Account, EmptyResponse, Gender};
use crate::{Client, Result};

impl Account {
    pub(crate) async fn _set_gender(client: &mut Client, gender: Gender) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>(
                "RAccountsBioSetSex",
                json!({ "sex": RawGender::from(gender) }),
                Vec::default(),
            )
            .await?;
        Ok(())
    }
}
