use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Queryable)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Queryable)]
pub struct Log {
    pub date: chrono::NaiveDateTime,
    pub song: i32,
    pub is_new: bool,
}
