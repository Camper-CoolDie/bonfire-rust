#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Kind {
    #[default]
    Text,
    Image,
    Images,
    Video,
    Quote,
    Link,
    Spoiler,
    Poll,
    LinkWithThumbnail,
    Table,
    YoutubeVideo,
    CodeBlock,
    BonfireLink,
    // Placed automatically at the end of each post in a post relay
    PostRelay,
    #[cfg_attr(feature = "serde", serde(untagged))]
    Unknown(i64),
}
