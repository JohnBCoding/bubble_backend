mod db;
mod models;
mod routes;
mod startup;
mod token;

pub mod prelude {
    pub use crate::db::*;
    pub use crate::models::*;
    pub use crate::routes::*;
    pub use crate::startup::*;
    pub use crate::token::*;
    pub use actix_cors::Cors;
    pub use actix_session::{
        config::CookieContentSecurity, storage::CookieSessionStore, Session, SessionMiddleware,
    };
    pub use actix_web::dev::Server;
    pub use actix_web::{
        cookie::{Key, SameSite},
        get,
        middleware::Logger,
        post, web,
        web::Data,
        web::Json,
        web::Path,
        App, HttpResponse, HttpServer, Responder,
    };
    pub use chrono::prelude::*;
    pub use futures::TryStreamExt;
    pub use mongodb::bson::oid::ObjectId;
    pub use mongodb::bson::{doc, from_document, to_bson};
    pub use mongodb::options::ClientOptions;
    pub use mongodb::{Client, Collection};
    pub use reqwest;
    pub use serde::{Deserialize, Serialize};
    pub use serde_with::{serde_as, DefaultOnNull};
    pub use std::cmp::{max, min};
    pub use std::collections::HashMap;
    pub use std::env;
    pub use std::net::TcpListener;
}
