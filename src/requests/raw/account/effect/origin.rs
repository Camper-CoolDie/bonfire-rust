use super::RawReasonKind;
use crate::models::account::EffectOrigin;
use crate::{Error, Result};

pub(super) struct IntoOriginOptions {
    pub is_system: bool,
    pub reason: String,
    pub reason_kind: RawReasonKind,
    pub from_account_name: String,
}

impl TryFrom<IntoOriginOptions> for EffectOrigin {
    type Error = Error;

    fn try_from(value: IntoOriginOptions) -> Result<Self> {
        Ok(if value.is_system {
            EffectOrigin::System {
                reason_kind: value.reason_kind.try_into()?,
            }
        } else {
            EffectOrigin::Account {
                name: value.from_account_name,
                reason: value.reason,
            }
        })
    }
}
