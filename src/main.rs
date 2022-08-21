mod config;
mod server;

#[macro_use]
extern crate lazy_static;

use ansi_term::Color::Blue;
use neofiglet::FIGfont;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use server::rejections::{NoContentProvided, NoValue};
use server::responses;
use server::routes;
use std::error::Error;
use std::net::SocketAddr;
use tracing::{info, warn};
use warp::{http::StatusCode, reject::MethodNotAllowed, Filter};

use crate::config::read;

#[derive(Serialize, Deserialize)]
pub struct Body {
    content: Value,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let figlet = FIGfont::standard()?;
    println!("{}", figlet.convert("VioleT").unwrap());
    println!("{}", "Volatile In-Memory Database");

    println!();
    println!("VERSION: {}", Blue.paint(env!("CARGO_PKG_VERSION")));
    println!();

    info!("Starting...");

    let config = read("./config.yml")?;
    let addr = format!("{}:{}", config.host, config.port).parse::<SocketAddr>()?;

    let create = warp::path!("add" / String)
        .and(warp::post())
        .and(warp::body::json::<Body>())
        .and_then(routes::create);

    let read = warp::path!("get" / String)
        .and(warp::get())
        .and_then(routes::fetch);

    let update = warp::path!("up" / String)
        .and(warp::patch())
        .and(warp::body::json::<Body>())
        .and_then(routes::update);

    let delete = warp::path!("del" / String)
        .and(warp::delete())
        .and_then(routes::delete);

    let routes = warp::any()
        .and(create.or(read).or(update).or(delete))
        .recover(handle_rejection);

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not set CTRL-C handler");
        warn!("Received Termination Signal...");
        std::process::exit(0)
    });

    warp::serve(routes).run(addr).await;

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
        code = StatusCode::NO_CONTENT
    } else {
        eprintln!("Unhandled rejection: {:?}", err);

        message = "INTERNAL_SERVER_ERROR";
        code = StatusCode::INTERNAL_SERVER_ERROR
    }

    Ok(responses::custom(message.to_string(), code))
}
