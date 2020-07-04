use warp::{http, Filter};
use serde::{Serialize, Deserialize};
use pretty_env_logger;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Repetition {
    device_id: String,
    session_id: String,
    set_id: String,
    rom: f64,
    velocity: f64,
    duration: f64,
    time: String
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("add_rep"))
        .and(warp::path::end())
        .and(json_body())
        .and_then(add_grocery_list_item);

    warp::serve(add_items)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn json_body() -> impl Filter<Extract = (Repetition,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn add_grocery_list_item(
    rep: Repetition
    ) -> Result<impl warp::Reply, warp::Rejection> {
        println!("{:?}", rep);

        Ok(warp::reply::with_status(
            "Added repetition",
            http::StatusCode::CREATED,
        ))
}