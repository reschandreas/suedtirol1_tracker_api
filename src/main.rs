#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

use actix_web::{web, App, HttpServer, HttpRequest, Responder};

mod db;
mod models;

async fn all() -> impl Responder {
    web::Json(self::db::get_all())
}

async fn current() -> impl Responder {
    web::Json(self::db::get_current())
}

async fn plays(req: HttpRequest) -> impl Responder {
    let song_id: i32 = req.match_info().query("id").parse().unwrap();
    if song_id > 0 {
        return web::Json(self::db::get_plays(song_id));
    } else {
        return web::Json(0);
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/v1/all", web::get().to(all))
            .route("/v1/current", web::get().to(current))
            .route("/v1/plays/{id}", web::get().to(plays))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
