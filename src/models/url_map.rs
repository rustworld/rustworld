use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use axum::http::StatusCode;
use axum::Json;


#[derive(Clone)]
pub struct UrlMap {
    pub urls: Arc<RwLock<HashMap<u32, String>>>,
}


impl UrlMap {
    pub fn new() -> Self {
        Self { urls: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub fn count(&self) -> usize {
        self.urls.read().unwrap().len()
    }

    pub fn create(&self, create_item: CreateItem) -> StatusCode {
        let mut urls = self.urls.write().unwrap();

        match urls.get(&create_item.key) {
            Some(_) => StatusCode::CONFLICT,
            None => {
                urls.insert(create_item.key, create_item.value);
                StatusCode::CREATED                
            }
        }
    }

    pub fn read(&self, key: u32) -> Result<Json<ReadItem>, StatusCode> {
        let urls = self.urls.read().unwrap();
        let value = urls.get(&key);

        match value {
            Some(val) => Ok(Json(ReadItem {
                key: key,
                value: val.clone(),
            })),
            None => Err(StatusCode::NOT_FOUND)
        }
    }
    
    pub fn update(&self, key: u32, update_item: UpdateItem) -> StatusCode {
        let mut urls = self.urls.write().unwrap();

        match urls.get_mut(&key) {
            Some(val) => {
                *val = update_item.value;
                StatusCode::OK
            }
            None => StatusCode::NOT_FOUND
        }
    }

    pub fn delete(&self, key: u32) -> StatusCode {
        let mut urls = self.urls.write().unwrap();

        match urls.get(&key) {
            Some(_) => {
                urls.remove(&key);
                StatusCode::OK
            }
            None => StatusCode::NOT_FOUND
        }
    }

    pub fn all(&self) -> Json<HashMap<u32, String>> {
        let urls = self.urls.write().unwrap();
        Json(urls.clone())
    }
}


#[derive(Debug, Deserialize)]
pub struct CreateItem {
    key: u32,
    value: String,
}

#[derive(Debug, Serialize)]
pub struct ReadItem {
    key: u32,
    value: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItem {
    value: String
}