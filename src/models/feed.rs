use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Feed {
    pub pagination: Pagination,
    pub data: Vec<Article>,
    pub refresh_cooldown: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    pub limit: i32,
    pub offset: i32,
    pub count: i32,
    pub total: i32,
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Article {
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub author: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub title: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub description: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub url: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub source: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub image: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub category: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub language: String,
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub published_at: String,
}
