mod rejections;
mod responses;
mod routes;
mod store;

#[macro_use]
extern crate lazy_static;

use rejections::{NoContentProvided, NoValue};
use std::{collections::HashMap, error::Error};
use tracing::warn;
use warp::{http::StatusCode, reject::MethodNotAllowed, Filter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let index = warp::get().and(warp::path!()).map(|| responses::success());

    let create = warp::path!("create" / String)
        .and(warp::post())
        .and(warp::body::json::<HashMap<String, serde_json::Value>>())
        .and_then(routes::create);

    let fetch = warp::path!("fetch" / String)
        .and(warp::get())
        .and_then(routes::fetch);

    let delete = warp::path!("delete" / String)
        .and(warp::delete())
        .and_then(routes::delete);

    let routes = warp::any()
        .and(index.or(create).or(fetch).or(delete))
        .recover(handle_rejection);

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not set CTRL-C handler");
        warn!("Received Termination Signal...");
        std::process::exit(0)
    });

    warp::serve(routes).run(([127, 0, 0, 1], 80)).await;

    Ok(())
}

async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    let message;
    let code: StatusCode;

    if err.is_not_found() {
        message = "Could not find that route";
        code = StatusCode::NOT_FOUND
    } else if let Some(_) = err.find::<MethodNotAllowed>() {
        message = "METHOD_NOT_ALLOWED";
        code = StatusCode::METHOD_NOT_ALLOWED
    } else if let Some(_) = err.find::<NoContentProvided>() {
        message = "NO_CONTENT_PROVIDED";
        code = StatusCode::BAD_REQUEST
    } else if let Some(_) = err.find::<NoValue>() {
        message = "NO_VALUE";
        code = StatusCode::BAD_REQUEST
    } else {
        eprintln!("Unhandled rejection: {:?}", err);

        message = "INTERNAL_SERVER_ERROR";
        code = StatusCode::INTERNAL_SERVER_ERROR
    }

    Ok(responses::custom(message.to_string(), code))
}
