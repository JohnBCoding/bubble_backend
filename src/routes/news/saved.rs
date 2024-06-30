use crate::prelude::*;

#[get("/saved/{user_id}")]
pub async fn get_saved(db: Data<Mongo>, session: Session, path: Path<String>) -> HttpResponse {
    let result = check_token(&session).await;

    let user_id = path.into_inner();
    if let Ok(_) = result {
        if let Ok(saved) = db.get_saved(&user_id).await {
            return HttpResponse::Ok().json(saved);
        } else {
            return HttpResponse::BadRequest().finish();
        }
    }

    HttpResponse::Unauthorized().finish()
}
