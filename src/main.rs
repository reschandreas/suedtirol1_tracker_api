#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate itertools;
extern crate serde;
extern crate serde_json;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

mod db;
mod models;

async fn all_logs() -> impl Responder {
    web::Json(self::db::get_all_logs())
}

async fn all_songs() -> impl Responder {
    web::Json(self::db::get_all_songs())
}

async fn current() -> impl Responder {
    web::Json(self::db::get_current())
}

async fn all_plays() -> impl Responder {
    web::Json(self::db::get_all_plays())
}

async fn plays(req: HttpRequest) -> impl Responder {
    let song_id: i32 = req.match_info().query("id").parse().unwrap_or(0);
    web::Json(self::db::get_plays(song_id))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/v1/all", web::get().to(all_logs))
            .route("/v1/current", web::get().to(current))
            .route("/v1/plays", web::get().to(all_plays))
            .route("/v1/plays/{id}", web::get().to(plays))
            .route("/v1/songs", web::get().to(all_songs))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
