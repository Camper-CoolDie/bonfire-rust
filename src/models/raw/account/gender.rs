use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::Gender;

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub(crate) enum RawGender {
    Male = 0,
    Female = 1,
    Other = 2,
}

impl From<RawGender> for Gender {
    fn from(value: RawGender) -> Self {
        match value {
            RawGender::Male => Gender::Male,
            RawGender::Female => Gender::Female,
            RawGender::Other => Gender::Other,
        }
    }
}

impl From<Gender> for RawGender {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Male => RawGender::Male,
            Gender::Female => RawGender::Female,
            Gender::Other => RawGender::Other,
        }
    }
}
