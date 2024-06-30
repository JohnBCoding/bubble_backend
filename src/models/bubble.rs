use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bubble {
    pub _id: ObjectId,
    pub user_id: String,
    pub current_feed: Option<Feed>,
    pub last_refresh: Option<String>,
    pub categories: HashMap<String, i32>,
    pub sources: HashMap<String, i32>,
    pub languages: HashMap<String, i32>,
}

impl Bubble {
    pub fn new(user_id: &str) -> Self {
        Self {
            _id: ObjectId::new(),
            user_id: user_id.to_string(),
            last_refresh: None,
            current_feed: None,
            categories: HashMap::new(),
            sources: HashMap::new(),
            languages: HashMap::new(),
        }
    }
}
