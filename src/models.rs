use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JoinResult {
    pub date: chrono::NaiveDateTime,
    pub song: crate::db::models::Song,
    pub is_new: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayResult {
    pub song: crate::db::models::Song,
    pub plays: usize,
}
