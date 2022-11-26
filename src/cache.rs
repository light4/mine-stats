use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{body::Bytes, extract::State, response::IntoResponse, Json};
use serde_json::json;

pub type SharedCache = Arc<RwLock<CacheStore>>;

#[derive(Default, Debug)]
pub struct CacheStore {
    db: HashMap<String, Bytes>,
}

pub fn cache_get(cache: &SharedCache, key: &str) -> Option<Bytes> {
    cache.read().unwrap().db.get(key).cloned()
}

pub fn cache_set(cache: SharedCache, key: String, bytes: Bytes) {
    cache.write().unwrap().db.insert(key, bytes);
}

pub fn list_keys(cache: &SharedCache) -> Vec<String> {
    let db = &cache.read().unwrap().db;

    db.keys()
        .map(|key| key.to_string())
        .collect::<Vec<String>>()
}

pub async fn list_keys_api(State(cache): State<SharedCache>) -> impl IntoResponse {
    let keys = list_keys(&cache);
    Json(json!({
        "items": keys,
        "count": keys.len(),
    }))
}
