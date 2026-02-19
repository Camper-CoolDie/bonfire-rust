use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::{Fandom, Language};
use crate::requests::raw::{RawFandom, RawLanguage};
use crate::{Client, Error, Result, RootError};

#[derive(Deserialize)]
pub(crate) struct Response {
    fandom: RawFandom,
}

impl TryFrom<Response> for Fandom {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        value.fandom.try_into()
    }
}

#[derive(Serialize)]
pub(crate) struct GetFandomRequest {
    #[serde(rename = "fandomId")]
    id: u64,
    #[serde(rename = "languageId")]
    language: i64,
    #[serde(rename = "accountLanguageId")]
    my_language: RawLanguage,
}
impl GetFandomRequest {
    pub(crate) fn new(id: u64, language: Option<Language>, my_language: Language) -> Self {
        Self {
            id,
            language: language.map_or(-1, |language| RawLanguage::from(language) as i64),
            my_language: RawLanguage::from(my_language),
        }
    }
}

impl Request for GetFandomRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client.send_request("RFandomsGet", self, Vec::new()).await
    }
}
