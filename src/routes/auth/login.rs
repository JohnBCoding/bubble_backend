use crate::prelude::*;

#[post("/login")]
pub async fn login_user(session: Session, user: Json<UserLogin>) -> HttpResponse {
    let firebase_key =
        env::var("FIREBASE_KEY").expect("Unable to get [FIREBASE_KEY] from environment.");

    let client = reqwest::Client::new();
    let uri = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
        firebase_key
    );
    let result = client.post(uri).json(&user).send().await;

    match result {
        Ok(res) => {
            if res.status() == 200 {
                let login_result = res.json::<LoginResponse>().await;
                if let Ok(login) = login_result {
                    // Set cookie tokens
                    let _result = session.insert("id_token", login.token.clone());
                    let _result = session.insert("refresh_token", login.refresh_token.clone());

                    // Return user information
                    let user_res = UserResponse {
                        user_id: login.id.clone(),
                    };

                    return HttpResponse::Ok().json(user_res);
                }
            }
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

    HttpResponse::BadRequest().finish()
}
