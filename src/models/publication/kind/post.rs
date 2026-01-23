use crate::models::publication::{PublicationInheritor, PublicationKind};

/// Represents the specific data for a post publication.
#[derive(Default, Clone, Debug)]
pub struct Post {
    /// The unique identifier of the corresponding publication
    pub id: u64,
    // /// The content of this post
    // pub pages: Vec<Page>,
    // /// A comment which earned the highest amount of karma
    // pub best_comment: Option<Publication<Comment>>,
    /// The identifier of the rubric this post is linked to
    pub rubric_id: Option<u64>,
    /// The name of the rubric this post is linked to
    pub rubric_name: Option<String>,
    /// The karma coefficient of the rubric this post is linked to
    pub rubric_karma_coef: Option<f64>,
    // /// A relay race this post is linked to
    // pub relay_race: Option<RelayRace>,
}

impl PublicationInheritor for Post {
    /// Returns the publication kind as [`PublicationKind::Post`].
    fn kind(&self) -> PublicationKind {
        PublicationKind::Post
    }
}
