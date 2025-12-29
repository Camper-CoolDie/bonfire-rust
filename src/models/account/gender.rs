use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents an account gender.
#[derive(Default, Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub enum Gender {
    /// Male gender
    #[default]
    Male = 0,
    /// Female gender
    Female = 1,
    /// Non-binary gender
    Other = 2,
}
