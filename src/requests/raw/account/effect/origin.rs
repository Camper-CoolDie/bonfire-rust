use super::RawReasonKind;
use crate::models::account::EffectOrigin;

pub(super) struct IntoOriginOptions {
    pub is_system: bool,
    pub reason: String,
    pub reason_kind: RawReasonKind,
    pub from_account_name: String,
}

impl From<IntoOriginOptions> for EffectOrigin {
    fn from(value: IntoOriginOptions) -> Self {
        if value.is_system {
            EffectOrigin::System {
                reason_kind: value.reason_kind.into(),
            }
        } else {
            EffectOrigin::Account {
                name: value.from_account_name,
                reason: value.reason,
            }
        }
    }
}
