use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct RawFavoritesFolder {
    pub id: u64,
    pub name: String,
}
