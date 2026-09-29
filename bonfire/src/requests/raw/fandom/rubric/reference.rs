use crate::models::RubricRef;

pub(crate) struct RawReference {
    pub id: u64,
    pub name: String,
    pub karma_coef: f64,
}

impl From<RawReference> for RubricRef {
    fn from(value: RawReference) -> Self {
        Self {
            id: value.id,
            name: value.name,
            karma_coef: value.karma_coef / 100.0,
        }
    }
}

impl From<RawReference> for Option<RubricRef> {
    fn from(value: RawReference) -> Self {
        match value.id {
            0 => None,
            _ => Some(value.into()),
        }
    }
}
