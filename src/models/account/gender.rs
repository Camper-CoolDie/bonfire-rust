/// Represents the declared gender of an account.
#[derive(Default, Clone, Debug)]
pub enum Gender {
    /// Male gender
    #[default]
    Male,
    /// Female gender
    Female,
    /// Non-binary gender
    Other,
}
