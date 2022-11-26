use std::{
    any::type_name,
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

pub const TIMEOUT_SECS: Duration = Duration::from_secs(60 * 60);

pub type SharedCache = Arc<RwLock<CacheStore>>;

#[derive(Default, Debug)]
pub struct CacheStore {
    db: HashMap<String, Vec<u8>>,
}

pub fn cache_get<T>(cache: &SharedCache, key: &str) -> Option<T>
where
    T: bincode::Decode,
{
    let new_key = format!("{}__{}", type_name::<T>(), key);
    cache.read().unwrap().db.get(&new_key).map(|i| {
        bincode::decode_from_slice(i, bincode::config::standard())
            .unwrap()
            .0
    })
}

pub fn cache_set<T>(cache: SharedCache, key: &str, input: T)
where
    T: bincode::Encode,
{
    let new_key = format!("{}__{}", type_name::<T>(), key);
    let encoded = bincode::encode_to_vec(input, bincode::config::standard()).unwrap();
    cache.write().unwrap().db.insert(new_key, encoded);
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
