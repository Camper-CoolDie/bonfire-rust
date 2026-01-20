use crate::models::publication::{PublicationInheritor, PublicationKind};

/// Represents a post.
#[derive(Default, Clone, Debug)]
pub struct Post {
    /// An identifier of the corresponding publication. Should always be set to a valid value if
    /// constructing with `{ ... }` and should always match
    /// [`Publication::id`][crate::models::Publication::id]
    pub id: u64,
    // /// The content of this post
    // pub pages: Vec<Page>,
    // /// A comment which earned the highest amount of karma
    // pub best_comment: Option<Publication<Comment>>,
    /// An identifier of a rubric which this post is linked to
    pub rubric_id: Option<u64>,
    /// A name of a rubric which this post is linked to
    pub rubric_name: Option<String>,
    /// A karma coefficient of a rubric which this post is linked to
    pub rubric_karma_coef: Option<f64>,
    // /// A relay race which this post is linked to
    // pub relay_race: Option<RelayRace>,
}

impl PublicationInheritor for Post {
    fn kind(&self) -> PublicationKind {
        PublicationKind::Post
    }
}
