use crate::prelude::*;

pub async fn start_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let port = listener.local_addr().unwrap().port();
    let db = Mongo::new().await;
    let env_key = env::var("SESSION_KEY").expect("Unable to get [SESSION_KEY] from environment.");
    let secret_key = Key::from(env_key.as_bytes());
    let server = HttpServer::new(move || {
        let store = CookieSessionStore::default();
        let session = SessionMiddleware::builder(store, secret_key.clone())
            .cookie_http_only(true)
            .cookie_same_site(SameSite::None)
            .cookie_secure(true)
            .cookie_content_security(CookieContentSecurity::Private)
            .build();
        let cors = Cors::default()
            .supports_credentials()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(session)
            .wrap(Logger::default())
            .app_data(web::Data::new(db.clone()))
            .service(
                web::scope("/auth")
                    //.service(register_user)
                    .service(login_user),
            )
            .service(
                web::scope("news")
                    .service(get_feed)
                    .service(save_article)
                    .service(get_saved)
                    .service(delete_saved)
                    .service(rate_article),
            )
    })
    .listen(listener)?
    .run();

    println!("\nServer is up on port {}...", port);

    Ok(server)
}
