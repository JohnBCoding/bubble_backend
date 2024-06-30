use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Saved {
    pub _id: Option<ObjectId>,
    pub user_id: String,
    pub article: Article,
}

impl Saved {
    pub fn new(user_id: &str, article: &Article) -> Self {
        Self {
            _id: Some(ObjectId::new()),
            user_id: user_id.to_string(),
            article: article.clone(),
        }
    }
}
