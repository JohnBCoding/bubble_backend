use crate::prelude::*;

#[post("/save")]
pub async fn save_article(db: Data<Mongo>, session: Session, save: Json<Saved>) -> HttpResponse {
    let result = check_token(&session).await;
    if let Ok(_) = result {
        if let Ok(_) = db.insert_saved(&save.user_id, &save.article).await {
            return HttpResponse::Ok().finish();
        } else {
            return HttpResponse::BadRequest().finish();
        }
    }

    HttpResponse::Unauthorized().finish()
}
