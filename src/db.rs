use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::JoinResult;
use std::vec::Vec;

pub mod models;
pub mod schema;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_all() -> Vec<JoinResult> {
    use self::schema::logs::dsl::*;
    use self::schema::songs::dsl::*;

    let conn = establish_connection();

    convert_to_join_result(
        logs.inner_join(songs)
            .order(date.desc())
            .load::<(Log, Song)>(&conn)
            .expect("Error loading logs"),
    )
}

pub fn get_current() -> Option<JoinResult> {
    use self::schema::logs::dsl::*;
    use self::schema::songs::dsl::*;

    let conn = establish_connection();

    convert_to_join_result(
        logs.inner_join(songs)
            .limit(1)
            .order(date.desc())
            .load::<(Log, Song)>(&conn)
            .expect("Error loading logs"),
    )
    .pop()
}

pub fn get_plays(song_id: i32) -> i64 {
    use self::schema::logs::dsl::*;
    use diesel::dsl::count_star;
    
    let conn = establish_connection();

    logs.select(count_star())
        .filter(song.eq(song_id))
        .first::<i64>(&conn)
        .unwrap()
}

fn convert_to_join_result(vec: Vec<(Log, Song)>) -> Vec<JoinResult> {
    let mut results: Vec<JoinResult> = Vec::new();
    for (log, song) in vec.iter() {
        results.push(JoinResult {
            date: log.date,
            song: song.clone(),
            is_new: log.is_new,
        });
    }
    results
}
