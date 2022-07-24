use crate::rejections::NoContentProvided;
use crate::rejections::NoValue;
use crate::responses;
use crate::store;
use crate::Body;
use std::sync::Mutex;
use warp::{http::StatusCode, reject};

lazy_static! {
    static ref STORE: Mutex<store::StoreRegistry> = Mutex::new(store::StoreRegistry::new());
}

pub async fn create(key: String, payload: Body) -> Result<impl warp::Reply, warp::Rejection> {
    let content = payload.content.as_str();

    if content.unwrap().is_empty() {
        return Err(reject::custom(NoContentProvided));
    }

    let _ = &STORE.lock().unwrap().add(key, content.unwrap().to_string());

    Ok(responses::success())
}

pub async fn fetch(key: String) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(value) = STORE.lock().unwrap().get(key) {
        Ok(responses::custom(value, StatusCode::OK))
    } else {
        Err(reject::custom(NoValue))
    }
}

pub async fn delete(key: String) -> Result<impl warp::Reply, warp::Rejection> {
    if STORE.lock().unwrap().delete(key) {
        Ok(responses::custom("DELETED".to_string(), StatusCode::OK))
    } else {
        Err(reject::custom(NoValue))
    }
}

pub async fn update(key: String, payload: Body) -> Result<impl warp::Reply, warp::Rejection> {
    if STORE.lock().unwrap().delete(&key) {
        let content = payload.content.as_str();

        if content.unwrap().is_empty() {
            return Err(reject::custom(NoContentProvided));
        }

        let _ = &STORE.lock().unwrap().add(key, content.unwrap().to_string());

        Ok(responses::custom("UPDATED".to_string(), StatusCode::OK))
    } else {
        Err(reject::custom(NoValue))
    }
}
