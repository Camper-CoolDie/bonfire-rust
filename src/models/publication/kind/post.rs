use crate::models::publication::{PublicationInheritor, PublicationKind};

/// Represents a post.
#[derive(Default, Clone, Debug)]
pub struct Post {
    /// An identifier of the corresponding publication. Should always be set to a valid value if
    /// constructing with `{ ... }` and match [Publication::id][crate::models::Publication::id]
    pub id: u64,
    // /// The post's content
    // pub pages: Vec<Page>,
    // /// A comment which earned the highest amount of karma
    // pub best_comment: Option<Publication<Comment>>,
    /// The post's rubric identifier
    pub rubric_id: Option<u64>,
    /// The post's rubric name
    pub rubric_name: Option<String>,
    /// The post's rubric karma coefficient
    pub rubric_karma_coef: Option<f32>,
    // /// The post's relay race
    // pub relay_race: Option<RelayRace>,
}

impl PublicationInheritor for Post {
    fn kind(&self) -> PublicationKind {
        PublicationKind::Post
    }
}
