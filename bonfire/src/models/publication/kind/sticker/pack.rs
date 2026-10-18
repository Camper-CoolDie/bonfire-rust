#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::client::Request as _;
use crate::models::publication::{Kind, Publishable, Removable};
use crate::models::{Account, Comment, ImageRef, Publication, Sticker};
use crate::requests::publication::sticker::pack::{
    CreatePackRequest, GetPackRequest, SetNameRequest,
};
use crate::sealed::Sealed;
use crate::{Client, Result};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Pack {
    pub name: String,
    pub author: Account,
    pub icon: ImageRef,
    pub stickers: Vec<Publication<Sticker>>,
    /// A comment which earned the highest amount of karma
    pub best_comment: Option<Publication<Comment>>,
    /// The total karma received by this publication (can be positive or negative)
    pub karma: f64,
    /// The karma that the authenticated user personally placed on this publication, if any
    pub my_karma: Option<f64>,
    /// The total number of comments associated with this publication
    pub comments_count: u64,
}
impl Pack {
    pub const NAME_MAX_LENGTH: usize = 100;
    pub const ICON_MAX_SIZE: usize = 32 * 1024;
    pub const ICON_DIMENSION: usize = 384;
}

impl Publishable for Pack {
    /// Returns the publication kind as [`Kind::StickerPack`].
    fn kind(&self) -> Kind {
        Kind::StickerPack
    }
}

impl Removable for Pack {}
impl Sealed for Pack {}

impl Publication<Pack> {
    pub async fn get_by_id(client: &Client, id: u64) -> Result<Self> {
        GetPackRequest::new(id).send(client).await
    }

    pub async fn get_by_sticker_id(client: &Client, id: u64) -> Result<Self> {
        GetPackRequest::new_by_sticker_id(id).send(client).await
    }

    pub async fn create(client: &Client, name: &str, icon: &[u8]) -> Result<Self> {
        CreatePackRequest::new(name, icon).send(client).await
    }

    pub async fn set_name(&self, client: &Client, name: &str) -> Result<&Self> {
        SetNameRequest::new(self.id, name).send(client).await?;
        Ok(self)
    }

    pub async fn set_name_and_icon(
        &self,
        client: &Client,
        name: &str,
        icon: &[u8],
    ) -> Result<ImageRef> {
        SetNameRequest::new_with_icon(self.id, name, icon)
            .send(client)
            .await
    }
}
