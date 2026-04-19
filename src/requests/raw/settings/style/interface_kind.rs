use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};

use crate::models::settings::InterfaceKind;
use crate::{Error, Result};

#[derive(Debug)]
pub(crate) enum RawInterfaceKind {
    NavigationPanel,
    Menu,
    Unknown(i32),
}

impl Serialize for RawInterfaceKind {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let kind = match self {
            RawInterfaceKind::NavigationPanel => 0,
            RawInterfaceKind::Menu => 1,
            RawInterfaceKind::Unknown(unknown) => *unknown,
        };

        serializer.serialize_i32(kind)
    }
}

impl<'de> Deserialize<'de> for RawInterfaceKind {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i32::deserialize(deserializer)? {
            0 => RawInterfaceKind::NavigationPanel,
            1 => RawInterfaceKind::Menu,
            other => RawInterfaceKind::Unknown(other),
        })
    }
}

impl TryFrom<RawInterfaceKind> for InterfaceKind {
    type Error = Error;

    fn try_from(value: RawInterfaceKind) -> Result<Self> {
        Ok(match value {
            RawInterfaceKind::NavigationPanel => InterfaceKind::NavigationPanel,
            RawInterfaceKind::Menu => InterfaceKind::Menu,
            RawInterfaceKind::Unknown(_) => return Err(Error::UnknownVariant(Box::new(value))),
        })
    }
}

impl From<InterfaceKind> for RawInterfaceKind {
    fn from(value: InterfaceKind) -> Self {
        match value {
            InterfaceKind::NavigationPanel => RawInterfaceKind::NavigationPanel,
            InterfaceKind::Menu => RawInterfaceKind::Menu,
        }
    }
}
