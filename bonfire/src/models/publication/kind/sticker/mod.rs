mod error;
mod pack;

pub use error::*;
pub use pack::Pack;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::client::Request as _;
use crate::models::publication::{Kind, Publishable, Removable};
use crate::models::{Account, ImageRef, Publication};
use crate::requests::publication::sticker::CreateStickerRequest;
use crate::sealed::Sealed;
use crate::{Client, Result};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Sticker {
    pub image: ImageRef,
    pub author: Account,
    pub pack_id: u64,
}
impl Sticker {
    pub const MAX_SIZE: usize = 32 * 1024;
    pub const GIF_MAX_SIZE: usize = 256 * 1024;
    pub const MAX_DIMENSION: usize = 300;
}

impl Publishable for Sticker {
    /// Returns the publication kind as [`Kind::Sticker`].
    fn kind(&self) -> Kind {
        Kind::Sticker
    }
}

impl Removable for Sticker {}
impl Sealed for Sticker {}

impl Publication<Sticker> {
    pub async fn create(
        client: &Client,
        pack_id: u64,
        image: &[u8],
    ) -> Result<Publication<Sticker>> {
        CreateStickerRequest::new(pack_id, image).send(client).await
    }

    pub async fn create_gif(
        client: &Client,
        pack_id: u64,
        first_frame: &[u8],
        animated: &[u8],
    ) -> Result<Publication<Sticker>> {
        CreateStickerRequest::new_gif(pack_id, first_frame, animated)
            .send(client)
            .await
    }
}
