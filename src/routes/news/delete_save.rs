use crate::prelude::*;

#[post("/delete_saved")]
pub async fn delete_saved(db: Data<Mongo>, session: Session, data: Json<Saved>) -> HttpResponse {
    let result = check_token(&session).await;

    if let Ok(_) = result {
        if let Ok(_) = db.delete_saved(&data).await {
            return HttpResponse::Ok().finish();
        } else {
            return HttpResponse::BadRequest().finish();
        }
    }

    HttpResponse::Unauthorized().finish()
}
