use std::net::SocketAddrV4;
use warp::{http, Filter};

use liftright_data_server::repetition::{Repetition, NewRepetition};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let addr: SocketAddrV4 = "127.0.0.1:3030".parse()
        .expect("Could not create IP.");

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("add_rep"))
        .and(warp::path::end())
        .and(json_body())
        .and_then(add_repetition);

    let server = warp::serve(add_items);
    server.run(addr).await
}

fn json_body() -> impl Filter<Extract = (NewRepetition,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn add_repetition(
    rep: NewRepetition
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let conn = liftright_data_server::establish_connection();
        Repetition::create(&conn, rep).unwrap();

        Ok(warp::reply::with_status(
            "good job",
            http::StatusCode::CREATED,
        ))
}