mod any;
mod direct;
mod fandom_root;
mod fandom_sub;
mod group;

pub(crate) use any::AnyRawChat;
pub(crate) use direct::RawDirect;
pub(crate) use fandom_root::RawFandomRoot;
pub(crate) use fandom_sub::{RawFandomSub, RawParams as RawFandomSubParams};
pub(crate) use group::{RawGroup, RawMemberRole, RawMemberStatus, RawParams as RawGroupParams};

pub(crate) enum RawKind {
    FandomRoot,
    FandomSub,
    Group,
    Direct,
    Unknown(i64),
}

impl From<RawKind> for i64 {
    fn from(value: RawKind) -> Self {
        match value {
            RawKind::FandomRoot => 1,
            RawKind::Direct => 2,
            RawKind::Group => 3,
            RawKind::FandomSub => 4,
            RawKind::Unknown(unknown) => unknown,
        }
    }
}

impl From<i64> for RawKind {
    fn from(value: i64) -> Self {
        match value {
            1 => RawKind::FandomRoot,
            2 => RawKind::Direct,
            3 => RawKind::Group,
            4 => RawKind::FandomSub,
            other => RawKind::Unknown(other),
        }
    }
}
