use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::{JoinResult, PlayResult};
use std::vec::Vec;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_all_logs() -> Vec<JoinResult> {
    use self::schema::logs::dsl::*;
    use self::schema::songs::dsl::*;

    let db = establish_connection();

    convert_to_join_result(
        logs.inner_join(songs)
            .order(date.desc())
            .load::<(Log, Song)>(&db)
            .expect("Error loading logs"),
    )
}

pub fn get_all_songs() -> Vec<Song> {
    use self::schema::songs::dsl::*;

    let db = establish_connection();

    songs.load::<Song>(&db).expect("Error loading songs")
}

pub fn get_current() -> Option<JoinResult> {
    use self::schema::logs::dsl::*;
    use self::schema::songs::dsl::*;

    let db = establish_connection();

    convert_to_join_result(
        logs.inner_join(songs)
            .limit(1)
            .order(date.desc())
            .load::<(Log, Song)>(&db)
            .expect("Error loading logs"),
    )
    .pop()
}

pub fn get_all_plays() -> Vec<PlayResult> {
    let mut results = Vec::new();

    for song in get_all_songs().iter() {
        results.push(PlayResult {
            song: song.clone(),
            plays: get_plays(song.id),
        });
    }
    results.sort_by(|a, b| b.plays.cmp(&a.plays));
    results
}

pub fn get_plays(song_id: i32) -> i64 {
    use self::schema::logs::dsl::*;
    use diesel::dsl::count_star;

    let db = establish_connection();

    logs.select(count_star())
        .filter(song.eq(song_id))
        .first::<i64>(&db)
        .unwrap()
}

fn convert_to_join_result(vec: Vec<(Log, Song)>) -> Vec<JoinResult> {
    let mut results = Vec::new();
    for (log, song) in vec.iter() {
        results.push(JoinResult {
            date: log.date,
            song: song.clone(),
            is_new: log.is_new,
        });
    }
    results
}
