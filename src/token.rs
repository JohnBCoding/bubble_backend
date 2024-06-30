use crate::prelude::*;

pub async fn check_token(session: &Session) -> Result<(), String> {
    let firebase_key =
        env::var("FIREBASE_KEY").expect("Unable to get [FIREBASE_KEY] from environment.");

    // Request client
    let client = reqwest::Client::new();

    // Check if token in cookie session is valid
    let valid = {
        if let Ok(result) = session.get::<String>("id_token") {
            if let Some(id) = result {
                // Check user data endpoint with token to validate
                let uri = format!(
                    "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
                    firebase_key
                );
                let result = client.post(uri).json(&id).send().await;
                if let Ok(res) = result {
                    if res.status() == 200 {
                        true
                    } else {
                        println!("line 24");
                        false
                    }
                } else {
                    println!("line 27");
                    false
                }
            } else {
                println!("line 32");
                false
            }
        } else {
            println!("line 36");
            false
        }
    };

    // If token has expired, check for refersh token and use it.
    if !valid {
        if let Ok(result) = session.get::<String>("refresh_token") {
            if let Some(id) = result {
                let refresh_payload = RefreshPayload {
                    grant_type: "refresh_token".to_string(),
                    refresh_token: id,
                };
                let uri = format!(
                    "https://securetoken.googleapis.com/v1/token?key={}",
                    firebase_key
                );
                let result = client.post(uri).json(&refresh_payload).send().await;
                if let Ok(res) = result {
                    if res.status() == 200 {
                        let refresh_res = res.json::<RefreshResponse>().await.unwrap();
                        let _ = session.insert("id_token", refresh_res.id_token);
                        let _ = session.insert("refresh_token", refresh_res.refresh_token);
                    } else {
                        println!("line 60");
                        return Err("User unable to be authenticated.".to_string());
                    }
                } else {
                    println!("line 64");
                    return Err("User unable to be authenticated.".to_string());
                }
            } else {
                println!("line 67");
                return Err("User unable to be authenticated.".to_string());
            }
        } else {
            println!("line 72");
            return Err("User unable to be authenticated.".to_string());
        }
    }

    Ok(())
}
