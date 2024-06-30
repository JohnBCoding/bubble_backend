use std::str::FromStr;

use crate::prelude::*;

const TIMER_MAX: i64 = 20;

#[get("/feed/{user_id}")]
pub async fn get_feed(db: Data<Mongo>, session: Session, path: Path<String>) -> HttpResponse {
    let result = check_token(&session).await;
    let user_id = path.into_inner();
    let bubble_result = db.get_bubble(&user_id).await;
    if let Ok(mut bubble) = bubble_result {
        if let Some(refresh_time) = bubble.last_refresh {
            println!("{}", refresh_time);
            let old_time = NaiveTime::from_str(&refresh_time).unwrap();
            let diff = Local::now().time() - old_time;
            if diff.num_minutes() <= TIMER_MAX && diff.num_minutes() >= 0 {
                println!(
                    "not enough time has passed, time passed: {}",
                    diff.num_minutes()
                );
                let feed_result = db.get_current_feed(&user_id).await;
                if let Ok(mut feed) = feed_result {
                    feed.refresh_cooldown = Some((TIMER_MAX - diff.num_minutes()) as i32);
                    return HttpResponse::Ok().json(&feed);
                }
            } else {
                bubble.last_refresh = Some(Local::now().time().to_string());
                let _ = db.save_bubble(&bubble).await;
            }
        } else {
            bubble.last_refresh = Some(Local::now().time().to_string());
            let _ = db.save_bubble(&bubble).await;
        }

        if let Ok(_) = result {
            let media_key =
                env::var("MEDIA_KEY").expect("Unable to get [MEDIA_KEY] from environment.");

            // Request client
            let client = reqwest::Client::new();

            // Check ratings for anything that should be removed from the query
            let mut sources = "&sources=".to_string();
            let mut categories = "&categories=".to_string();
            let mut languages = "&languages=".to_string();

            let mut added = false;
            bubble.sources.iter().for_each(|(key, value)| {
                if *value <= -10 {
                    if !added {
                        sources = format!("{}-{}", sources, key);
                        added = true;
                    } else {
                        sources = format!("{}, -{}", sources, key);
                    }
                }
            });

            println!("{}", sources);

            added = false;
            bubble.categories.iter().for_each(|(key, value)| {
                if *value <= -10 {
                    if !added {
                        categories = format!("{}-{}", categories, key);
                        added = true;
                    } else {
                        categories = format!("{}, -{}", categories, key);
                    }
                }
            });

            println!("{}", categories);

            added = false;
            bubble.languages.iter().for_each(|(key, value)| {
                if *value <= -10 {
                    if !added {
                        languages = format!("{}-{}", languages, key);
                        added = true;
                    } else {
                        languages = format!("{}, -{}", languages, key);
                    }
                }
            });

            println!("{}", languages);

            let uri = format!(
                "http://api.mediastack.com/v1/news?access_key={}{}{}{}&limit=100",
                media_key, sources, categories, languages
            );

            let result = client.get(uri).send().await;
            if let Ok(res) = result {
                if let Ok(feed) = res.json::<Feed>().await {
                    let _ = db.save_feed(&user_id, &feed).await;
                    return HttpResponse::Ok().json(&feed);
                } else {
                    let feed_result = db.get_current_feed(&user_id).await;
                    if let Ok(feed) = feed_result {
                        return HttpResponse::Ok().json(&feed);
                    }
                }
            }
        } else {
            println!("{:?}", result.err());
        }
    }

    HttpResponse::Unauthorized().finish()
}
