use crate::prelude::*;

#[post("/rate/{user_id}/{action}")]
pub async fn rate_article(
    db: Data<Mongo>,
    session: Session,
    data: Json<Article>,
    path: Path<(String, String)>,
) -> HttpResponse {
    let result = check_token(&session).await;

    let (user_id, action) = path.into_inner();

    if let Ok(_) = result {
        if let Ok(feed) = db.rate(&user_id, &data, &action).await {
            return HttpResponse::Ok().json(feed);
        } else {
            return HttpResponse::BadRequest().finish();
        }
    }

    HttpResponse::Unauthorized().finish()
}
