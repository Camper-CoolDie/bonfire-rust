use serde::Deserialize;

use crate::models::PublicationRef;
use crate::requests::raw::publication::RawKind;

#[derive(Deserialize)]
pub(crate) struct RawReference {
    #[serde(rename = "J_UNIT_ID")]
    pub id: u64,
    #[serde(rename = "J_UNIT_TYPE")]
    pub kind: RawKind,
}

impl From<RawReference> for PublicationRef {
    fn from(value: RawReference) -> Self {
        Self {
            id: value.id,
            kind: value.kind.into(),
        }
    }
}

impl From<RawReference> for Option<PublicationRef> {
    fn from(value: RawReference) -> Self {
        match value.id {
            0 => None,
            _ => Some(value.into()),
        }
    }
}
