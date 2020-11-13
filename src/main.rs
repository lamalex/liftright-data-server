use clap::{crate_version, value_t, App, Arg};
use std::env;
use color_eyre::eyre;

#[cfg(feature = "server_warp")]
#[tokio::main]
pub async fn main() -> Result<(), eyre::Error> {
    start().await
}

#[cfg(feature = "server_actix")]
#[actix_rt::main]
pub async fn main() -> Result<(), eyre::Error> {
    start().await?
}

async fn start() -> Result<(), eyre::Error> {
    color_eyre::install()?;

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=liftright_data_server=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "liftright_data_server=info");
    }
    pretty_env_logger::init();

    const DEFAULT_PORT: &str = "3030";
    const ABOUT: &str = "Data collection server for LiftRight";

    let opts = App::new("liftright data server")
        .version(crate_version!())
        .author("Alex L. Launi <alaun001@odu.edu>")
        .about(ABOUT)
        .arg(
            Arg::with_name("port")
                .help("Port to listen on")
                .short("p")
                .long("port")
                .default_value(DEFAULT_PORT)
                .value_name("PORT"),
        )
        .get_matches();

    let port = value_t!(opts, "port", u32).unwrap_or_else(|e| e.exit());
    webserver::run(port).await.map_err(eyre::Error::new)
}

#[cfg(feature = "server_warp")]
mod webserver {
    
    use liftright_data_server::LrdsError;

    use std::net::SocketAddrV4;
    use warp::Filter;
    use super::filters;
    
    pub async fn run(port: u32) -> Result<(), LrdsError> {
        let db = liftright_data_server::establish_db_connection().await?;
        let api = filters::rest_api(db).with(warp::log("liftright_data_server"));

        let addr: SocketAddrV4 = format!("0.0.0.0:{}", port)
            .parse()
            .expect("Could not create IP.");

        warp::serve(api).run(addr).await;
        Ok(())
    }
}

#[cfg(feature = "server_warp")]
mod filters {
    use super::handlers;
    use uuid::Uuid;
    use warp::Filter;

    use liftright_data_server::json_api::{
        AddImuDataPayload, AddRepetitionPayload, AddSurveyPayload,
    };

    pub fn rest_api(
        db: mongodb::Collection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        repetitions_add(db.clone())
            .or(rtfb_status(db.clone()))
            .or(survey_submit(db.clone()))
            .or(add_imu_records(db))
            .or(heartbeat())
    }

    fn heartbeat() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "heartbeat")
            .and(warp::get())
            .and_then(handlers::heartbeat)
    }

    fn rtfb_status(
        db: mongodb::Collection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "rtfb_status" / Uuid)
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::rtfb_status)
    }

    fn repetitions_add(
        db: mongodb::Collection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "add_repetition")
            .and(warp::put())
            .and(with_db(db))
            .and(json_deserialize::<AddRepetitionPayload>())
            .and_then(handlers::add_repetition)
    }

    fn add_imu_records(
        db: mongodb::Collection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "add_imu_records")
            .and(warp::put())
            .and(with_db(db))
            .and(json_deserialize::<AddImuDataPayload>())
            .and_then(handlers::add_imu_records)
    }

    fn survey_submit(
        db: mongodb::Collection,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("v1" / "submit_survey")
            .and(warp::put())
            .and(with_db(db))
            .and(json_deserialize::<AddSurveyPayload>())
            .and_then(handlers::submit_survey)
    }

    fn json_deserialize<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
    where
        T: serde::de::DeserializeOwned + Send,
    {
        warp::body::content_length_limit(1024 * 1024).and(warp::body::json())
    }

    fn with_db(
        collection: mongodb::Collection,
    ) -> impl Filter<Extract = (mongodb::Collection,), Error = warp::reject::Rejection> + Clone
    {
        warp::any().map(move || collection.clone()).and_then(
            |collection: mongodb::Collection| async move {
                if true {
                    Ok(collection)
                } else {
                    Err(warp::reject())
                }
            },
        )
    }
}

#[cfg(feature = "server_warp")]
mod handlers {
    use uuid::Uuid;
    use warp::http;

    use liftright_data_server::{
        json_api::{AddImuDataPayload, AddRepetitionPayload, AddSurveyPayload},
        user::User,
    };

    #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
    struct RtfbJsonReply {
        rtfb_status: bool,
    }

    #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
    struct RecordsUpdatedJsonReply {
        updated_count: i64,
    }

    pub async fn heartbeat() -> Result<impl warp::Reply, warp::Rejection> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("System time is before epoch!")
            .as_secs();
        Ok(warp::reply::with_status(
            format!("{}", now),
            http::StatusCode::OK,
        ))
    }

    pub async fn rtfb_status(
        device_id: Uuid,
        collection: mongodb::Collection,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let user = User::new(device_id);
        user.check_rtfb_status(collection)
            .await
            .map_err(warp::reject::custom)
            .map(|rtfb_status| warp::reply::json(&RtfbJsonReply { rtfb_status }))
    }

    pub async fn add_repetition(
        collection: mongodb::Collection,
        body: AddRepetitionPayload,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        body.set
            .add_repetition(collection, body.repetition)
            .await
            .map_err(warp::reject::custom)
            .map(|_| warp::reply())
    }

    pub async fn add_imu_records(
        collection: mongodb::Collection,
        imurecords: AddImuDataPayload,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        imurecords
            .data
            .insert(collection)
            .await
            .map_err(warp::reject::custom)
            .map(|_| warp::reply())
    }

    pub async fn submit_survey(
        collection: mongodb::Collection,
        survey: AddSurveyPayload,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        survey
            .insert(collection)
            .await
            .map_err(warp::reject::custom)
            .map(|_| warp::reply())
    }
}

#[cfg(feature = "server_actix")]
mod webserver {
    use actix_web::{App, HttpServer};
    use color_eyre::Result;

    pub async fn run(port: u32) -> Result<()> {
        let server = HttpServer::new(App::new)
            .listen(std::net::TcpListener::bind(format!("0.0.0.0:{}", port))?)?
            .run();
        
        Ok(server.await?)
    }
}