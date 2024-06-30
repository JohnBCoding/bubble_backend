use crate::prelude::*;

const DISLIKE_MULTI: i32 = -2;
const LIKE_MULTI: i32 = 3;
const SAVE_MULTI: i32 = 5;
const RATING_MAX: i32 = 25;
const RATING_MIN: i32 = -25;

#[derive(Clone)]
pub struct Mongo {
    pub client: Client,
    pub bubble_collection: Collection<Bubble>,
    pub saved_collection: Collection<Saved>,
}

impl Mongo {
    pub async fn new() -> Self {
        let db_uri =
            env::var("DATABASE_URI").expect("Unable to get [DATABASE_URI] from environment.");

        let client_options = ClientOptions::parse(db_uri)
            .await
            .expect("Unable to parse connection string");

        let client =
            Client::with_options(client_options).expect("Unable to connect to Mongo Cluster");

        let bubble_collection = client.database("users").collection::<Bubble>("bubble");
        let saved_collection = client.database("users").collection::<Saved>("saved");

        Self {
            client,
            bubble_collection,
            saved_collection,
        }
    }

    /// Ping the Mongo DB to make sure a valid connection is present.
    pub async fn ping(&self) -> mongodb::error::Result<()> {
        self.client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;

        println!("Successfully pinged DB.");

        Ok(())
    }

    pub async fn rate(
        &self,
        user_id: &str,
        article: &Article,
        action: &str,
    ) -> mongodb::error::Result<Feed> {
        let result = self
            .bubble_collection
            .find_one(doc! {"user_id": user_id}, None)
            .await?;

        let (mut bubble, new) = if let Some(bubble) = result {
            (bubble, false)
        } else {
            (Bubble::new(&user_id), true)
        };

        update_rating(&mut bubble, article, action);

        // Delete from feed
        let mut feed = bubble.current_feed.unwrap().clone();
        for (index, compare_article) in feed.data.iter().enumerate() {
            if compare_article.author == article.author
                && compare_article.title == article.title
                && compare_article.description == article.description
            {
                feed.data.remove(index);
                break;
            }
        }

        bubble.current_feed = Some(feed);

        if !new {
            let _ = self
                .bubble_collection
                .replace_one(doc! {"user_id": user_id}, bubble.clone(), None)
                .await?;
        } else {
            let _ = self
                .bubble_collection
                .insert_one(bubble.clone(), None)
                .await?;
        }

        return Ok(bubble.current_feed.unwrap());
    }

    pub async fn save_feed(&self, user_id: &str, feed: &Feed) -> mongodb::error::Result<()> {
        let result = self
            .bubble_collection
            .find_one_and_update(
                doc! {"user_id": user_id},
                doc! {"$set": doc!{"current_feed": to_bson(feed).unwrap()}},
                None,
            )
            .await?;

        if result.is_none() {
            let mut bubble = Bubble::new(&user_id);
            bubble.current_feed = Some(feed.clone());
            let _ = self.bubble_collection.insert_one(bubble, None).await?;
        }
        Ok(())
    }

    // Returns current feed from bubble from user_id
    pub async fn get_current_feed(&self, user_id: &str) -> mongodb::error::Result<Feed> {
        let result = self
            .bubble_collection
            .find_one(doc! {"user_id": user_id}, None)
            .await?;

        if let Some(bubble) = result {
            return Ok(bubble.current_feed.unwrap());
        }

        Err(mongodb::error::Error::custom("Unable to locate bubble"))
    }

    // Returns current bubble from user_id
    pub async fn get_bubble(&self, user_id: &str) -> mongodb::error::Result<Bubble> {
        let result = self
            .bubble_collection
            .find_one(doc! {"user_id": user_id}, None)
            .await?;

        if let Some(bubble) = result {
            return Ok(bubble);
        }

        Err(mongodb::error::Error::custom("Unable to locate bubble"))
    }

    // Returns current bubble from user_id
    pub async fn save_bubble(&self, bubble: &Bubble) -> mongodb::error::Result<()> {
        let _ = self
            .bubble_collection
            .replace_one(doc! {"_id": bubble._id}, bubble, None)
            .await?;

        Ok(())
    }

    /// Insert new saved article into DB with given user_id
    pub async fn insert_saved(
        &self,
        user_id: &str,
        article: &Article,
    ) -> mongodb::error::Result<()> {
        let _result = self
            .saved_collection
            .insert_one(Saved::new(user_id, article), None)
            .await?;

        let result = self
            .bubble_collection
            .find_one(doc! {"user_id": user_id}, None)
            .await?;

        if let Some(mut bubble) = result {
            update_rating(&mut bubble, article, "save");

            // Delete from feed
            let mut feed = bubble.current_feed.unwrap().clone();
            for (index, compare_article) in feed.data.iter().enumerate() {
                if compare_article.author == article.author
                    && compare_article.title == article.title
                    && compare_article.description == article.description
                {
                    feed.data.remove(index);
                    break;
                }
            }

            bubble.current_feed = Some(feed);

            let _ = self
                .bubble_collection
                .replace_one(doc! {"user_id": user_id}, bubble.clone(), None)
                .await?;
        }

        Ok(())
    }

