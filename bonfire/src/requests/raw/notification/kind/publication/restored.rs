use serde::Deserialize;

use crate::models::notification::PublicationRestored;
use crate::requests::raw::RawPublicationRef;
use crate::requests::raw::publication::RawKind;

#[derive(Deserialize)]
pub(crate) struct RawRestored {
    #[serde(rename = "unitId")]
    pub id: u64,
    #[serde(rename = "unitType")]
    pub kind: RawKind,
    #[serde(rename = "parentUnitId")]
    pub parent_id: u64,
    #[serde(rename = "parentPublicationType")]
    pub parent_kind: RawKind,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl From<RawRestored> for PublicationRestored {
    fn from(value: RawRestored) -> Self {
        Self {
            publication: RawPublicationRef {
                id: value.id,
                kind: value.kind,
            }
            .into(),
            parent: RawPublicationRef {
                id: value.parent_id,
                kind: value.parent_kind,
            }
            .into(),
            reason: value.reason,
        }
    }
}
