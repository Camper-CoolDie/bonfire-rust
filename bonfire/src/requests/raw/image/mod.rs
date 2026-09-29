mod gif;
mod normal;

pub(crate) use gif::Gif;
pub(crate) use normal::Normal;
use serde::de::DeserializeOwned;

use crate::Result;

pub(crate) trait Image: Send + Sync {
    type Target: Send;
    type Response: DeserializeOwned;

    fn attachments(&self) -> Vec<&[u8]>;
    fn parse_response(&self, response: Self::Response) -> Result<Self::Target>;
}