    /// Insert new saved article into DB with given user_id
    pub async fn get_saved(&self, user_id: &str) -> mongodb::error::Result<Vec<Saved>> {
        let mut saved = Vec::new();
        let mut results = self
            .saved_collection
            .find(doc! {"user_id": user_id}, None)
            .await?;

        while let Some(saved_entry) = results.try_next().await? {
            saved.push(saved_entry);
        }

        Ok(saved)
    }

    /// Delete saved article in DB
    pub async fn delete_saved(&self, saved: &Saved) -> mongodb::error::Result<()> {
        let _result = self
            .saved_collection
            .delete_one(doc! {"_id": saved._id}, None)
            .await?;

        Ok(())
    }
}

fn update_rating(bubble: &mut Bubble, article: &Article, action: &str) {
    match action {
        "like" => {
            // Sources
            if let Some(source_value) = bubble.sources.get(&article.source) {
                bubble.sources.insert(
                    article.source.clone(),
                    min(source_value + (1 * LIKE_MULTI), RATING_MAX),
                );
            } else {
                bubble
                    .sources
                    .insert(article.source.clone(), 1 * LIKE_MULTI);
            }

            // Categories
            if let Some(cat_value) = bubble.categories.get(&article.category) {
                bubble.categories.insert(
                    article.category.clone(),
                    min(cat_value + (1 * LIKE_MULTI), RATING_MAX),
                );
            } else {
                bubble
                    .categories
                    .insert(article.category.clone(), 1 * LIKE_MULTI);
            }

            // Languages
            if let Some(lang_value) = bubble.languages.get(&article.language) {
                bubble.languages.insert(
                    article.language.clone(),
                    min(lang_value + (1 * LIKE_MULTI), RATING_MAX),
                );
            } else {
                bubble
                    .languages
                    .insert(article.language.clone(), 1 * LIKE_MULTI);
            }
        }
        "dislike" => {
            // Sources
            if let Some(source_value) = bubble.sources.get(&article.source) {
                bubble.sources.insert(
                    article.source.clone(),
                    max(source_value + (1 * DISLIKE_MULTI), RATING_MIN),
                );
            } else {
                bubble
                    .sources
                    .insert(article.source.clone(), 1 * DISLIKE_MULTI);
            }

            // Categories
            if let Some(cat_value) = bubble.categories.get(&article.category) {
                bubble.categories.insert(
                    article.category.clone(),
                    max(cat_value + (1 * DISLIKE_MULTI), RATING_MIN),
                );
            } else {
                bubble
                    .categories
                    .insert(article.category.clone(), 1 * DISLIKE_MULTI);
            }

            // Languages
            if let Some(lang_value) = bubble.languages.get(&article.language) {
                bubble.languages.insert(
                    article.language.clone(),
                    max(lang_value + (1 * DISLIKE_MULTI), RATING_MIN),
                );
            } else {
                bubble
                    .languages
                    .insert(article.language.clone(), 1 * DISLIKE_MULTI);
            }

            // Update other fields(besides language) if lower than cutoff
            // This is so categories are fitlered through every now and again to give variety
            bubble.sources.iter_mut().for_each(|(_key, value)| {
                if *value <= -10 {
                    *value += 1;
                }
            });

            bubble.categories.iter_mut().for_each(|(_key, value)| {
                if *value <= -10 {
                    *value += 1;
                }
            });
        }
        "save" => {
            // Sources
            if let Some(source_value) = bubble.sources.get(&article.source) {
                bubble.sources.insert(
                    article.source.clone(),
                    min(source_value + (1 * SAVE_MULTI), RATING_MAX),
                );
            } else {
                bubble
                    .sources
                    .insert(article.source.clone(), 1 * SAVE_MULTI);
            }

            // Categories
            if let Some(cat_value) = bubble.categories.get(&article.category) {
                bubble.categories.insert(
                    article.category.clone(),
                    min(cat_value + (1 * SAVE_MULTI), RATING_MAX),
                );
            } else {
                bubble
                    .categories
                    .insert(article.category.clone(), 1 * SAVE_MULTI);
            }

            // Languages
            if let Some(lang_value) = bubble.languages.get(&article.language) {
                bubble.languages.insert(
                    article.language.clone(),
                    min(lang_value + (1 * SAVE_MULTI), RATING_MAX),
                );
            } else {
                bubble
                    .languages
                    .insert(article.language.clone(), 1 * SAVE_MULTI);
            }
        }
        _ => {}
    }
}
